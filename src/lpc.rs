//! A linear predictor fitted to one channel of a block, as FLAC's LPC
//! subframe carries it: coefficients quantized to a precision and a shift,
//! and the residual they leave.
//!
//! The samples are seen through a Tukey window, their autocorrelation taken
//! to the highest order, and Levinson-Durbin gives the predictor of every
//! order up to it with the error each leaves; each is quantized, the error
//! its rounding adds is taken from the autocorrelation, and the order
//! estimated cheapest, residual, rounding and coefficients together, is
//! kept as a candidate with its estimate and its quantized predictor.

/// The highest order fitted; libFLAC's default level stops here too.
pub const MAX_ORDER: usize = 8;
/// The highest fixed predictor order FLAC has.
pub const MAX_FIXED_ORDER: usize = 4;
/// The most a coefficient shift can be, FLAC's field holding 0 to 15.
const MAX_SHIFT: i32 = 15;
/// The width the samples are scaled to for the autocorrelation, the most a
/// scaled sample may be, and the samples a chunk of the sums, whose
/// products are summed in 32 bits: `CHUNK` products of `LIMIT` must fit.
const SCALED: u32 = 14;
const LIMIT: i16 = (1 << (SCALED - 1)) - 1;
const CHUNK: usize = 32;
const _: () = assert!(CHUNK as i64 * (LIMIT as i64 * LIMIT as i64) <= i32::MAX as i64);
/// A block whose fit at order 2 leaves less than this of its power has its
/// autocorrelation taken again in double precision: the pure tone, 1.1 to
/// 1.2 millionths on the made tone's blocks, whose fit the window's 15-bit
/// weights alone move by a tenth of its bytes either way. On the
/// recordings' blocks dumped the rule falls on a fifth of a per cent of the
/// Aria's, a tenth of the Raven's and none of the rest's; a rule on the
/// rounding noise against the fit's error, which took the loud, resonant
/// passage of the indie record from 0.11% of its bytes to 0.09%, sent a
/// sixth of the Aria's blocks to the exact sums and halved its gain, so
/// it is not applied.
const EXACT_GAIN: f64 = 2e-6;
/// The bits a sample the fit may be estimated to lose to the fixed predictor
/// and still be tried: the estimates' difference was measured to be within
/// this of the truth, no block's bytes changing between this and no margin.
pub const MARGIN: f64 = 1.0 / 16.0;

/// The quantized predictor of one block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Predictor {
    pub order: usize,
    /// The bits a coefficient is written in, 1 to 15.
    pub precision: u32,
    /// The bits the prediction's sum is shifted down by.
    pub shift: u32,
    /// The coefficients, the first `order` of them, and zero past it, which
    /// the residual's loops rely on where they run at a higher order.
    pub coefs: [i32; MAX_ORDER],
}

/// The window the samples are seen through before the fit: Tukey, a quarter
/// of the block tapered by a half cosine at each end, as libFLAC's default;
/// the tapers' weights in 15 bits for the scaled sums, the flat middle being
/// the samples as they are, and the whole window in double precision for
/// the exact ones.
pub struct Window {
    taper: Vec<i16>,
    exact: Vec<f64>,
}

impl Window {
    pub fn new(n: usize) -> Self {
        let taper = n / 4;
        let mut exact = vec![1.0f64; n];
        for i in 0..taper {
            let v = 0.5 - 0.5 * (std::f64::consts::PI * i as f64 / taper as f64).cos();
            exact[i] = v;
            exact[n - 1 - i] = v;
        }
        Self { taper: exact[..taper].iter().map(|&v| (v * 32768.0).round().min(32767.0) as i16).collect(), exact }
    }
}

/// Room for a channel's samples scaled and windowed, kept across blocks,
/// and whether its last block was a tone, summed exactly.
#[derive(Default)]
pub struct Windowed {
    scaled: Vec<i16>,
    exact: Vec<f64>,
    tone: bool,
}

/// The coefficient precision for a stream of `bits` a sample in blocks of
/// `block`: libFLAC's choice for its default level, the blocks it has data
/// for being 16-bit.
pub fn precision(bits: u32, block: usize) -> u32 {
    if bits < 16 {
        (2 + bits / 2).max(5)
    } else if bits == 16 {
        match block {
            ..=192 => 7,
            193..=384 => 8,
            385..=576 => 9,
            577..=1152 => 10,
            1153..=2304 => 11,
            2305..=4608 => 12,
            _ => 13,
        }
    } else {
        (8 + bits / 2).min(13)
    }
}

/// What a channel's autocorrelation says its coding will cost, in bits
/// estimated from the squared error a predictor leaves, as libFLAC estimates
/// them: the estimates share whatever a Rice code adds to the error's
/// logarithm, so their differences are fair guesses at the real ones, within
/// 0.04 bit a sample measured at 16 bits.
pub struct Estimate {
    /// The bits of the fixed predictor of `fixed_order`, residual and
    /// warm-up samples.
    pub fixed: f64,
    /// The fixed order estimated: the one given, or the one chosen here.
    pub fixed_order: usize,
    /// The fitted predictor estimated cheapest, where it is estimated to
    /// beat the fixed one or come within [`MARGIN`] of it; none where nothing
    /// of the signal is left to predict or the fixed predictor does as well.
    pub fit: Option<Candidate>,
}

/// A predictor fitted and quantized, with the bits it is estimated to
/// take, residual, warm-up samples and coefficients together.
pub struct Candidate {
    predictor: Predictor,
    bits: f64,
}

impl Candidate {
    /// The bits the fit is estimated to take, on the same scale as
    /// [`Estimate::fixed`].
    pub fn bits(&self) -> f64 {
        self.bits
    }

    /// The predictor as it is written.
    pub fn predictor(&self) -> Predictor {
        self.predictor
    }
}

/// A channel's autocorrelation through the window, at every lag to the
/// highest order.
pub type Autocorrelation = [f64; MAX_ORDER + 1];

/// The autocorrelation of `signal`, whose samples are `width` bits wide,
/// seen through `window`, `windowed` being room for the product: the
/// samples scaled to [`SCALED`] bits and summed as integers ([`scaled`]),
/// which loses bits only on a block wider than that, in the samples' own
/// proportion, and is exact in the sums; a block whose fit at order 2
/// leaves less than [`EXACT_GAIN`] of its power, the pure tone, is summed
/// again in double precision ([`exact`]), and the channel's next block
/// exactly first, the scaled sums taken up again on the block after one
/// that is no tone. Measured apart on the EPYC, 960 samples: the scaled
/// sums 0.49 µs against the exact 1.14, 0.12 of it the scaling and window;
/// the normalised autocorrelation within 2e-5 of the exact one on noise
/// and music and within 1e-7 on a resonant block. In the encoder every
/// stereo recording took 12% to 18% fewer instructions, white noise 3%,
/// and the made sounds ran 0.85 to 0.91 of their time; the bytes rose
/// 0.11% on the indie record, 0.012% on the Fantasy, 0.008% on the train
/// station and 0.004% on the Aria, and nowhere else.
pub fn autocorrelation(signal: &[i32], width: u32, window: &Window, windowed: &mut Windowed) -> Autocorrelation {
    /// Whether a fit at order 2 leaves less than `EXACT_GAIN` of the power,
    /// from the two reflection coefficients.
    fn tone(r: &Autocorrelation) -> bool {
        let e1 = r[0] - r[1] * r[1] / r[0];
        let e2 = if e1 > 0.0 {
            let k2 = (r[2] - r[1] * r[1] / r[0]) / e1;
            e1 * (1.0 - k2 * k2)
        } else {
            0.0
        };
        e2 < EXACT_GAIN * r[0]
    }
    if windowed.tone {
        let r = exact(signal, window, &mut windowed.exact);
        windowed.tone = tone(&r);
        return r;
    }
    let r = scaled(signal, width, window, &mut windowed.scaled);
    if tone(&r) {
        windowed.tone = true;
        exact(signal, window, &mut windowed.exact)
    } else {
        r
    }
}

/// The autocorrelation from the samples scaled to `SCALED` bits, the right
/// shift rounded, clamped to `LIMIT` and the tapers' products' high halves
/// taken in 16-bit lanes, then the lags summed as integers ([`lags`]) and
/// scaled back. The clamp to i16 holds nothing, the scaled sample fitting,
/// and lets the compiler pack the lanes in one instruction; the clamp to
/// `LIMIT` keeps a chunk's products within 32 bits, which the one sample
/// of `-LIMIT - 1` would not.
fn scaled(signal: &[i32], width: u32, window: &Window, windowed: &mut Vec<i16>) -> Autocorrelation {
    let n = signal.len();
    let padded = n.div_ceil(CHUNK) * CHUNK + MAX_ORDER;
    if windowed.len() != padded {
        windowed.clear();
        windowed.resize(padded, 0);
    }
    let shift = SCALED as i32 - width as i32;
    let (up, down) = (shift.max(0) as u32, (-shift).max(0) as u32);
    let half = (1i32 << down) >> 1;
    for (x, &s) in windowed[..n].iter_mut().zip(signal) {
        *x = ((((s << up) + half) >> down).clamp(-32768, 32767) as i16).clamp(-LIMIT, LIMIT);
    }
    let taper = window.taper.len();
    let (head, rest) = windowed[..n].split_at_mut(taper);
    let tail = &mut rest[n - 2 * taper..];
    for (x, &w) in head.iter_mut().zip(&window.taper) {
        *x = ((i32::from(*x << 1) * i32::from(w)) >> 16) as i16;
    }
    for (x, &w) in tail.iter_mut().zip(window.taper.iter().rev()) {
        *x = ((i32::from(*x << 1) * i32::from(w)) >> 16) as i16;
    }
    let back = 2f64.powi(-2 * shift);
    lags(windowed, n).map(|s| s as f64 * back)
}

/// The autocorrelation in double precision through the window, as every
/// block had it before the scaled sums: the predictor of a quiet, resonant
/// block is as sensitive to the sum's rounding as to its own quantization,
/// and single precision was measured to cost 0.5% to 1% of the bytes on
/// piano and voice.
fn exact(signal: &[i32], window: &Window, windowed: &mut Vec<f64>) -> Autocorrelation {
    windowed.clear();
    windowed.extend(signal.iter().zip(&window.exact).map(|(&s, &w)| f64::from(s) * w));
    lags_exact(windowed)
}

/// Fit a predictor of up to `max_order` to a channel of `n` samples of `bps`
/// bits from its autocorrelation, with coefficients of about `precision`
/// bits, and estimate its bits beside the fixed predictor's from the same
/// autocorrelation, of `fixed_order` where the samples have chosen one and
/// otherwise of the order the autocorrelation says is best. Each order's
/// predictor is quantized and weighed with the error its rounding adds
/// ([`rounding_error`]), which on a pure tone is the whole of the residual
/// and on sound is nothing; the fit is the order whose estimated bits are
/// fewest, the lowest on a tie.
pub fn estimate(autoc: &Autocorrelation, n: usize, max_order: usize, bps: u32, precision: u32, fixed_order: Option<usize>) -> Estimate {
    debug_assert!((1..=MAX_ORDER).contains(&max_order) && n > max_order);
    // The fixed order given, or the one the autocorrelation says leaves the
    // least error, where the samples' own order search has not run.
    let fixed_order = fixed_order.unwrap_or_else(|| (0..=MAX_FIXED_ORDER).min_by(|&a, &b| fixed_error(autoc, a).total_cmp(&fixed_error(autoc, b))).expect("five orders"));
    // The residual's bits from the error it leaves: a Rice code of a
    // Laplacian residual of mean magnitude m, which is the root of half the
    // mean square, costs about log2(1 + 2m) + 1 bits a sample, a bit at the
    // least where the residual is at the width's floor and log2(m) + 2 where
    // it is large; libFLAC's estimate, log2(m), is the same but for the two
    // bits and the floor, and the floor is what tells a quiet channel from a
    // constant one. Then each warm-up sample at the width of a sample, each
    // coefficient at that and its precision.
    let scale = 0.5 / n as f64;
    let bits_of = |err: f64, order: usize, overhead: f64| ((1.0 + 2.0 * (err * scale).max(0.0).sqrt()).log2() + 1.0) * (n - order) as f64 + order as f64 * overhead;
    let fixed = bits_of(fixed_error(autoc, fixed_order), fixed_order, f64::from(bps));

    let mut coefs = [[0f64; MAX_ORDER]; MAX_ORDER];
    let mut errors = [0f64; MAX_ORDER];
    let orders = levinson(autoc, max_order, &mut coefs, &mut errors);
    let overhead = f64::from(bps + precision);
    // Each order's bits before its rounding, and the order they are fewest
    // at, which is quantized first; the rounding only adds, so another
    // order is quantized only where its bits before rounding are still
    // under the best with it, which on sound, where the rounding is
    // nothing, is no other order, and on a tone, where the orders tie at
    // the floor, is each of them.
    let mut plain = [f64::INFINITY; MAX_ORDER];
    for (i, &err) in errors[..orders].iter().enumerate() {
        plain[i] = bits_of(err, i + 1, overhead);
    }
    let first = (0..orders).min_by(|&a, &b| plain[a].total_cmp(&plain[b]));
    let mut best: Option<Candidate> = None;
    let weigh = |i: usize, best: &mut Option<Candidate>| {
        let order = i + 1;
        if let Some((predictor, rounding)) = quantize(&coefs[i][..order], precision_for(bps, precision, order), autoc) {
            let bits = bits_of(errors[i] + rounding, order, overhead);
            if best.as_ref().is_none_or(|b| bits < b.bits) {
                *best = Some(Candidate { predictor, bits });
            }
        }
    };
    if let Some(first) = first {
        weigh(first, &mut best);
        for i in (0..orders).filter(|&i| i != first) {
            if best.as_ref().is_none_or(|b| plain[i] < b.bits) {
                weigh(i, &mut best);
            }
        }
    }
    let fit = best.filter(|b| b.bits < fixed + MARGIN * n as f64);
    Estimate { fixed, fixed_order, fit }
}

/// The precision a predictor of `order` is quantized to for a sample of
/// `bps` bits, from the stream's `precision`: for a sample of 17 bits or
/// fewer no finer than lets the prediction's sum stay in 32 bits.
fn precision_for(bps: u32, precision: u32, order: usize) -> u32 {
    if bps <= 17 { precision.min(32 - bps - order.ilog2()) } else { precision }
}

/// The shift that puts `cmax`, the largest coefficient's magnitude, just
/// under the top of `precision` bits, one kept for the sign; the field
/// holds no more than 15, and a negative shift is not allowed, so too
/// large a coefficient is scaled down instead. Zero where there is no
/// coefficient to scale.
fn shift_for(cmax: f64, precision: u32) -> i32 {
    if cmax <= 0.0 || !cmax.is_finite() {
        return 0;
    }
    (precision as i32 - 2 - exponent(cmax)).min(MAX_SHIFT)
}

/// The floor of the logarithm to base 2 of `x`, positive and finite, from
/// its exponent bits, a subnormal counting as the smallest normal: the
/// library's `log2` and `floor` are calls at the x86-64 baseline, and
/// every order's predictor is quantized on every fit.
fn exponent(x: f64) -> i32 {
    debug_assert!(x > 0.0 && x.is_finite());
    ((x.to_bits() >> 52) & 0x7FF) as i32 - 1023
}

/// Two to the power `e`, for `e` within a double's exponent, from the bits.
fn pow2(e: i32) -> f64 {
    debug_assert!((-1022..=1023).contains(&e));
    f64::from_bits(((1023 + e) as u64) << 52)
}

/// `x` rounded to the nearest whole, halves away from zero, as a
/// conversion that truncates, which is one instruction where `round` is a
/// call at the x86-64 baseline.
fn round(x: f64) -> i64 {
    (x + 0.5f64.copysign(x)) as i64
}

/// The squared error the fixed predictor of `order` leaves on the windowed
/// signal, from its autocorrelation: the predictor's coefficients are the
/// binomials with alternating signs, and the error is their own
/// autocorrelation against the signal's.
fn fixed_error(autoc: &[f64; MAX_ORDER + 1], order: usize) -> f64 {
    const TAPS: [[f64; 5]; 5] = [
        [1.0, 0.0, 0.0, 0.0, 0.0],
        [2.0, -1.0, 0.0, 0.0, 0.0],
        [6.0, -4.0, 1.0, 0.0, 0.0],
        [20.0, -15.0, 6.0, -1.0, 0.0],
        [70.0, -56.0, 28.0, -8.0, 1.0],
    ];
    let taps = &TAPS[order];
    taps[0] * autoc[0] + 2.0 * taps[1..].iter().zip(&autoc[1..]).map(|(&t, &r)| t * r).sum::<f64>()
}

/// The sum of `x[i] * x[i + lag]` over the `n` samples of `x` for every lag
/// to the highest order, `x` padded with zeros to whole chunks and
/// `MAX_ORDER` more so that every lag's pair is in bounds and nothing is
/// checked in the loop. Each chunk's products are summed in 32 bits, two
/// products to a `pmaddwd`, and carried to 64 bits once a chunk: 0.37 µs
/// at 960 samples where the exact sums take 1.0, 18 floating-point
/// operations on two samples on the x86-64 baseline's two ports. A chunk
/// of 64 or 128 the compiler unrolls into widening multiplies, twice the
/// time; a chunk of 16 carries twice as often, 0.05 µs more; the chunk's
/// length as a value rather than a constant, or the four lanes written out
/// and each carried on its own, come out scalar. Not inlined, so that the
/// sums have the registers to themselves.
#[inline(never)]
fn lags(x: &[i16], n: usize) -> [i64; MAX_ORDER + 1] {
    let mut acc = [0i64; MAX_ORDER + 1];
    for c in 0..n.div_ceil(CHUNK) {
        let w = &x[c * CHUNK..c * CHUNK + CHUNK + MAX_ORDER];
        let a = &w[..CHUNK];
        for (lag, acc) in acc.iter_mut().enumerate() {
            let b = &w[lag..lag + CHUNK];
            let mut s = 0i32;
            for i in 0..CHUNK {
                s += i32::from(a[i]) * i32::from(b[i]);
            }
            *acc += i64::from(s);
        }
    }
    acc
}

/// `a * b + c`, fused where the machine has the instruction: on aarch64,
/// where every core has it, and on x86-64 built with `fma`. At the x86-64
/// baseline a fused multiply-add is a library call, and the product and the
/// sum are two instructions; the two roundings differ from the one in the
/// last bit of the autocorrelation, under what any fit is decided by.
#[inline(always)]
fn mul_add(a: f64, b: f64, c: f64) -> f64 {
    if cfg!(any(target_arch = "aarch64", target_feature = "fma")) { a.mul_add(b, c) } else { a * b + c }
}

/// The lags in double precision, one pass over the samples two at a time,
/// each seen through a window of the two and the eight after them, so
/// every lag's pair is in bounds by the window's length and nothing is
/// checked in the loop; the nine lags' sums two to a register, a sample
/// broadcast against the pairs of samples after it. Not inlined, so that
/// the nine sums and their operands have the registers to themselves.
#[inline(never)]
fn lags_exact(x: &[f64]) -> Autocorrelation {
    const W: usize = 2;
    let n = x.len();
    let mut lanes = [[0f64; W]; MAX_ORDER + 1];
    let whole = n.saturating_sub(MAX_ORDER) / W * W;
    for w in x[..whole + MAX_ORDER.min(n)].windows(W + MAX_ORDER).step_by(W) {
        for (lag, lanes) in lanes.iter_mut().enumerate() {
            for j in 0..W {
                lanes[j] = mul_add(w[j], w[j + lag], lanes[j]);
            }
        }
    }
    let mut out = [0f64; MAX_ORDER + 1];
    for (lag, (o, lanes)) in out.iter_mut().zip(&lanes).enumerate() {
        let tail = x[whole.min(n.saturating_sub(lag))..n.saturating_sub(lag)].iter().zip(&x[(whole + lag).min(n)..]).map(|(&a, &b)| a * b).sum::<f64>();
        *o = lanes.iter().sum::<f64>() + tail;
    }
    out
}

/// Levinson-Durbin: the predictor of each order from 1 to `max_order` from
/// the autocorrelation, in `coefs[order - 1][..order]`, and the squared error
/// each leaves in `errors[order - 1]`. Returns the orders found, fewer than
/// asked where the error reaches zero or the recursion breaks down.
fn levinson(autoc: &[f64; MAX_ORDER + 1], max_order: usize, coefs: &mut [[f64; MAX_ORDER]; MAX_ORDER], errors: &mut [f64; MAX_ORDER]) -> usize {
    let mut lpc = [0f64; MAX_ORDER];
    let mut err = autoc[0];
    for i in 0..max_order {
        if err.is_nan() || err <= 0.0 {
            return i;
        }
        let mut r = -autoc[i + 1];
        for j in 0..i {
            r -= lpc[j] * autoc[i - j];
        }
        r /= err;
        if !r.is_finite() {
            return i;
        }
        for j in 0..i / 2 {
            let tmp = lpc[j];
            lpc[j] += r * lpc[i - 1 - j];
            lpc[i - 1 - j] += r * tmp;
        }
        if i % 2 == 1 {
            lpc[i / 2] += lpc[i / 2] * r;
        }
        lpc[i] = r;
        err *= 1.0 - r * r;
        for j in 0..=i {
            coefs[i][j] = -lpc[j];
        }
        errors[i] = err;
    }
    max_order
}

/// The coefficients quantized to `precision` bits with the shift that keeps
/// the largest of them in range, each rounded with the error of the ones
/// before it carried along, as libFLAC does, and the squared error the
/// rounding adds to the predictor's over the windowed signal, from `autoc`.
/// A coefficient at the precision's edge, as a low tone's first, two
/// cosines of its frequency and just under 2, is clamped there, and the
/// carry takes most of the clamp's error into the next coefficient: the
/// shift one less, tried for that case, changed no byte of any sound
/// measured and is not tried. None where the coefficients are all zero.
fn quantize(coefs: &[f64], precision: u32, autoc: &Autocorrelation) -> Option<(Predictor, f64)> {
    let cmax = coefs.iter().fold(0f64, |m, &c| m.max(c.abs()));
    if cmax <= 0.0 || !cmax.is_finite() {
        return None;
    }
    let shift = shift_for(cmax, precision);
    let scale = pow2(shift);
    let qmax = (1i64 << (precision - 1)) - 1;
    let qmin = -qmax - 1;
    let mut out = Predictor { order: coefs.len(), precision, shift: shift.max(0) as u32, coefs: [0; MAX_ORDER] };
    let mut error = 0f64;
    for (q, &c) in out.coefs.iter_mut().zip(coefs) {
        error += c * scale;
        let rounded = round(error).clamp(qmin, qmax);
        error -= rounded as f64;
        *q = rounded as i32;
    }
    let error = rounding_error(coefs, &out, autoc);
    Some((out, error))
}

/// The squared error `predictor`'s rounding adds to what `coefs`, the
/// exact fit, leave over the signal as the autocorrelation sees it, the
/// windowed block with zeros beyond it: the fit's error is least there
/// over the coefficients, so the quantized predictor's is the fit's and
/// the quadratic form of the coefficients' errors over the autocorrelation,
/// exactly, the floor of the prediction's shift aside. On the samples
/// themselves the fit's own error at the block's edges adds a cross term
/// of either sign. On sound the form is nothing beside the residual; on a
/// pure tone it is most of it, and what tells the orders apart.
fn rounding_error(coefs: &[f64], predictor: &Predictor, autoc: &Autocorrelation) -> f64 {
    let scale = pow2(-(predictor.shift as i32));
    let mut e = [0f64; MAX_ORDER];
    for ((e, &c), &q) in e.iter_mut().zip(coefs).zip(&predictor.coefs) {
        *e = c - f64::from(q) * scale;
    }
    let e = &e[..coefs.len()];
    let mut total = 0f64;
    for (i, &ei) in e.iter().enumerate() {
        total += ei * ei * autoc[0];
        for (j, &ej) in e[i + 1..].iter().enumerate() {
            total += 2.0 * ei * ej * autoc[j + 1];
        }
    }
    total.max(0.0)
}

/// The residual `predictor` leaves over `signal`, of `bps` bits a sample,
/// from its `order`th sample on, `halves` being room for the samples as 16
/// bits, or as their upper 16 bits and their lower bits. False where a
/// residual would not fit a 32-bit sample or would be its most negative
/// value, which FLAC forbids a residual to be, and which only a wide
/// sample's wild predictor can make.
///
/// The 16-bit loops are run at the orders the compiler lays out well, a
/// lower order over a copy led by zero samples, its taps past the order
/// zero: the compiler takes the taps two at a time into one multiply-add
/// of 16-bit lanes, an odd order's last tap through a 32-bit multiply
/// instead, and the split loop at orders 2, 3, 6 and 7 comes out partly or
/// wholly scalar. Measured apart on the EPYC over 960 samples, the 16-bit
/// loop at order 3 run as 4 is 0.66 of its time, 5 as 6 0.81 and 7 as 8
/// 0.89, where 2 as 4 is 1.7, and the split loop at 2 as 4 0.47, 3 as 4
/// 0.35, 6 as 8 0.75 and 7 as 8 0.56, where 5 as 8 is 1.16. In the
/// encoder, against the run before: pink noise 5.3% fewer instructions,
/// the indie record 3.6%, the Aria 3.1%, the 1962 speech 1.9%, the 1924
/// record 1.6% and the 16-bit recordings 0.3% to 0.8%, the same bytes.
pub fn residual(signal: &[i32], predictor: &Predictor, bps: u32, halves: &mut [Vec<i16>; 2], out: &mut Vec<i32>) -> bool {
    out.clear();
    let [upper, lower] = halves;
    let n = signal.len();
    let order = predictor.order;
    if bps <= 16 {
        // Samples and coefficients of 16 bits, which the compiler multiplies
        // and adds in pairs with one instruction at every x86-64's baseline.
        let run = match order {
            3 => 4,
            5 => 6,
            7 => 8,
            o => o,
        };
        let lead = run - order;
        upper.resize(n + lead, 0);
        upper[..lead].fill(0);
        for (h, &s) in upper[lead..].iter_mut().zip(signal) {
            *h = s as i16;
        }
        match run {
            1 => narrow::<1>(upper, predictor, out),
            2 => narrow::<2>(upper, predictor, out),
            4 => narrow::<4>(upper, predictor, out),
            6 => narrow::<6>(upper, predictor, out),
            _ => narrow::<8>(upper, predictor, out),
        }
        true
    } else if predictor.shift >= bps - 16 {
        // A wider sample as its upper 16 bits and its lower `bps - 16`, each
        // part's sum taken as the 16-bit path takes it; see `split`.
        let low = bps - 16;
        let run = match order {
            2 | 3 => 4,
            6 | 7 => 8,
            o => o,
        };
        let lead = run - order;
        upper.resize(n + lead, 0);
        lower.resize(n + lead, 0);
        upper[..lead].fill(0);
        lower[..lead].fill(0);
        for ((u, l), &s) in upper[lead..].iter_mut().zip(&mut lower[lead..]).zip(signal) {
            *u = (s >> low) as i16;
            *l = (s & ((1 << low) - 1)) as i16;
        }
        let predicted = &signal[order..];
        match run {
            1 => split::<1>(upper, lower, predicted, predictor, low, out),
            4 => split::<4>(upper, lower, predicted, predictor, low, out),
            5 => split::<5>(upper, lower, predicted, predictor, low, out),
            _ => split::<8>(upper, lower, predicted, predictor, low, out),
        }
        true
    } else {
        wide(signal, predictor, out)
    }
}

/// The residual of 16-bit samples with 32-bit sums, the order a constant so
/// the inner loop unrolls and the outer vectorises; `ORDER` is the loop's,
/// the predictor's or one above it with the taps past it zero. The output
/// is sized first and written through a slice: written through `extend`,
/// the stores could alias the closure's coefficients for all the compiler
/// knows, and the loop stays scalar.
fn narrow<const ORDER: usize>(signal: &[i16], predictor: &Predictor, out: &mut Vec<i32>) {
    let c: [i16; ORDER] = std::array::from_fn(|j| predictor.coefs[j] as i16);
    let shift = predictor.shift;
    out.resize(signal.len() - ORDER, 0);
    for (o, w) in out.iter_mut().zip(signal.windows(ORDER + 1)) {
        let mut sum = 0i32;
        for j in 0..ORDER {
            sum += i32::from(c[j]) * i32::from(w[ORDER - 1 - j]);
        }
        *o = i32::from(w[ORDER]) - (sum >> shift);
    }
}

/// The residual of samples of 17 to 24 bits, split as `upper * 2^low +
/// lower` with `upper` of 16 bits and `lower` of `low`, both as 16-bit
/// lanes: a sum of coefficients and samples is the upper part's sum times
/// `2^low` plus the lower's, and shifted down by `shift >= low` it is the
/// upper's sum plus the lower's shifted down by `low`, all shifted by the
/// rest, exactly, since the lower sum's bits below `low` cannot carry past
/// the floor. Neither sum outgrows 32 bits: coefficients are 13 bits at the
/// most, the upper part 16 and the lower 8, over eight taps. The 32-bit
/// lanes this replaced have no multiply at the x86-64 baseline, and took
/// four times as long on the side channel of a 16-bit stereo pair, which is
/// 17 bits wide. `predicted` is the samples predicted, from the
/// predictor's order on, and `ORDER` the loop's, the predictor's or one
/// above it with the halves led by as many zero samples.
fn split<const ORDER: usize>(upper: &[i16], lower: &[i16], predicted: &[i32], predictor: &Predictor, low: u32, out: &mut Vec<i32>) {
    let c: [i16; ORDER] = std::array::from_fn(|j| predictor.coefs[j] as i16);
    let shift = predictor.shift - low;
    out.resize(predicted.len(), 0);
    for (((o, &s), u), l) in out.iter_mut().zip(predicted).zip(upper.windows(ORDER)).zip(lower.windows(ORDER)) {
        let mut sum_upper = 0i32;
        let mut sum_lower = 0i32;
        for j in 0..ORDER {
            sum_upper += i32::from(c[j]) * i32::from(u[ORDER - 1 - j]);
            sum_lower += i32::from(c[j]) * i32::from(l[ORDER - 1 - j]);
        }
        *o = s - ((sum_upper + (sum_lower >> low)) >> shift);
    }
}

/// The residual with 64-bit sums, for a sample too wide for 32: false
/// where one would not fit, or would be `i32::MIN`, which a decoder may
/// not be able to negate (RFC 9639, 9.2.7.3).
fn wide(signal: &[i32], predictor: &Predictor, out: &mut Vec<i32>) -> bool {
    let order = predictor.order;
    let c = &predictor.coefs[..order];
    let shift = predictor.shift;
    for w in signal.windows(order + 1) {
        let sum: i64 = c.iter().zip(w[..order].iter().rev()).map(|(&c, &s)| i64::from(c) * i64::from(s)).sum();
        let Ok(r) = i32::try_from(i64::from(w[order]) - (sum >> shift)) else { return false };
        if r == i32::MIN {
            return false;
        }
        out.push(r);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A second-order recurrence is recovered exactly by Levinson-Durbin
    /// from its own autocorrelation, and leaves no error.
    #[test]
    fn levinson_recovers_a_recurrence() {
        // s[i] = 1.6 s[i-1] - 0.8 s[i-2], from an impulse.
        let mut s = vec![0f64; 512];
        s[0] = 1.0;
        for i in 1..s.len() {
            s[i] = 1.6 * s[i - 1] - 0.8 * if i >= 2 { s[i - 2] } else { 0.0 };
        }
        let mut autoc = [0f64; MAX_ORDER + 1];
        for (lag, o) in autoc.iter_mut().enumerate() {
            *o = (0..s.len() - lag).map(|i| s[i] * s[i + lag]).sum();
        }
        let mut coefs = [[0f64; MAX_ORDER]; MAX_ORDER];
        let mut errors = [0f64; MAX_ORDER];
        let orders = levinson(&autoc, 4, &mut coefs, &mut errors);
        assert!(orders >= 2, "{orders}");
        assert!((coefs[1][0] - 1.6).abs() < 1e-3 && (coefs[1][1] + 0.8).abs() < 1e-3, "{:?}", &coefs[1][..2]);
        // What is left is the impulse itself, which nothing before predicts.
        assert!((errors[1] - 1.0).abs() < 1e-6 && errors[0] > 1.5, "{:?}", &errors[..2]);
    }

    #[test]
    fn quantized_coefficients_fit_their_precision() {
        let autoc = [1.0, 0.5, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let (p, _) = quantize(&[1.6, -0.8], 10, &autoc).unwrap();
        assert_eq!((p.order, p.precision, p.shift), (2, 10, 8));
        assert_eq!(p.coefs[..2], [410, -205]);
        // A coefficient past the precision is scaled, never shifted up.
        let (p, _) = quantize(&[3000.0, 1.0], 10, &autoc).unwrap();
        assert_eq!(p.shift, 0);
        assert!(p.coefs[0] <= 511, "{:?}", p.coefs);
        assert!(quantize(&[0.0, 0.0], 10, &autoc).is_none());
    }

    /// A low tone's predictor, two cosines of its frequency and minus one,
    /// has its first coefficient just under 2, which rounds past the edge
    /// of the precision at the shift the largest coefficient sets and is
    /// clamped there, the clamp's error carried into the second, where it
    /// is nearly cancelled: the rounding error is the quadratic form, and
    /// a tenth of what the first coefficient's error alone would be.
    #[test]
    fn a_clamped_coefficient_carries_its_error_into_the_next() {
        let w: f64 = 200.0 / 48000.0 * std::f64::consts::TAU;
        let coefs = [2.0 * w.cos(), -1.0];
        let autoc: Autocorrelation = std::array::from_fn(|lag| (lag as f64 * w).cos());
        let (p, error) = quantize(&coefs, 10, &autoc).unwrap();
        assert_eq!((p.shift, p.coefs[0], p.coefs[1]), (8, 511, -255), "{p:?}");
        let e = [coefs[0] - 511.0 / 256.0, coefs[1] + 255.0 / 256.0];
        assert_eq!(error, e[0] * e[0] * autoc[0] + e[1] * e[1] * autoc[0] + 2.0 * e[0] * e[1] * autoc[1]);
        assert!(error * 10.0 < e[0] * e[0] * autoc[0], "{error}");
        // The error of the exact predictor is nothing, and of one off by a
        // whole in its first coefficient the autocorrelation's zero lag.
        let exact = Predictor { order: 2, precision: 10, shift: 0, coefs: [0; MAX_ORDER] };
        assert_eq!(rounding_error(&[0.0, 0.0], &exact, &autoc), 0.0);
        let off = Predictor { order: 1, precision: 10, shift: 0, coefs: [1, 0, 0, 0, 0, 0, 0, 0] };
        assert_eq!(rounding_error(&[0.0], &off, &autoc), autoc[0]);
        assert_eq!((round(2.5), round(-2.5), round(0.4), round(-0.4)), (3, -3, 0, 0));
        assert_eq!((exponent(1.0), exponent(1.9999), exponent(0.3), pow2(-3), pow2(10)), (0, 0, -2, 0.125, 1024.0));
    }

    /// What the quantized predictor adds to the fit's squared error on a
    /// tone, worked over the signal as the autocorrelation sees it, zeros
    /// beyond the block, with the exact coefficients and with the quantized
    /// ones, is the quadratic form of the rounding over the autocorrelation
    /// to a part in a million: a tone's residual is nearly all rounding,
    /// and the form is what the order choice weighs.
    #[test]
    fn the_rounding_error_is_what_the_quantized_predictor_adds() {
        let n = 960;
        let s: Vec<f64> = (0..n).map(|i| 16000.0 * (i as f64 * 440.0 / 48000.0 * std::f64::consts::TAU).sin() + ((i * 7919) % 13) as f64 - 6.0).collect();
        let mut autoc = [0f64; MAX_ORDER + 1];
        for (lag, o) in autoc.iter_mut().enumerate() {
            *o = (0..n - lag).map(|i| s[i] * s[i + lag]).sum();
        }
        let mut coefs = [[0f64; MAX_ORDER]; MAX_ORDER];
        let mut errors = [0f64; MAX_ORDER];
        assert!(levinson(&autoc, 2, &mut coefs, &mut errors) == 2);
        let exact = &coefs[1][..2];
        let (p, model) = quantize(exact, 10, &autoc).unwrap();
        let scale = pow2(-(p.shift as i32));
        let rounded = [f64::from(p.coefs[0]) * scale, f64::from(p.coefs[1]) * scale];
        let at = |i: i64| if (0..n as i64).contains(&i) { s[i as usize] } else { 0.0 };
        let energy = |c: &[f64]| (0..n as i64 + 2).map(|i| (at(i) - c[0] * at(i - 1) - c[1] * at(i - 2)).powi(2)).sum::<f64>();
        let added = energy(&rounded) - energy(exact);
        assert!(added > 0.0 && (added - model).abs() < 1e-6 * added, "added {added} against the model's {model}");
    }

    /// The fitted predictor leaves a tone's residual at little more than the
    /// noise on it, in the 16-bit path and the split one.
    #[test]
    fn a_tone_is_predicted_away() {
        let n = 960;
        let mut state = 0x2545_F491u32;
        let s: Vec<i32> = (0..n)
            .map(|i| {
                state ^= state << 13;
                state ^= state >> 17;
                state ^= state << 5;
                (20000.0 * (i as f64 * 0.3).sin()).round() as i32 + (state % 7) as i32 - 3
            })
            .collect();
        let window = Window::new(n);
        let mut windowed = Windowed::default();
        let width = 33 - s.iter().fold(0i32, |m, &s| m | (s ^ (s >> 31))).leading_zeros();
        for bps in [16, 17, 24] {
            let estimate = estimate(&autocorrelation(&s, width, &window, &mut windowed), n, MAX_ORDER, bps, precision(bps, n), Some(2));
            let fit = estimate.fit.expect("a tone is worth fitting");
            assert!(fit.bits() < estimate.fixed);
            let p = fit.predictor();
            let mut out = Vec::new();
            assert!(residual(&s, &p, bps, &mut [Vec::new(), Vec::new()], &mut out));
            assert_eq!(out.len(), n - p.order);
            let worst = out.iter().map(|r| r.abs()).max().unwrap();
            // The noise, amplified by the predictor, and the coefficients'
            // rounding on samples of fifteen bits.
            assert!(worst < 300, "{bps} bits: order {} leaves {worst}", p.order);
        }
    }

    #[test]
    fn the_window_tapers_a_quarter_each_end() {
        let w = Window::new(400);
        assert_eq!(w.taper.len(), 100);
        assert_eq!(w.taper[0], 0);
        assert!(w.taper[50] > 16300 && w.taper[50] < 16500);
        assert!(w.taper[99] > 32700);
        assert_eq!(w.exact[0], 0.0);
        assert!(w.exact[50] > 0.49 && w.exact[50] < 0.51);
        assert_eq!(w.exact[100], 1.0);
        assert_eq!(w.exact[299], 1.0);
        assert_eq!(w.exact[399], 0.0);
    }

    /// The scaled autocorrelation is within 3e-5 of the double-precision
    /// one at every lag, normalised, on a quiet tone of 10 bits, on a loud
    /// 16-bit mix, on 24-bit noise and on a block of 882; and a loud pure
    /// tone, whose fit leaves next to nothing, is summed exactly, the
    /// double-precision one to the last bits.
    #[test]
    fn the_scaled_autocorrelation_is_the_exact_one_within_the_scaling() {
        let mut state = 0x2545_F491u32;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            state
        };
        let signals: Vec<Vec<i32>> = vec![
            (0..960).map(|i| ((i as f64 * 0.0471).sin() * 300.0).round() as i32).collect(),
            (0..960).map(|i| ((i as f64 * 0.031).sin() * 9000.0 + (i as f64 * 0.173).sin() * 6000.0 + (i as f64 * 0.0071).sin() * 4000.0).round() as i32 + (next() % 211) as i32 - 105).collect(),
            (0..960).map(|_| (next() >> 8) as i32 - (1 << 23)).collect(),
            (0..882).map(|i| ((i as f64 * 0.0171).sin() * 20000.0 + (i as f64 * 0.0173).sin() * 20000.0).round() as i32).collect(),
            (0..960).map(|i| ((i as f64 * 0.0471).sin() * 30000.0).round() as i32).collect(),
        ];
        for (which, s) in signals.iter().enumerate() {
            let n = s.len();
            let taper = n / 4;
            let w = |i: usize| if i < taper { 0.5 - 0.5 * (std::f64::consts::PI * i as f64 / taper as f64).cos() } else if i >= n - taper { 0.5 - 0.5 * (std::f64::consts::PI * (n - 1 - i) as f64 / taper as f64).cos() } else { 1.0 };
            let x: Vec<f64> = s.iter().enumerate().map(|(i, &v)| f64::from(v) * w(i)).collect();
            let exact: Vec<f64> = (0..=MAX_ORDER).map(|lag| (0..n - lag).map(|i| x[i] * x[i + lag]).sum()).collect();
            let width = 33 - s.iter().fold(0i32, |m, &s| m | (s ^ (s >> 31))).leading_zeros();
            let got = autocorrelation(s, width, &Window::new(n), &mut Windowed::default());
            let bound = if which == 4 { 1e-12 } else { 3e-5 };
            for lag in 0..=MAX_ORDER {
                let err = (got[lag] / got[0] - exact[lag] / exact[0]).abs();
                assert!(err < bound, "signal {which}, width {width}, {n} samples, lag {lag}: {} against {}", got[lag], exact[lag]);
            }
        }
    }

    /// A wide sample's residual of exactly `i32::MIN`, which FLAC forbids, is
    /// refused like one that would not fit, where one past it by one is kept.
    #[test]
    fn the_most_negative_residual_is_refused() {
        let p = Predictor { order: 1, precision: 2, shift: 0, coefs: [1, 0, 0, 0, 0, 0, 0, 0] };
        let mut out = Vec::new();
        assert!(!wide(&[1, i32::MIN + 1], &p, &mut out));
        out.clear();
        assert!(wide(&[1, i32::MIN + 2], &p, &mut out));
        assert_eq!(out, [i32::MIN + 1]);
    }
}

