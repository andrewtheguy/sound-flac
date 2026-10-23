//! A linear predictor fitted to one channel of a block, as FLAC's LPC
//! subframe carries it: coefficients quantized to a precision and a shift,
//! and the residual they leave.
//!
//! The samples are seen through a Tukey window, their autocorrelation taken
//! to the highest order, and Levinson-Durbin gives the predictor of every
//! order up to it with the error each leaves; the order estimated cheapest,
//! residual and coefficients together, is quantized and kept.

/// The highest order fitted; libFLAC's default level stops here too.
pub const MAX_ORDER: usize = 8;
/// The most a coefficient shift can be, FLAC's field holding 0 to 15.
const MAX_SHIFT: i32 = 15;
/// The bits a sample the fit may be estimated to lose to the fixed predictor
/// and still be tried: the estimates' difference was measured to be within
/// this of the truth, no block's bytes changing between this and no margin.
const MARGIN: f64 = 1.0 / 16.0;

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
pub struct Window(Vec<f32>);

impl Window {
    pub fn new(n: usize) -> Self {
        let taper = n / 4;
        let mut w = vec![1.0f32; n];
        for i in 0..taper {
            let v = (0.5 - 0.5 * (std::f64::consts::PI * i as f64 / taper as f64).cos()) as f32;
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

/// Fit a predictor of up to `max_order` to `signal`, of `bps` bits a sample,
/// with coefficients of about `precision` bits: the order whose estimated
/// bits, residual and coefficients together, are fewest. None where nothing
/// of the signal is left to predict, or where the fixed predictor of
/// `fixed_order` is estimated, from the same autocorrelation, to do as well:
/// the estimates share whatever a Rice code adds to the error's logarithm,
/// so their difference is a fair guess at the real one.
pub fn fit(signal: &[i32], window: &Window, windowed: &mut [f32], max_order: usize, bps: u32, precision: u32, fixed_order: usize) -> Option<Predictor> {
    let n = signal.len();
    debug_assert!((1..=MAX_ORDER).contains(&max_order) && n > max_order);
    for ((x, &s), &w) in windowed.iter_mut().zip(signal).zip(&window.0) {
        *x = s as f32 * w;
    }
    let mut autoc = [0f64; MAX_ORDER + 1];
    autocorrelation(windowed, &mut autoc);

    let mut coefs = [[0f64; MAX_ORDER]; MAX_ORDER];
    let mut errors = [0f64; MAX_ORDER];
    let orders = levinson(&autoc, max_order, &mut coefs, &mut errors);
    if orders == 0 {
        return None;
    }

    // The order estimated cheapest: the residual's bits from the error it
    // leaves, as libFLAC estimates them, and each coefficient at the width
    // of a sample and a coefficient.
    let scale = 0.5 / n as f64;
    let overhead = f64::from(bps + precision);
    let estimate = |err: f64, order: usize, overhead: f64| 0.5 * (err * scale).log2() * (n - order) as f64 + order as f64 * overhead;
    let mut best = (0, f64::INFINITY);
    for (i, &err) in errors[..orders].iter().enumerate() {
        let order = i + 1;
        let bits = estimate(err, order, overhead);
        if bits < best.1 {
            best = (order, bits);
        }
    }
    let (order, bits) = best;
    if bits >= estimate(fixed_error(&autoc, fixed_order), fixed_order, f64::from(bps)) + MARGIN * n as f64 {
        return None;
    }
    // Coefficients fine enough, and for a sample of 17 bits or fewer no finer
    // than lets the prediction's sum stay in 32 bits.
    let precision = if bps <= 17 { precision.min(32 - bps - order.ilog2()) } else { precision };
    quantize(&coefs[order - 1][..order], precision)
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

/// `out[lag]` is the sum of `x[i] * x[i + lag]` over the block for every lag
/// to the highest order, all nine taken in one pass of eight lanes of single
/// precision, which the compiler vectorises; a block's rounding is a few
/// parts in a thousand million, and the predictor is quantized far coarser.
/// A pass a lag, or lanes split so their accumulators stay in registers, both
/// measured slower.
fn autocorrelation(x: &[f32], out: &mut [f64; MAX_ORDER + 1]) {
    let n = x.len();
    let mut lanes = [[0f32; 8]; MAX_ORDER + 1];
    let mut i = 0;
    while i + 8 + MAX_ORDER <= n {
        let base: &[f32; 8] = x[i..i + 8].try_into().expect("eight");
        for (lag, lanes) in lanes.iter_mut().enumerate() {
            let shifted: &[f32; 8] = x[i + lag..i + lag + 8].try_into().expect("eight");
            for ((lane, &a), &b) in lanes.iter_mut().zip(base).zip(shifted) {
                *lane += a * b;
            }
        }
        i += 8;
    }
    for (lag, (o, lanes)) in out.iter_mut().zip(&lanes).enumerate() {
        let tail = x[i..n - lag].iter().zip(&x[i + lag..]).map(|(&a, &b)| f64::from(a * b)).sum::<f64>();
        *o = lanes.iter().map(|&lane| f64::from(lane)).sum::<f64>() + tail;
    }
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
    let log2cmax = cmax.log2().floor() as i32;
    // The shift that puts the largest coefficient just under the top, one
    // bit kept for the sign; the field holds no more than 15, and a negative
    // shift is not allowed, so too large a coefficient is scaled down
    // instead.
    let shift = (precision as i32 - 2 - log2cmax).min(MAX_SHIFT);
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
/// bits. False where a residual would not fit a 32-bit sample, which only a
/// wide sample's wild predictor can make.
pub fn residual(signal: &[i32], predictor: &Predictor, bps: u32, halves: &mut Vec<i16>, out: &mut Vec<i32>) -> bool {
    out.clear();
    if bps <= 16 {
        // Samples and coefficients of 16 bits, which the compiler multiplies
        // and adds in pairs with one instruction at every x86-64's baseline.
        halves.clear();
        halves.extend(signal.iter().map(|&s| s as i16));
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
    } else if predictor.precision + bps + predictor.order.ilog2() <= 32 {
        // The prediction's sum fits 32 bits when the width, the precision
        // and the order's logarithm do.
        match predictor.order {
            1 => middling::<1>(signal, predictor, out),
            2 => middling::<2>(signal, predictor, out),
            3 => middling::<3>(signal, predictor, out),
            4 => middling::<4>(signal, predictor, out),
            5 => middling::<5>(signal, predictor, out),
            6 => middling::<6>(signal, predictor, out),
            7 => middling::<7>(signal, predictor, out),
            _ => middling::<8>(signal, predictor, out),
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

/// The residual with 32-bit sums of 32-bit samples.
fn middling<const ORDER: usize>(signal: &[i32], predictor: &Predictor, out: &mut Vec<i32>) {
    let c: [i32; ORDER] = std::array::from_fn(|j| predictor.coefs[j]);
    let shift = predictor.shift;
    out.resize(signal.len() - ORDER, 0);
    for (o, w) in out.iter_mut().zip(signal.windows(ORDER + 1)) {
        let mut sum = 0i32;
        for j in 0..ORDER {
            sum += c[j] * w[ORDER - 1 - j];
        }
        *o = w[ORDER] - (sum >> shift);
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
    /// noise on it, in the 16-bit, the 32-bit and the 64-bit paths.
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
        let mut windowed = vec![0f32; n];
        for bps in [16, 17, 24] {
            let p = fit(&s, &window, &mut windowed, MAX_ORDER, bps, precision(bps, n), 2).unwrap();
            let mut out = Vec::new();
            assert!(residual(&s, &p, bps, &mut Vec::new(), &mut out));
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

