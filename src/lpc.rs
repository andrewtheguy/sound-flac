//! A linear predictor fitted to one channel of a block, as FLAC's LPC
//! subframe carries it: coefficients quantized to a precision and a shift,
//! and the residual they leave.
//!
//! The samples are seen through a Tukey window, their autocorrelation taken
//! to the highest order, and Levinson-Durbin gives the predictor of every
//! order up to it with the error each leaves; the order estimated cheapest,
//! residual and coefficients together, is kept as a candidate with its
//! estimate, and quantized where the channel is to be written.

/// The highest order fitted; libFLAC's default level stops here too.
pub const MAX_ORDER: usize = 8;
/// The highest fixed predictor order FLAC has.
pub const MAX_FIXED_ORDER: usize = 4;
/// The most a coefficient shift can be, FLAC's field holding 0 to 15.
const MAX_SHIFT: i32 = 15;
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
    pub coefs: [i32; MAX_ORDER],
}

/// The window the samples are seen through before the fit: Tukey, a quarter
/// of the block tapered by a half cosine at each end, as libFLAC's default.
pub struct Window(Vec<f64>);

impl Window {
    pub fn new(n: usize) -> Self {
        let taper = n / 4;
        let mut w = vec![1.0f64; n];
        for i in 0..taper {
            let v = 0.5 - 0.5 * (std::f64::consts::PI * i as f64 / taper as f64).cos();
            w[i] = v;
            w[n - 1 - i] = v;
        }
        Self(w)
    }
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

/// A predictor fitted but not yet quantized, with the bits it is estimated
/// to take, residual, warm-up samples and coefficients together.
pub struct Candidate {
    order: usize,
    coefs: [f64; MAX_ORDER],
    bits: f64,
}

impl Candidate {
    /// The bits the fit is estimated to take, on the same scale as
    /// [`Estimate::fixed`].
    pub fn bits(&self) -> f64 {
        self.bits
    }
}

/// A channel's autocorrelation through the window, at every lag to the
/// highest order.
pub type Autocorrelation = [f64; MAX_ORDER + 1];

/// The autocorrelation of `signal` seen through `window`, `windowed` being
/// room for the product.
pub fn autocorrelation(signal: &[i32], window: &Window, windowed: &mut [f64]) -> Autocorrelation {
    for ((x, &s), &w) in windowed.iter_mut().zip(signal).zip(&window.0) {
        *x = f64::from(s) * w;
    }
    lags(windowed)
}

/// Fit a predictor of up to `max_order` to a channel of `n` samples of `bps`
/// bits from its autocorrelation, with coefficients of about `precision`
/// bits, and estimate its bits beside the fixed predictor's from the same
/// autocorrelation, of `fixed_order` where the samples have chosen one and
/// otherwise of the order the autocorrelation says is best. The fit is the order
/// whose estimated bits are fewest; the orders that predict a tone away tie
/// at the floor and the lowest is taken.
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
    let mut best = (0, f64::INFINITY);
    for (i, &err) in errors[..orders].iter().enumerate() {
        let order = i + 1;
        let bits = bits_of(err, order, overhead);
        if bits < best.1 {
            best = (order, bits);
        }
    }
    let (order, bits) = best;
    let fit = (order > 0 && bits < fixed + MARGIN * n as f64).then(|| Candidate { order, coefs: coefs[order - 1], bits });
    Estimate { fixed, fixed_order, fit }
}

impl Candidate {
    /// The candidate quantized for a sample of `bps` bits: coefficients of
    /// `precision` bits, and for a sample of 17 bits or fewer no finer than
    /// lets the prediction's sum stay in 32 bits. None where they round to
    /// nothing.
    pub fn quantized(&self, bps: u32, precision: u32) -> Option<Predictor> {
        quantize(&self.coefs[..self.order], precision_for(bps, precision, self.order))
    }
}

/// The precision a predictor of `order` is quantized to for a sample of
/// `bps` bits, from the stream's `precision`: for a sample of 17 bits or
/// fewer no finer than lets the prediction's sum stay in 32 bits.
fn precision_for(bps: u32, precision: u32, order: usize) -> u32 {
    if bps <= 17 { precision.min(32 - bps - order.ilog2()) } else { precision }
}

/// The shift that puts the largest of `coefs` just under the top of
/// `precision` bits, one kept for the sign; the field holds no more than
/// 15, and a negative shift is not allowed, so too large a coefficient is
/// scaled down instead. Zero where there is no coefficient to scale.
fn shift_for(coefs: &[f64], precision: u32) -> i32 {
    let cmax = coefs.iter().fold(0f64, |m, &c| m.max(c.abs()));
    if cmax <= 0.0 || !cmax.is_finite() {
        return 0;
    }
    (precision as i32 - 2 - cmax.log2().floor() as i32).min(MAX_SHIFT)
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

/// The sum of `x[i] * x[i + lag]` over the block for every lag to the
/// highest order, in double precision: the predictor of a quiet, resonant
/// block is as sensitive to the sum's rounding as to its own quantization,
/// and single precision was measured to cost 0.5% to 1% of the bytes on
/// piano and voice.
///
/// One pass over the samples two at a time, each seen through a window of
/// the two and the eight after them, so every lag's pair is in bounds by
/// the window's length and nothing is checked in the loop. The compiler
/// lays the nine lags' sums out two to a register, a sample broadcast
/// against the pairs of samples after it, 18 floating-point operations on
/// two samples, which is the floor the x86-64 baseline's two ports set:
/// 1.4 µs at 960 samples, where the two passes of five and four lags this
/// replaced, each pair indexed and checked, took 2.4 measured apart and
/// 2.8 in the encoder. Four lanes a lag were measured no faster, the
/// floating-point work being the same. Not inlined, so that the nine sums
/// and their operands have the registers to themselves whatever the caller
/// holds, which is where the one pass lost before.
#[inline(never)]
fn lags(x: &[f64]) -> Autocorrelation {
    const W: usize = 2;
    let n = x.len();
    let mut lanes = [[0f64; W]; MAX_ORDER + 1];
    let whole = n.saturating_sub(MAX_ORDER) / W * W;
    for w in x[..whole + MAX_ORDER.min(n)].windows(W + MAX_ORDER).step_by(W) {
        for (lag, lanes) in lanes.iter_mut().enumerate() {
            for j in 0..W {
                lanes[j] += w[j] * w[j + lag];
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
/// before it carried along, as libFLAC does. None where they are all zero.
fn quantize(coefs: &[f64], precision: u32) -> Option<Predictor> {
    let cmax = coefs.iter().fold(0f64, |m, &c| m.max(c.abs()));
    if cmax <= 0.0 || !cmax.is_finite() {
        return None;
    }
    let shift = shift_for(coefs, precision);
    let scale = 2f64.powi(shift);
    let qmax = (1i64 << (precision - 1)) - 1;
    let qmin = -qmax - 1;
    let mut out = Predictor { order: coefs.len(), precision, shift: shift.max(0) as u32, coefs: [0; MAX_ORDER] };
    let mut error = 0f64;
    for (q, &c) in out.coefs.iter_mut().zip(coefs) {
        error += c * scale;
        let rounded = (error.round() as i64).clamp(qmin, qmax);
        error -= rounded as f64;
        *q = rounded as i32;
    }
    Some(out)
}

/// The residual `predictor` leaves over `signal`, of `bps` bits a sample,
/// from its `order`th sample on, `halves` being room for the samples as 16
/// bits, or as their upper 16 bits and their lower bits. False where a
/// residual would not fit a 32-bit sample, which only a wide sample's wild
/// predictor can make.
pub fn residual(signal: &[i32], predictor: &Predictor, bps: u32, halves: &mut [Vec<i16>; 2], out: &mut Vec<i32>) -> bool {
    out.clear();
    let [upper, lower] = halves;
    if bps <= 16 {
        // Samples and coefficients of 16 bits, which the compiler multiplies
        // and adds in pairs with one instruction at every x86-64's baseline.
        upper.resize(signal.len(), 0);
        for (h, &s) in upper.iter_mut().zip(signal) {
            *h = s as i16;
        }
        let halves = &upper[..];
        match predictor.order {
            1 => narrow::<1>(halves, predictor, out),
            2 => narrow::<2>(halves, predictor, out),
            3 => narrow::<3>(halves, predictor, out),
            4 => narrow::<4>(halves, predictor, out),
            5 => narrow::<5>(halves, predictor, out),
            6 => narrow::<6>(halves, predictor, out),
            7 => narrow::<7>(halves, predictor, out),
            _ => narrow::<8>(halves, predictor, out),
        }
        true
    } else if predictor.shift >= bps - 16 {
        // A wider sample as its upper 16 bits and its lower `bps - 16`, each
        // part's sum taken as the 16-bit path takes it; see `split`.
        let low = bps - 16;
        upper.resize(signal.len(), 0);
        lower.resize(signal.len(), 0);
        for ((u, l), &s) in upper.iter_mut().zip(lower.iter_mut()).zip(signal) {
            *u = (s >> low) as i16;
            *l = (s & ((1 << low) - 1)) as i16;
        }
        match predictor.order {
            1 => split::<1>(upper, lower, signal, predictor, low, out),
            2 => split::<2>(upper, lower, signal, predictor, low, out),
            3 => split::<3>(upper, lower, signal, predictor, low, out),
            4 => split::<4>(upper, lower, signal, predictor, low, out),
            5 => split::<5>(upper, lower, signal, predictor, low, out),
            6 => split::<6>(upper, lower, signal, predictor, low, out),
            7 => split::<7>(upper, lower, signal, predictor, low, out),
            _ => split::<8>(upper, lower, signal, predictor, low, out),
        }
        true
    } else {
        wide(signal, predictor, out)
    }
}

/// The residual of 16-bit samples with 32-bit sums, the order a constant so
/// the inner loop unrolls and the outer vectorises. The output is sized
/// first and written through a slice: written through `extend`, the stores
/// could alias the closure's coefficients for all the compiler knows, and
/// the loop stays scalar.
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
/// 17 bits wide.
fn split<const ORDER: usize>(upper: &[i16], lower: &[i16], signal: &[i32], predictor: &Predictor, low: u32, out: &mut Vec<i32>) {
    let c: [i16; ORDER] = std::array::from_fn(|j| predictor.coefs[j] as i16);
    let shift = predictor.shift - low;
    out.resize(signal.len() - ORDER, 0);
    for (((o, &s), u), l) in out.iter_mut().zip(&signal[ORDER..]).zip(upper.windows(ORDER)).zip(lower.windows(ORDER)) {
        let mut sum_upper = 0i32;
        let mut sum_lower = 0i32;
        for j in 0..ORDER {
            sum_upper += i32::from(c[j]) * i32::from(u[ORDER - 1 - j]);
            sum_lower += i32::from(c[j]) * i32::from(l[ORDER - 1 - j]);
        }
        *o = s - ((sum_upper + (sum_lower >> low)) >> shift);
    }
}

/// The residual with 64-bit sums, for a sample too wide for 32.
fn wide(signal: &[i32], predictor: &Predictor, out: &mut Vec<i32>) -> bool {
    let order = predictor.order;
    let c = &predictor.coefs[..order];
    let shift = predictor.shift;
    for w in signal.windows(order + 1) {
        let sum: i64 = c.iter().zip(w[..order].iter().rev()).map(|(&c, &s)| i64::from(c) * i64::from(s)).sum();
        let Ok(r) = i32::try_from(i64::from(w[order]) - (sum >> shift)) else { return false };
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
        let p = quantize(&[1.6, -0.8], 10).unwrap();
        assert_eq!((p.order, p.precision, p.shift), (2, 10, 8));
        assert_eq!(p.coefs[..2], [410, -205]);
        // A coefficient past the precision is scaled, never shifted up.
        let p = quantize(&[3000.0, 1.0], 10).unwrap();
        assert_eq!(p.shift, 0);
        assert!(p.coefs[0] <= 511, "{:?}", p.coefs);
        assert!(quantize(&[0.0, 0.0], 10).is_none());
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
        let mut windowed = vec![0f64; n];
        for bps in [16, 17, 24] {
            let estimate = estimate(&autocorrelation(&s, &window, &mut windowed), n, MAX_ORDER, bps, precision(bps, n), Some(2));
            let fit = estimate.fit.expect("a tone is worth fitting");
            assert!(fit.bits() < estimate.fixed);
            let p = fit.quantized(bps, precision(bps, n)).unwrap();
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
        assert_eq!(w.0[0], 0.0);
        assert!(w.0[50] > 0.49 && w.0[50] < 0.51);
        assert_eq!(w.0[100], 1.0);
        assert_eq!(w.0[299], 1.0);
        assert_eq!(w.0[399], 0.0);
    }
}

