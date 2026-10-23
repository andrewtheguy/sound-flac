//! One channel of a block: how it is best coded, and writing it that way.
//!
//! A channel is coded as a constant if every sample is the same, as it is if
//! nothing beats that, and otherwise with the predictor that leaves the
//! smallest residual: first FLAC's fixed polynomial of the order that does,
//! and then, for a channel that is to be written, a linear predictor fitted
//! to the block if that writes fewer bits still. A residual is Rice-coded in
//! the partitions that cost least. Bits the samples all have zero at the
//! bottom are shifted out first, as FLAC's wasted bits.

use crate::bits::BitWriter;
use crate::lpc::{self, Predictor, Window};

/// The highest fixed predictor order FLAC has.
const MAX_FIXED_ORDER: usize = 4;
/// The finest partitioning tried, 256 partitions; FLAC allows up to 15.
const MAX_PARTITION_ORDER: u32 = 8;
/// The fewest residuals a partition is tried with: a parameter every fewer
/// seldom pays for itself, and the finest sums are the costliest to take.
const MIN_PARTITION: usize = 16;
/// The largest Rice parameter a 4-bit field holds, 15 being the escape.
const MAX_PARAM_4: u32 = 14;

/// How a channel is coded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Constant(i32),
    Verbatim,
    Fixed { order: usize },
    Lpc(Predictor),
}

impl Kind {
    /// The samples written as they are before the residual begins.
    fn order(&self) -> usize {
        match self {
            Kind::Constant(_) | Kind::Verbatim => 0,
            Kind::Fixed { order } => *order,
            Kind::Lpc(p) => p.order,
        }
    }
}

/// One way of coding the channel, with everything writing it needs.
struct Coding {
    kind: Kind,
    /// The predictor's residual, `signal.len() - order` of them.
    residual: Vec<i32>,
    /// The residual folded to unsigned, as Rice codes it.
    folded: Vec<u32>,
    /// The Rice parameter of each partition, at `partition_order`.
    chosen: Vec<u8>,
    partition_order: u32,
    /// Whether a parameter takes 5 bits rather than 4.
    wide_params: bool,
    /// The bits the subframe takes.
    bits: u64,
}

impl Coding {
    fn new(block: usize) -> Self {
        Self {
            kind: Kind::Verbatim,
            residual: vec![0; block],
            folded: vec![0; block],
            chosen: vec![0; 1 << MAX_PARTITION_ORDER],
            partition_order: 0,
            wide_params: false,
            bits: 0,
        }
    }
}

/// A channel of a block and what analysing it found; the buffers are reused
/// from block to block.
pub struct Channel {
    /// The samples, shifted down by `wasted` once analysed.
    pub signal: Vec<i32>,
    /// The coding chosen so far.
    best: Coding,
    /// Room for a coding tried against it.
    trial: Coding,
    /// Room for the orders' residuals while the fixed order is chosen.
    scratch: [Vec<i32>; 2],
    /// The samples windowed for the linear predictor's fit.
    windowed: Vec<f64>,
    /// The samples as 16 bits, for a linear predictor's residual when they fit.
    halves: Vec<i16>,
    /// Sums of a folded residual over the finest partitions.
    sums: Vec<u64>,
    /// The Rice parameters of the partitioning being tried.
    params: Vec<u8>,
    /// Bits every sample had zero at the bottom, shifted out.
    wasted: u32,
    /// The sample width this channel is coded at, less `wasted`.
    bps: u32,
    /// The precision a linear predictor's coefficients are quantized to.
    precision: u32,
    /// The fixed predictor order that left the smallest residual.
    fixed_order: usize,
}

impl Channel {
    /// A channel of `block` samples, coefficients at `precision` bits.
    pub fn new(block: usize, precision: u32) -> Self {
        Self {
            signal: vec![0; block],
            best: Coding::new(block),
            trial: Coding::new(block),
            scratch: [vec![0; block], vec![0; block]],
            windowed: vec![0.0; block],
            halves: vec![0; block],
            sums: vec![0; 1 << MAX_PARTITION_ORDER],
            params: vec![0; 1 << MAX_PARTITION_ORDER],
            wasted: 0,
            bps: 0,
            precision,
            fixed_order: 0,
        }
    }

    /// The bits the subframe takes as coded so far.
    pub fn bits(&self) -> u64 {
        self.best.bits
    }

    /// Decide how `signal`, of `bps` bits a sample, is coded without a fit,
    /// and count the bits. The passes over the samples are plain loops the
    /// compiler vectorises for whatever a target is built for; compiling them
    /// again for AVX2 and picking that at run time was measured to gain
    /// nothing.
    pub fn analyse(&mut self, bps: u32) {
        let n = self.signal.len();
        let all = self.signal.iter().fold(0i32, |acc, &s| acc | s);
        self.wasted = if all == 0 { 0 } else { all.trailing_zeros() };
        if self.wasted > 0 {
            for s in &mut self.signal {
                *s >>= self.wasted;
            }
        }
        self.bps = bps - self.wasted;
        let header = self.header();

        let first = self.signal[0];
        if self.signal.iter().all(|&s| s == first) {
            self.best.kind = Kind::Constant(first);
            self.best.bits = header + u64::from(self.bps);
            return;
        }
        self.best.kind = Kind::Verbatim;
        self.best.bits = header + n as u64 * u64::from(self.bps);

        self.fixed_order = 0;
        if n <= MAX_FIXED_ORDER {
            return;
        }
        let order = best_order(&self.signal, &mut self.scratch);
        self.fixed_order = order;
        residual(&self.signal, order, &mut self.best.residual);
        let bits = header + (order as u64) * u64::from(self.bps) + partition(&mut self.best, n, &mut self.sums, &mut self.params);
        if bits < self.best.bits {
            self.best.kind = Kind::Fixed { order };
            self.best.bits = bits;
        }
    }

    /// Fit a linear predictor to the channel analysed and keep it if it
    /// writes fewer bits than the coding so far.
    pub fn refine(&mut self, window: &Window) {
        if let Kind::Constant(_) = self.best.kind {
            return;
        }
        let n = self.signal.len();
        let max_order = lpc::MAX_ORDER.min(n / 8);
        if max_order == 0 {
            return;
        }
        let Some(predictor) = lpc::fit(&self.signal, window, &mut self.windowed, max_order, self.bps, self.precision, self.fixed_order) else { return };
        if !lpc::residual(&self.signal, &predictor, self.bps, &mut self.halves, &mut self.trial.residual) {
            return;
        }
        let order = predictor.order as u64;
        let bits = self.header()
            + order * u64::from(self.bps)
            + 4
            + 5
            + order * u64::from(predictor.precision)
            + partition(&mut self.trial, n, &mut self.sums, &mut self.params);
        if bits < self.best.bits {
            self.trial.kind = Kind::Lpc(predictor);
            self.trial.bits = bits;
            std::mem::swap(&mut self.best, &mut self.trial);
        }
    }

    /// The bits of the subframe header, with the wasted bits' count.
    fn header(&self) -> u64 {
        8 + u64::from(self.wasted)
    }

    /// Write the subframe as analysed.
    pub fn write(&self, w: &mut BitWriter<'_>) {
        let bps = self.bps;
        let coding = &self.best;
        w.put(0, 1);
        match coding.kind {
            Kind::Constant(_) => w.put(0b000000, 6),
            Kind::Verbatim => w.put(0b000001, 6),
            Kind::Fixed { order } => w.put(0b001000 | order as u64, 6),
            Kind::Lpc(p) => w.put(0b100000 | (p.order as u64 - 1), 6),
        }
        if self.wasted > 0 {
            w.put(1, 1);
            w.zeros(self.wasted - 1);
            w.put(1, 1);
        } else {
            w.put(0, 1);
        }
        match coding.kind {
            Kind::Constant(value) => {
                w.put(value as u32 as u64, bps);
                return;
            }
            Kind::Verbatim => {
                for &s in &self.signal {
                    w.put(s as u32 as u64, bps);
                }
                return;
            }
            Kind::Fixed { .. } => {}
            Kind::Lpc(p) => {
                for &s in &self.signal[..p.order] {
                    w.put(s as u32 as u64, bps);
                }
                w.put(u64::from(p.precision - 1), 4);
                w.put(u64::from(p.shift), 5);
                for &c in &p.coefs[..p.order] {
                    w.put(c as u32 as u64, p.precision);
                }
            }
        }
        if let Kind::Fixed { order } = coding.kind {
            for &s in &self.signal[..order] {
                w.put(s as u32 as u64, bps);
            }
        }
        let order = coding.kind.order();
        let (method, param_bits) = if coding.wide_params { (1, 5) } else { (0, 4) };
        w.put(method, 2);
        w.put(u64::from(coding.partition_order), 4);
        let n = self.signal.len();
        let parts = 1usize << coding.partition_order;
        let size = n >> coding.partition_order;
        for i in 0..parts {
            let k = u32::from(coding.chosen[i]);
            w.put(u64::from(k), param_bits);
            let start = if i == 0 { 0 } else { i * size - order };
            let mask = (1u32 << k) - 1;
            let (pairs, odd) = coding.folded[start..(i + 1) * size - order].as_chunks::<2>();
            for &[u1, u2] in pairs {
                // Two codes in one write where they fit a word, which at the
                // parameters sound gives they nearly always do.
                let (t1, t2) = ((u1 >> k) + 1 + k, (u2 >> k) + 1 + k);
                if t1 + t2 <= 63 {
                    let code = |u: u32| (1u64 << k) | u64::from(u & mask);
                    w.put((code(u1) << t2) | code(u2), t1 + t2);
                } else {
                    w.rice(u1 >> k, u1 & mask, k);
                    w.rice(u2 >> k, u2 & mask, k);
                }
            }
            for &u in odd {
                w.rice(u >> k, u & mask, k);
            }
        }
    }
}

/// Choose the partitioning and the Rice parameters of `coding`'s residual
/// over a block of `n`, and return the bits the residual coding takes.
fn partition(coding: &mut Coding, n: usize, sums: &mut [u64], params: &mut [u8]) -> u64 {
    let order = n - coding.residual.len();
    coding.folded.clear();
    coding.folded.extend(coding.residual.iter().map(|&r| zigzag(r)));
    let finest = MAX_PARTITION_ORDER.min(n.trailing_zeros()).min((n / (order + 1).max(MIN_PARTITION)).ilog2());
    let parts = 1usize << finest;
    let size = n >> finest;
    for (i, sum) in sums[..parts].iter_mut().enumerate() {
        let start = if i == 0 { 0 } else { i * size - order };
        *sum = wide_sum(&coding.folded[start..(i + 1) * size - order]);
    }
    let mut best = (u64::MAX, 0, false);
    for p in (0..=finest).rev() {
        let parts = 1usize << p;
        let size = n >> p;
        let mut total = 0;
        let mut wide = false;
        for (i, &sum) in sums[..parts].iter().enumerate() {
            let count = if i == 0 { size - order } else { size };
            let (k, bits) = best_param(sum, count as u64);
            wide |= k > MAX_PARAM_4;
            total += bits;
            params[i] = k as u8;
        }
        total += 2 + 4 + (parts as u64) * if wide { 5 } else { 4 };
        if total <= best.0 {
            best = (total, p, wide);
            coding.chosen[..parts].copy_from_slice(&params[..parts]);
        }
        // The sums of the partitioning one coarser.
        for i in 0..parts / 2 {
            sums[i] = sums[2 * i] + sums[2 * i + 1];
        }
    }
    let (bits, p, wide) = best;
    coding.partition_order = p;
    coding.wide_params = wide;
    bits
}

/// A residual folded to unsigned: 0, -1, 1, -2, 2, ... to 0, 1, 2, 3, 4, ...
#[inline]
fn zigzag(r: i32) -> u32 {
    ((r << 1) ^ (r >> 31)) as u32
}

/// The sum of `values`, taken in eight lanes of 32 bits over blocks of 32,
/// which the compiler vectorises: four of the 29 bits a folded residual can
/// have fit a lane.
#[inline]
fn wide_sum(values: &[u32]) -> u64 {
    let (blocks, rest) = values.as_chunks::<32>();
    let mut total = 0u64;
    for block in blocks {
        let mut lanes = [0u32; 8];
        for eight in block.as_chunks::<8>().0 {
            for (lane, &v) in lanes.iter_mut().zip(eight) {
                *lane += v;
            }
        }
        total += lanes.iter().map(|&lane| u64::from(lane)).sum::<u64>();
    }
    total + rest.iter().map(|&v| u64::from(v)).sum::<u64>()
}

/// The sum of the magnitudes of `values`, in eight lanes over blocks of 64:
/// eight of the 28 bits a residual can have fit a lane.
#[inline]
fn abs_sum(values: &[i32]) -> u64 {
    let (blocks, rest) = values.as_chunks::<64>();
    let mut total = 0u64;
    for block in blocks {
        let mut lanes = [0u32; 8];
        for eight in block.as_chunks::<8>().0 {
            for (lane, &v) in lanes.iter_mut().zip(eight) {
                *lane += v.unsigned_abs();
            }
        }
        total += lanes.iter().map(|&lane| u64::from(lane)).sum::<u64>();
    }
    total + rest.iter().map(|&v| u64::from(v.unsigned_abs())).sum::<u64>()
}

/// The fixed predictor order whose residual is smallest in all, judged over
/// the samples every order predicts; the lower order on a tie. Each order's
/// residual is the difference of the one before, taken in a pass of its own
/// through `scratch`, which the compiler vectorises.
#[inline]
fn best_order(signal: &[i32], scratch: &mut [Vec<i32>; 2]) -> usize {
    let n = signal.len();
    let [a, b] = scratch;
    let (a, b) = (&mut a[..n - 1], &mut b[..n - 2]);
    let mut sums = [0u64; MAX_FIXED_ORDER + 1];
    // An order's residual starts `order` samples in, so the samples every
    // order predicts begin that much later in each.
    sums[0] = abs_sum(&signal[MAX_FIXED_ORDER..]);
    difference(signal, a);
    sums[1] = abs_sum(&a[MAX_FIXED_ORDER - 1..]);
    difference(a, b);
    sums[2] = abs_sum(&b[MAX_FIXED_ORDER - 2..]);
    difference(b, &mut a[..n - 3]);
    sums[3] = abs_sum(&a[MAX_FIXED_ORDER - 3..n - 3]);
    difference(&a[..n - 3], &mut b[..n - 4]);
    sums[4] = abs_sum(&b[..n - 4]);
    let mut best = 0;
    for order in 1..=MAX_FIXED_ORDER {
        if sums[order] < sums[best] {
            best = order;
        }
    }
    best
}

/// Each value less the one before it, into `out`, one shorter.
#[inline]
fn difference(values: &[i32], out: &mut [i32]) {
    for ((o, &after), &before) in out.iter_mut().zip(&values[1..]).zip(values) {
        *o = after.wrapping_sub(before);
    }
}

/// The residual of the fixed predictor of `order` over `signal`, from its
/// `order`th sample on.
#[inline]
fn residual(signal: &[i32], order: usize, out: &mut Vec<i32>) {
    out.clear();
    let s = signal;
    match order {
        0 => out.extend_from_slice(s),
        1 => out.extend(s.windows(2).map(|w| w[1].wrapping_sub(w[0]))),
        2 => out.extend(s.windows(3).map(|w| w[2].wrapping_sub(2i32.wrapping_mul(w[1])).wrapping_add(w[0]))),
        3 => out.extend(s.windows(4).map(|w| {
            w[3].wrapping_sub(3i32.wrapping_mul(w[2])).wrapping_add(3i32.wrapping_mul(w[1])).wrapping_sub(w[0])
        })),
        _ => out.extend(s.windows(5).map(|w| {
            w[4].wrapping_sub(4i32.wrapping_mul(w[3]))
                .wrapping_add(6i32.wrapping_mul(w[2]))
                .wrapping_sub(4i32.wrapping_mul(w[1]))
                .wrapping_add(w[0])
        })),
    }
}

/// The Rice parameter for `count` residuals whose zigzag values sum to `sum`,
/// and the bits they take with it, estimated from the sum alone: the parameter
/// near the mean's logarithm, one either side of it tried, whichever is
/// cheapest. No division: the logarithms' difference is within one of the
/// mean's.
#[inline]
fn best_param(sum: u64, count: u64) -> (u32, u64) {
    if sum < count {
        return (0, count + sum);
    }
    let near = sum.ilog2() - count.ilog2();
    let cost = |k: u32| count * (u64::from(k) + 1) + (sum >> k);
    let mut best = (near, cost(near));
    for k in [near.saturating_sub(1), near + 1] {
        let bits = cost(k);
        if bits < best.1 {
            best = (k, bits);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_order_with_the_smallest_residual_is_chosen() {
        let mut scratch = [vec![0; 64], vec![0; 64]];
        let ramp: Vec<i32> = (0..64).collect();
        assert_eq!(best_order(&ramp, &mut scratch), 2, "a ramp has a zero second difference");
        let parabola: Vec<i32> = (0..64).map(|i| i * i).collect();
        assert_eq!(best_order(&parabola, &mut scratch), 3);
        let noise: Vec<i32> = (0..64).map(|i| ((i * 7919) % 201) - 100).collect();
        assert_eq!(best_order(&noise, &mut scratch), 0, "noise is left as it is");
    }

    #[test]
    fn a_residual_is_what_the_predictor_misses() {
        let mut out = Vec::new();
        residual(&[1, 4, 9, 16, 25, 36], 2, &mut out);
        assert_eq!(out, [2, 2, 2, 2]);
        residual(&[1, 4, 9, 16, 25, 36], 0, &mut out);
        assert_eq!(out, [1, 4, 9, 16, 25, 36]);
    }

    #[test]
    fn the_parameter_tracks_the_mean() {
        assert_eq!(best_param(0, 100), (0, 100));
        // A mean of 8 wants a parameter near 3.
        let (k, _) = best_param(800, 100);
        assert!((2..=3).contains(&k), "{k}");
        let (k, _) = best_param(100 << 20, 100);
        assert!((19..=20).contains(&k), "{k}");
    }

    #[test]
    fn wasted_bits_are_shifted_out() {
        let mut ch = Channel::new(64, 10);
        for (i, s) in ch.signal.iter_mut().enumerate() {
            *s = ((i as i32 % 7) - 3) << 8;
        }
        ch.analyse(16);
        assert_eq!(ch.wasted, 8);
        assert_eq!(ch.bps, 8);
        assert!(ch.signal.iter().all(|&s| (-3..=3).contains(&s)));
    }

    /// A tone the fixed predictors leave much of is nearly all predicted by
    /// the fitted one, which is kept; noise is left as it was.
    #[test]
    fn the_fit_is_kept_only_where_it_wins() {
        let n = 960;
        let window = Window::new(n);
        let mut ch = Channel::new(n, 10);
        for (i, s) in ch.signal.iter_mut().enumerate() {
            *s = (12000.0 * (i as f64 * 0.35).sin() + 6000.0 * (i as f64 * 0.9).sin()).round() as i32;
        }
        ch.analyse(16);
        let fixed = ch.bits();
        ch.refine(&window);
        assert!(matches!(ch.best.kind, Kind::Lpc(_)), "{:?}", ch.best.kind);
        assert!(ch.bits() * 5 < fixed * 4, "fitted {} against fixed {fixed}", ch.bits());
        assert_eq!(ch.best.residual.len() + ch.best.kind.order(), n);

        let mut ch = Channel::new(n, 10);
        let mut state = 0x9E37_79B9u32;
        for s in &mut ch.signal {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            *s = (state >> 21) as i32 - 1024;
        }
        ch.analyse(16);
        let before = (ch.best.kind, ch.bits());
        ch.refine(&window);
        assert_eq!((ch.best.kind, ch.bits()), before);
    }
}
