//! One channel of a block: how it is best coded, and writing it that way.
//!
//! A channel is taken in with its wasted bits shifted out, the width its
//! samples need found, and coded as a constant if every sample is the same
//! ([`Channel::prepare`]); otherwise it is as it is until it is coded with
//! the predictor that leaves the smallest residual. Where a stereo pair is
//! being chosen, each channel is weighed before any is coded
//! ([`Channel::weigh`]): the bits its fit or its best fixed predictor is
//! estimated to take, from its autocorrelation. Where the estimates do not
//! settle the pair, each channel is counted ([`Channel::count`]): the fit's
//! residual is taken and counted in one Rice partition, a true count of
//! its bits within a partitioning's gain, and the fixed polynomial of the
//! order the autocorrelation chooses is counted the same way beside it
//! where the fit is not clear; the pair is chosen on those, and its two
//! channels are finished from what was counted or weighed
//! ([`Channel::finish`]). A channel coded is coded the fit first
//! ([`Channel::code_alone`], [`Channel::code_fit_first`]): the fit's and
//! the best fixed predictor's estimates are taken from the one
//! autocorrelation, and where the fit is estimated to win by [`ALONE`] a
//! sample after the channel's correction it is coded and the fixed path,
//! FLAC's fixed polynomial of the order that leaves the smallest residual,
//! coded and counted ([`Channel::analyse_fixed`]), is not run; otherwise
//! both are coded and the better kept, which is also what brings the
//! correction up to date; a channel whose fit has lost that comparison by
//! [`LOSS`] a sample [`LOSSES`] blocks running has the fit skipped for
//! [`SKIPS`] blocks, as a channel of noise has. A channel not coded on a block is bounded from
//! one pass over its differences ([`Channel::at_most`]). A residual is
//! Rice-coded in the partitions that cost least.
//!
//! Measured on 20 ms blocks of 16-bit stereo at 48 kHz, a channel's fixed
//! orders and their sums are about 1.1 µs where all four orders are tried,
//! a pass each, and the search stops at the first order that does not win,
//! its fixed coding's partitions
//! 0.8, its autocorrelation 1.4 and the fit from it 0.3, the fit's residual
//! 1.1 through 16-bit lanes, which a side channel takes too where its
//! samples fit 16 bits, and 1.6 split where they do not, and its partitions
//! 0.8, of which the fold and the finest sums that count it in one
//! partition are 0.25; the bound 0.3; the fixed predictor's one-partition
//! count 0.3 at order 0,
//! 0.22, 0.27 and 0.33 at orders 1 to 3 and 0.54 at order 4, taken as
//! order 2 twice where one pass of five taps took 0.77; writing a block's
//! two channels is about 3.4 µs at 16 bits and 6 at 24, the Rice codes
//! four, three or two to a word by the parameter. Measured apart at
//! 960 samples: the four fixed orders in one pass, with the binomial taps
//! as differences of differences, did not vectorise and took 4.3 µs
//! against the four passes' 1.1, and the sums alone then the chosen order
//! 1.2, so the four passes stay; a partition's fold is 0.14 and its finest
//! sums 0.09 in 32-bit lanes against 0.18 in 64-bit, the search over the
//! orders 0.4, and stopping that search at the first order that does not
//! win, from the coarsest, cost 0.07% of the orchestra's bytes, at the
//! second rise no bytes and no time; the one-partition count without its
//! residual stored was no faster than with it.

use crate::bits::BitWriter;
use crate::lpc::{self, Autocorrelation, Candidate, Estimate, MAX_FIXED_ORDER, Predictor, Window};
/// The finest partitioning tried, 256 partitions; FLAC allows up to 15.
const MAX_PARTITION_ORDER: u32 = 8;
/// The fewest residuals a partition is tried with: a parameter every fewer
/// seldom pays for itself, and the finest sums are the costliest to take.
const MIN_PARTITION: usize = 16;
/// The largest Rice parameter a 4-bit field holds, 15 being the escape.
const MAX_PARAM_4: u32 = 14;
/// The largest a 5-bit field holds, 31 being the escape: a parameter is
/// never chosen past it, however wild a wide sample's residual, so a
/// partition's header is always a parameter and never an escape.
const MAX_PARAM_5: u32 = 30;
/// The bits a sample a fit must be estimated to save over the best fixed
/// predictor, after the channel's correction, for a channel coded on its
/// own to be coded with the fit alone, its fixed path not run.
const ALONE: f64 = 1.0 / 8.0;
/// The bits a sample a stereo pair must be estimated to cost under every
/// other, over the block, for it to be taken without the four channels
/// counted. The estimates of the four channels are off against each other
/// by up to an eighth of a bit a sample on 8-bit music, where every block's
/// estimated pair is wrong, and by less than a tenth on every recording
/// measured; past a quarter no count changed a choice.
pub(crate) const PAIR: f64 = 1.0 / 4.0;
/// The bits a sample a fit coded alone may cost over what its estimate and
/// the channel's last fit said before the fixed path is run after all,
/// since what the estimate does not see, a pure tone's coefficients'
/// rounding, can be the whole of a frame.
const SLACK: f64 = 1.0 / 4.0;

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
    /// The residual of each fixed predictor order from 1 to 4, which choosing
    /// the order leaves behind, so the one chosen is not computed again.
    scratch: [Vec<i32>; MAX_FIXED_ORDER],
    /// The samples windowed for the linear predictor's fit.
    windowed: Vec<f64>,
    /// The samples as 16 bits for a linear predictor's residual, or where
    /// they are wider their upper 16 bits and their lower bits.
    halves: [Vec<i16>; 2],
    /// Sums of a folded residual over the finest partitions.
    sums: Vec<u64>,
    /// The Rice parameters of the partitioning being tried.
    params: Vec<u8>,
    /// Bits every sample had zero at the bottom, shifted out.
    wasted: u32,
    /// The sample width this channel is coded at, less `wasted`.
    bps: u32,
    /// The width the shifted samples need, `bps` or less: a side channel
    /// of two 16-bit channels is 17 bits wide by the stream and seldom in
    /// its samples, and a residual over 16-bit samples is the cheaper pass.
    width: u32,
    /// The precision a linear predictor's coefficients are quantized to.
    precision: u32,
    /// The fixed predictor order that left the smallest residual.
    fixed_order: usize,
    /// The bits the channel costs in one Rice partition, for choosing a
    /// pair before either channel is finished.
    coarse: u64,
    /// The estimate the channel was weighed on, until it is counted or
    /// finished from it.
    weighed: Option<Estimate>,
    /// What finishing the channel counted needs, until it is.
    counted: Option<Counted>,
    /// Whether `best` is the coding to write, or only the start of one.
    coded: bool,
    /// Whether the fixed path has run on this block.
    analysed: bool,
    /// The bound on the channel's bits found this block, if one was.
    bound: Option<u64>,
    /// Whether the block is long enough for a linear predictor to be fitted.
    predictable: bool,
    /// Whether the last fit weighed was not worth trying.
    fit_declined: bool,
    /// The blocks in a row the fit, coded beside the fixed coding, has
    /// lost to it on real bits; a block with no such comparison, a constant,
    /// ends the row, unless the fit is being skipped on the row's account.
    lost: u8,
    /// The blocks in a row the fit has been skipped on.
    skipped: u8,
    /// The last fit's and fixed coding's estimated bits, for the corrections.
    est_fit: f64,
    est_fixed: f64,
    /// How much the fit's real bits over its estimate exceeded the fixed
    /// coding's over its own, a sample, the last time both were coded: the
    /// estimates are from the windowed samples and blind to the
    /// coefficients' rounding, and the fit's advantage is overstated by
    /// this, 0.2 bits a sample and more on an orchestra at 16 bits, about
    /// none on piano at 24. None until both have been coded.
    correction: Option<f64>,
    /// The last coded fit's real bits over its estimate, a sample.
    fit_over: f64,
}

/// The blocks in a row a fit is skipped on before it is tried again.
const SKIPS: u8 = 7;
/// The blocks in a row a fit must lose to the fixed coding by [`LOSS`] a
/// sample, both coded, for the fit to be skipped: on 8-bit music and on one
/// channel of a pure tone the fixed coding wins every block while the fit
/// is estimated to win by a tenth of a bit a sample, and the fit's whole
/// path, the autocorrelation, the Levinson recursion, its residual and its
/// partitions, was half the block.
const LOSSES: u8 = 2;
/// The bits a sample a fit must lose by to count as lost: on the recordings
/// most of the fit's losses are within a sixteenth of a bit a sample and the
/// next block's fit wins, where on 8-bit music the fit loses by a quarter
/// of a bit on every block.
const LOSS: f64 = 1.0 / 8.0;

/// A channel counted in one partition and not yet finished: the fit whose
/// residual is in the trial, and whether it was estimated to win clearly,
/// the fixed predictor counted beside it where it was not.
struct Counted {
    fit: Candidate,
    clear: bool,
}

impl Channel {
    /// A channel of `block` samples, coefficients at `precision` bits.
    pub fn new(block: usize, precision: u32) -> Self {
        Self {
            signal: vec![0; block],
            best: Coding::new(block),
            trial: Coding::new(block),
            scratch: std::array::from_fn(|_| vec![0; block]),
            windowed: vec![0.0; block],
            halves: [vec![0; block], vec![0; block]],
            sums: vec![0; 1 << MAX_PARTITION_ORDER],
            params: vec![0; 1 << MAX_PARTITION_ORDER],
            wasted: 0,
            bps: 0,
            width: 0,
            precision,
            fixed_order: 0,
            coarse: 0,
            weighed: None,
            counted: None,
            coded: false,
            analysed: false,
            bound: None,
            predictable: false,
            fit_declined: false,
            lost: 0,
            skipped: 0,
            est_fit: 0.0,
            est_fixed: 0.0,
            correction: None,
            fit_over: 0.0,
        }
    }

    /// The bits the subframe takes as coded.
    pub fn bits(&self) -> u64 {
        self.best.bits
    }

    /// The bits the subframe would take written as it is.
    pub fn verbatim(&self) -> u64 {
        self.header() + self.signal.len() as u64 * u64::from(self.bps)
    }

    /// The bits the subframe costs in one Rice partition, as
    /// [`Self::count`] counted it, or its bits where it is coded.
    pub fn coarse(&self) -> u64 {
        self.coarse
    }

    /// Take `signal` in, of `bps` bits a sample: shift the wasted bits out,
    /// find the width the samples need, and code a channel that is one
    /// value as a constant; any other is as it is until it is coded. One
    /// pass folds the samples' bits together for the wasted bits, their
    /// differences from the first for the constant, and their magnitudes
    /// for the width, which the compiler vectorises where a test of every
    /// sample against the first, leaving at the first that differs, stayed
    /// scalar. False where a sample does not fit `bps`, which the width
    /// says for nothing; the channel is then not to be coded.
    pub fn prepare(&mut self, bps: u32) -> bool {
        let n = self.signal.len();
        let first = self.signal[0];
        let (mut all, mut differ, mut magnitude) = (0i32, 0i32, 0i32);
        for &s in &self.signal {
            all |= s;
            differ |= s ^ first;
            magnitude |= s ^ (s >> 31);
        }
        // A sample's two's complement is the bits of its magnitude, less
        // one for a negative, which the fold has, and a sign bit.
        if 33 - magnitude.leading_zeros() > bps {
            return false;
        }
        self.wasted = if all == 0 { 0 } else { all.trailing_zeros() };
        if self.wasted > 0 {
            for s in &mut self.signal {
                *s >>= self.wasted;
            }
        }
        self.bps = bps - self.wasted;
        self.width = 33 - (magnitude >> self.wasted).leading_zeros();
        let header = self.header();
        self.weighed = None;
        self.counted = None;
        self.coded = false;
        self.analysed = false;
        self.bound = None;
        self.predictable = false;
        if differ == 0 {
            self.best.kind = Kind::Constant(first >> self.wasted);
            self.best.bits = header + u64::from(self.bps);
            self.coded = true;
            self.analysed = true;
            // No fit is compared on a constant block, so the fit's losses
            // on either side of it are not in a row; a skip under way,
            // which the losses earned, is kept.
            if self.skipped == 0 {
                self.lost = 0;
            }
        } else {
            self.best.kind = Kind::Verbatim;
            self.best.bits = self.verbatim();
            self.predictable = lpc::MAX_ORDER.min(n / 8) > 0;
        }
        self.coarse = self.best.bits;
        true
    }

    /// Code the channel prepared with the fixed predictor of the order that
    /// leaves the smallest residual, where that beats the coding so far,
    /// and count the bits: the start a fit is weighed against and tried on.
    /// Once a block. The passes over the samples are plain loops the
    /// compiler vectorises for whatever a target is built for; compiling
    /// them again for AVX2 and picking that at run time was measured to
    /// gain nothing.
    pub fn analyse_fixed(&mut self) {
        if self.analysed {
            return;
        }
        self.analysed = true;
        if self.signal.len() <= MAX_FIXED_ORDER {
            return;
        }
        self.fixed_order = best_order(&self.signal, &mut self.scratch);
        self.fixed();
    }

    /// The autocorrelation of the channel analysed, through `window`, for
    /// its fit; none where there is nothing to fit.
    pub fn autocorrelation(&mut self, window: &Window) -> Option<Autocorrelation> {
        self.predictable.then(|| lpc::autocorrelation(&self.signal, window, &mut self.windowed))
    }

    /// Weigh the channel prepared for the choice of a pair, from `autoc`,
    /// its autocorrelation or one derived for it, where it has one: the
    /// bits the fit or the fixed predictor, whichever is estimated cheaper,
    /// is estimated to take, with the header. The estimate is kept for
    /// counting or coding the channel ([`Self::count`], [`Self::finish`]).
    /// A channel with no autocorrelation, noise by the skip rule, is counted
    /// at its fixed order, 0 by that rule, and a block too short for a fit is
    /// coded outright: their bits are returned.
    pub fn weigh(&mut self, autoc: Option<&Autocorrelation>) -> f64 {
        if self.coded {
            return self.coarse as f64;
        }
        if !self.predictable {
            self.analyse_fixed();
            self.code();
            return self.coarse as f64;
        }
        let Some(autoc) = autoc else {
            self.coarse = self.measure(self.fixed_order).min(self.best.bits);
            return self.coarse as f64;
        };
        let estimate = self.estimate_from(autoc);
        let bits = self.header() as f64 + estimate.fit.as_ref().map_or(estimate.fixed, |fit| fit.bits().min(estimate.fixed));
        self.weighed = Some(estimate);
        bits
    }

    /// Count the channel weighed in one Rice partition: the fit first, as
    /// [`Self::code_fit_first`] codes it, but for the search over the
    /// partitions, which is left to [`Self::finish`] for the channels
    /// chosen. The fit's residual is taken and its folded sums with it, so
    /// the count is the residual's true bits in one partition, within what
    /// the partitioning gains, where the estimate is off by a bit a sample
    /// either way; where the fit is not clear, the fixed predictor of the
    /// order the autocorrelation chooses is counted the same way beside it.
    pub fn count(&mut self) {
        let Some(estimate) = self.weighed.take() else {
            return;
        };
        let samples = self.signal.len() as f64;
        let Some(fit) = estimate.fit else {
            self.coarse = self.measure(self.fixed_order).min(self.best.bits);
            return;
        };
        let clear = self.correction.is_some_and(|c| fit.bits() + (c + ALONE) * samples < estimate.fixed);
        let Some(bits) = self.count_fit(&fit) else {
            self.coarse = self.measure(self.fixed_order).min(self.best.bits);
            return;
        };
        self.coarse = bits.min(self.best.bits);
        if !clear {
            self.coarse = self.coarse.min(self.measure(self.fixed_order));
        }
        self.counted = Some(Counted { fit, clear });
    }

    /// Code the channel weighed or counted, where it was chosen: a
    /// channel counted has the fit's residual in the trial, whose
    /// partitions are searched, and the fixed path runs where the fit was
    /// not clear or came out past [`SLACK`] over its estimate, as
    /// [`Self::code_fit_first`] runs it, so that the channel's correction
    /// learns what it would have; a channel only weighed is coded from its
    /// estimate as that codes it, and one neither takes the fixed path.
    /// Once a block.
    pub fn finish(&mut self) {
        if self.coded {
            return;
        }
        if let Some(estimate) = self.weighed.take() {
            self.code_estimated(estimate, false);
            return;
        }
        self.coded = true;
        let Some(Counted { fit, clear }) = self.counted.take() else {
            self.analyse_fixed();
            return;
        };
        let bits = self.search_fit(&fit);
        let over = (bits as f64 - fit.bits()) / self.signal.len() as f64;
        if clear && over <= self.fit_over + SLACK {
            self.fit_over = over;
            self.lost = 0;
            if bits < self.best.bits {
                std::mem::swap(&mut self.best, &mut self.trial);
            }
        } else {
            self.analyse_fixed();
            self.settle(bits);
        }
    }

    /// The bits the channel takes coded with the fixed predictor of `order`
    /// in one Rice partition, its residual taken into the scratch of that
    /// order in one pass with the folded sum: a true count, over what the
    /// partitions chosen gain, where the estimate is off by a bit a sample
    /// and more.
    fn measure(&mut self, order: usize) -> u64 {
        let n = self.signal.len();
        let sum = match order {
            0 => folded_sum(&self.signal),
            4 => {
                // Order 4 as order 2 of order 2: one pass of five taps was
                // measured at 0.77 µs over 960 samples, the two passes 0.54.
                let [_, second, _, fourth] = &mut self.scratch;
                fixed_residual(&self.signal, 2, &mut second[..n - 2]);
                fixed_residual(&second[..n - 2], 2, &mut fourth[..n - 4])
            }
            _ => fixed_residual(&self.signal, order, &mut self.scratch[order - 1][..n - order]),
        };
        let (k, rice) = best_param(sum, (n - order) as u64);
        let param = if k > MAX_PARAM_4 { 5 } else { 4 };
        self.header() + order as u64 * u64::from(self.bps) + 2 + 4 + param + rice
    }

    /// The most the channel can cost coded, from its bits where it is
    /// coded, and otherwise from one pass over the samples: the fixed
    /// predictor of order 1 in one Rice partition, which the coding
    /// chosen, of the best order in the best partitions, never exceeds.
    /// For a pair not coded this block to be weighed against the pair that
    /// was, on a block where the other two channels are not coded.
    pub fn at_most(&mut self) -> u64 {
        if self.coded {
            return self.best.bits;
        }
        if let Some(bound) = self.bound {
            return bound;
        }
        let n = self.signal.len();
        let bound = if n <= MAX_FIXED_ORDER {
            self.best.bits
        } else {
            // Twice the magnitude is at least the folded value.
            let sum = difference(&self.signal, &mut self.scratch[0][..n - 1]);
            let (k, rice) = best_param(2 * sum, (n - 1) as u64);
            let param = if k > MAX_PARAM_4 { 5 } else { 4 };
            (self.header() + u64::from(self.bps) + 2 + 4 + param + rice).min(self.best.bits)
        };
        self.bound = Some(bound);
        bound
    }

    /// Code the channel prepared on its own, where no pair is being chosen
    /// and the channel's bits need only be its own: the fit first where
    /// one is wanted, and otherwise the fixed path. With `both`, the fixed
    /// coding is made beside the fit whatever the fit is estimated at, which
    /// brings the channel's correction up to date.
    pub fn code_alone(&mut self, window: &Window, both: bool) {
        if self.fit_wanted()
            && let Some(autoc) = self.autocorrelation(window)
        {
            self.code_fit_first(&autoc, both);
        } else {
            self.analyse_fixed();
            self.code();
        }
    }

    /// Code the channel prepared from its autocorrelation, the fit first,
    /// where no pair is being chosen and the channel's bits need only be
    /// its own. Where the fit is estimated to beat the best fixed predictor
    /// by [`ALONE`] a sample, the fit is coded and the fixed path, the
    /// order search and its partitions, is not run, unless the fit's real
    /// bits are over its estimate by [`SLACK`] a sample, when the fixed
    /// coding is made after all and the two compared. Otherwise the fixed
    /// path runs and the fit is tried against it as [`Self::code`] tries
    /// it. The fixed order the next block's skip rule reads is the one the
    /// autocorrelation chose. With `both`, both codings are made and
    /// compared whatever the fit is estimated at, as the fit is not clear.
    pub fn code_fit_first(&mut self, autoc: &Autocorrelation, both: bool) {
        let estimate = self.estimate_from(autoc);
        self.code_estimated(estimate, both);
    }

    /// The fit's and the fixed predictor's estimates from `autoc`, the fixed
    /// order the autocorrelation's; the channel's fixed order and the skip
    /// rule's state are brought to it.
    fn estimate_from(&mut self, autoc: &Autocorrelation) -> Estimate {
        let n = self.signal.len();
        let estimate = lpc::estimate(autoc, n, lpc::MAX_ORDER.min(n / 8), self.bps, self.precision, None);
        self.fixed_order = estimate.fixed_order;
        // Declined for the skip's purposes where it is not estimated to win:
        // on noise the fit is within the margin above the fixed predictor,
        // tried, and loses, every block.
        self.fit_declined = estimate.fit.as_ref().is_none_or(|fit| fit.bits() >= estimate.fixed);
        self.skipped = 0;
        if let Some(fit) = &estimate.fit {
            self.est_fit = fit.bits();
            self.est_fixed = estimate.fixed;
        }
        estimate
    }

    /// Code the channel from `estimate`, the fit first as
    /// [`Self::code_fit_first`] says, or `both` codings.
    fn code_estimated(&mut self, estimate: Estimate, both: bool) {
        let samples = self.signal.len() as f64;
        self.coded = true;
        let Some(fit) = estimate.fit else {
            self.analyse_fixed();
            return;
        };
        let clear = !both && self.correction.is_some_and(|c| fit.bits() + (c + ALONE) * samples < estimate.fixed);
        if clear && let Some(bits) = self.fit_into_trial(&fit) {
            let over = (bits as f64 - fit.bits()) / samples;
            if over <= self.fit_over + SLACK {
                self.fit_over = over;
                self.lost = 0;
                if bits < self.best.bits {
                    std::mem::swap(&mut self.best, &mut self.trial);
                }
            } else {
                self.analyse_fixed();
                self.settle(bits);
            }
        } else {
            self.analyse_fixed();
            self.try_fit(&fit);
        }
    }

    /// The fit coded as the trial, at `bits`, against the fixed coding in
    /// `best`: the better is kept, and the channel's correction and the
    /// fit's excess over its estimate are brought to this block.
    fn settle(&mut self, bits: u64) {
        let samples = self.signal.len() as f64;
        let over = (bits as f64 - self.est_fit) / samples;
        self.fit_over = over;
        self.correction = Some((over - (self.best.bits as f64 - self.est_fixed) / samples).max(0.0));
        self.lost = if bits as f64 - self.best.bits as f64 >= LOSS * samples { self.lost.saturating_add(1) } else { 0 };
        if bits < self.best.bits {
            std::mem::swap(&mut self.best, &mut self.trial);
        }
    }

    /// Whether a fit is worth weighing on this block: not where there is
    /// nothing to fit, and not where the block is noise by the fixed
    /// predictors, order 0 leaving the smallest residual, and the last fit
    /// weighed on this channel was not estimated to win, which is what the
    /// sound of a quiet desktop is block after block. The fit is tried again
    /// every eighth block, since a tone high enough that the differences do
    /// not shorten it is noise by this test and not to the fit.
    pub fn fit_wanted(&mut self) -> bool {
        if !self.predictable {
            return false;
        }
        if ((self.fixed_order == 0 && self.fit_declined) || self.lost >= LOSSES) && self.skipped < SKIPS {
            self.skipped += 1;
            return false;
        }
        true
    }

    /// The bits every sample had zero at the bottom, shifted out by analysing.
    pub fn wasted(&self) -> u32 {
        self.wasted
    }


    /// The channel analysed is coded as it stands.
    pub fn code(&mut self) {
        self.coded = true;
        self.coarse = self.best.bits;
    }

    /// Try `fit` on the samples and keep it where it writes fewer bits than
    /// the coding so far.
    fn try_fit(&mut self, fit: &Candidate) {
        if let Some(bits) = self.fit_into_trial(fit) {
            self.settle(bits);
        }
    }

    /// `fit` coded on the samples as the trial coding, and its bits; none
    /// where its residual would not fit a sample.
    fn fit_into_trial(&mut self, fit: &Candidate) -> Option<u64> {
        self.count_fit(fit)?;
        Some(self.search_fit(fit))
    }

    /// `fit`'s residual into the trial, folded and summed over the finest
    /// partitions, and the bits it takes in one partition; none where the
    /// residual would not fit a sample. [`Self::search_fit`] finishes it.
    fn count_fit(&mut self, fit: &Candidate) -> Option<u64> {
        let predictor = fit.predictor();
        if !lpc::residual(&self.signal, &predictor, self.width, &mut self.halves, &mut self.trial.residual) {
            return None;
        }
        let total = fold(&mut self.trial, self.signal.len(), &mut self.sums);
        let order = predictor.order as u64;
        let (k, rice) = best_param(total, self.signal.len() as u64 - order);
        let param = if k > MAX_PARAM_4 { 5 } else { 4 };
        Some(self.fit_header(&predictor) + 2 + 4 + param + rice)
    }

    /// The trial's residual, as [`Self::count_fit`] left it, in the
    /// partitions that cost least: the trial is `fit` coded, and its bits.
    fn search_fit(&mut self, fit: &Candidate) -> u64 {
        let predictor = fit.predictor();
        let bits = self.fit_header(&predictor) + search(&mut self.trial, self.signal.len(), &mut self.sums, &mut self.params);
        self.trial.kind = Kind::Lpc(predictor);
        self.trial.bits = bits;
        bits
    }

    /// The bits of an LPC subframe before its residual: the header, the
    /// warm-up samples, the precision, the shift and the coefficients.
    fn fit_header(&self, predictor: &Predictor) -> u64 {
        let order = predictor.order as u64;
        self.header() + order * u64::from(self.bps) + 4 + 5 + order * u64::from(predictor.precision)
    }

    /// The fixed predictor of the order found, its residual as the order
    /// search left it, kept where it beats the coding so far.
    fn fixed(&mut self) {
        let n = self.signal.len();
        let order = self.fixed_order;
        let residual = if order == 0 { &self.signal[..] } else { &self.scratch[order - 1][..n - order] };
        self.best.residual.clear();
        self.best.residual.extend_from_slice(residual);
        let bits = self.header() + (order as u64) * u64::from(self.bps) + partition(&mut self.best, n, &mut self.sums, &mut self.params);
        if bits < self.best.bits {
            self.best.kind = Kind::Fixed { order };
            self.best.bits = bits;
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
            w.rice_codes(&coding.folded[start..(i + 1) * size - order], k);
        }
    }
}

/// Choose the partitioning and the Rice parameters of `coding`'s residual
/// over a block of `n`, and return the bits the residual coding takes.
fn partition(coding: &mut Coding, n: usize, sums: &mut [u64], params: &mut [u8]) -> u64 {
    fold(coding, n, sums);
    search(coding, n, sums, params)
}

/// The finest partitioning of a residual over a block of `n` with `order`
/// warm-up samples: as many partitions as the block's factors of two and
/// [`MIN_PARTITION`] allow, to [`MAX_PARTITION_ORDER`].
fn finest(n: usize, order: usize) -> u32 {
    MAX_PARTITION_ORDER.min(n.trailing_zeros()).min((n / (order + 1).max(MIN_PARTITION)).ilog2())
}

/// Fold `coding`'s residual to unsigned as Rice codes it and sum it over
/// the finest partitions into `sums`, for [`search`]; returns the sum over
/// the whole, which is the one partition's.
fn fold(coding: &mut Coding, n: usize, sums: &mut [u64]) -> u64 {
    let order = n - coding.residual.len();
    // Sized first and written through a slice, which the compiler
    // vectorises where it leaves `extend` scalar.
    coding.folded.resize(coding.residual.len(), 0);
    let mut all = 0u32;
    for (f, &r) in coding.folded.iter_mut().zip(&coding.residual) {
        *f = zigzag(r);
        all |= *f;
    }
    let finest = finest(n, order);
    let parts = 1usize << finest;
    let size = n >> finest;
    // The finest sums in 32-bit lanes where the largest value times the
    // partition fits one, which a 16-bit stream's always does, and in 64-bit
    // lanes where it may not: measured apart over 32 partitions of 30, 0.09
    // against 0.18 µs, the widening to 64 bits costing more than the adds.
    let sum: fn(&[u32]) -> u64 = if u64::from(all) * size as u64 <= u64::from(u32::MAX) { narrow_sum } else { wide_sum };
    for (i, s) in sums[..parts].iter_mut().enumerate() {
        let start = if i == 0 { 0 } else { i * size - order };
        *s = sum(&coding.folded[start..(i + 1) * size - order]);
    }
    sums[..parts].iter().sum()
}

/// Choose the partitioning and the Rice parameters of `coding`'s residual,
/// folded and summed over the finest partitions by [`fold`], and return
/// the bits the residual coding takes.
fn search(coding: &mut Coding, n: usize, sums: &mut [u64], params: &mut [u8]) -> u64 {
    let order = n - coding.residual.len();
    let finest = finest(n, order);
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

/// The sum of `values`, each at most `u32::MAX / values.len()`, in four
/// lanes of 32 bits.
#[inline]
fn narrow_sum(values: &[u32]) -> u64 {
    let (fours, rest) = values.as_chunks::<4>();
    let mut lanes = [0u32; 4];
    for four in fours {
        for (lane, &v) in lanes.iter_mut().zip(four) {
            *lane += v;
        }
    }
    u64::from(lanes.iter().sum::<u32>()) + rest.iter().map(|&v| u64::from(v)).sum::<u64>()
}

/// The sum of `values` whatever they are, in four lanes of 64 bits. Lanes of
/// 32 bits over blocks of 32 were tried first: a partition of a 20 ms block
/// is 30 or 60 samples, which never fills a block of 32, and the sum went
/// scalar.
#[inline]
fn wide_sum(values: &[u32]) -> u64 {
    let (fours, rest) = values.as_chunks::<4>();
    let mut lanes = [0u64; 4];
    for four in fours {
        for (lane, &v) in lanes.iter_mut().zip(four) {
            *lane += u64::from(v);
        }
    }
    lanes.iter().sum::<u64>() + rest.iter().map(|&v| u64::from(v)).sum::<u64>()
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
/// the samples every order predicts, the orders taken from 0 up to the
/// first whose residual is not smaller than the one before's; the lower
/// order on a tie. Each order's residual is the difference of the one
/// before, taken in a pass of its own into `scratch[order - 1]`, which the
/// compiler vectorises, and left there for the order chosen to be coded
/// from. The residual's size over the order falls and then rises, the
/// differences shortening what is smooth and lengthening what is noise:
/// stopped at the first rise, the bytes were the same on every made sound
/// and recording measured, and a block of noise, where order 0 wins, takes
/// one difference pass in place of four.
#[inline]
fn best_order(signal: &[i32], scratch: &mut [Vec<i32>; MAX_FIXED_ORDER]) -> usize {
    let n = signal.len();
    // An order's residual starts `order` samples in, so the samples every
    // order predicts begin that much later in each.
    let mut least = abs_sum(&signal[MAX_FIXED_ORDER..]);
    let mut best = 0;
    let mut before = signal;
    let mut rest = &mut scratch[..];
    for order in 1..=MAX_FIXED_ORDER {
        let (this, after) = rest.split_first_mut().expect("an order's residual");
        let this = &mut this[..n - order];
        let whole = difference(before, this);
        // The sum is of the whole residual; the few samples before the
        // highest order's start come off it.
        let sum = whole - this[..MAX_FIXED_ORDER - order].iter().map(|&v| u64::from(v.unsigned_abs())).sum::<u64>();
        if sum >= least {
            break;
        }
        least = sum;
        best = order;
        before = this;
        rest = after;
    }
    best
}

/// Each value less the one before it, into `out`, one shorter, and the sum
/// of their magnitudes, taken in the same pass in eight lanes over blocks
/// of 64 as [`abs_sum`] takes it.
#[inline]
fn difference(values: &[i32], out: &mut [i32]) -> u64 {
    let n = out.len();
    let (blocks, rest) = out.as_chunks_mut::<64>();
    let mut total = 0u64;
    for (k, block) in blocks.iter_mut().enumerate() {
        let from = &values[k * 64..k * 64 + 65];
        let mut lanes = [0u32; 8];
        for (eight, w) in block.as_chunks_mut::<8>().0.iter_mut().zip(from.windows(9).step_by(8)) {
            for (j, (o, lane)) in eight.iter_mut().zip(lanes.iter_mut()).enumerate() {
                let d = w[j + 1].wrapping_sub(w[j]);
                *o = d;
                *lane += d.unsigned_abs();
            }
        }
        total += lanes.iter().map(|&lane| u64::from(lane)).sum::<u64>();
    }
    let done = n - rest.len();
    for ((o, &after), &before) in rest.iter_mut().zip(&values[done + 1..]).zip(&values[done..]) {
        let d = after.wrapping_sub(before);
        *o = d;
        total += u64::from(d.unsigned_abs());
    }
    total
}

/// The sum of `values` folded as Rice codes them, in eight lanes over
/// blocks of 32: four of the 29 bits a folded residual can have fit a lane.
#[inline]
fn folded_sum(values: &[i32]) -> u64 {
    let (blocks, rest) = values.as_chunks::<32>();
    let mut total = 0u64;
    for block in blocks {
        let mut lanes = [0u32; 8];
        for eight in block.as_chunks::<8>().0 {
            for (lane, &v) in lanes.iter_mut().zip(eight) {
                *lane += zigzag(v);
            }
        }
        total += lanes.iter().map(|&lane| u64::from(lane)).sum::<u64>();
    }
    total + rest.iter().map(|&v| u64::from(zigzag(v))).sum::<u64>()
}

/// The residual of the fixed predictor of `order`, 1 to 3, over `signal`
/// into `out`, `order` shorter, in one pass with the sum of the residual
/// folded as Rice codes it, taken as [`folded_sum`] takes it: 0.22, 0.27
/// and 0.33 µs over 960 samples. Order 4 is order 2 taken twice.
fn fixed_residual(signal: &[i32], order: usize, out: &mut [i32]) -> u64 {
    match order {
        1 => fixed_pass::<1>(signal, out),
        2 => fixed_pass::<2>(signal, out),
        3 => fixed_pass::<3>(signal, out),
        _ => unreachable!("a fixed order of 1 to 3"),
    }
}

#[inline]
fn fixed_pass<const ORDER: usize>(signal: &[i32], out: &mut [i32]) -> u64 {
    // The binomial taps of the orders, the oldest sample first.
    let residual = |w: &[i32]| -> i32 {
        match ORDER {
            1 => w[1].wrapping_sub(w[0]),
            2 => w[2].wrapping_sub(w[1].wrapping_mul(2)).wrapping_add(w[0]),
            _ => w[3].wrapping_sub(w[2].wrapping_mul(3)).wrapping_add(w[1].wrapping_mul(3)).wrapping_sub(w[0]),
        }
    };
    let n = out.len();
    let (blocks, rest) = out.as_chunks_mut::<32>();
    let mut total = 0u64;
    for (k, block) in blocks.iter_mut().enumerate() {
        let from = &signal[k * 32..k * 32 + 32 + ORDER];
        let mut lanes = [0u32; 8];
        for (eight, w) in block.as_chunks_mut::<8>().0.iter_mut().zip(from.windows(8 + ORDER).step_by(8)) {
            for (j, (o, lane)) in eight.iter_mut().zip(lanes.iter_mut()).enumerate() {
                let r = residual(&w[j..j + ORDER + 1]);
                *o = r;
                *lane += zigzag(r);
            }
        }
        total += lanes.iter().map(|&lane| u64::from(lane)).sum::<u64>();
    }
    let done = n - rest.len();
    for (o, w) in rest.iter_mut().zip(signal[done..].windows(ORDER + 1)) {
        let r = residual(w);
        *o = r;
        total += u64::from(zigzag(r));
    }
    total
}

/// The Rice parameter for `count` residuals whose zigzag values sum to `sum`,
/// and the bits they take with it, estimated from the sum alone: the parameter
/// near the mean's logarithm, one either side of it tried, whichever is
/// cheapest, none past what a 5-bit field holds. No division: the
/// logarithms' difference is within one of the mean's. The cost is convex
/// in the parameter, and a walk from the difference by exact cost
/// differences, a comparison of two quotients a step in place of a
/// multiplication a trial, found the same parameter on every sound and
/// took 1.01 to 1.05 of the time a block on arm64, so the three trials
/// stay.
#[inline]
fn best_param(sum: u64, count: u64) -> (u32, u64) {
    if sum < count {
        return (0, count + sum);
    }
    let near = (sum.ilog2() - count.ilog2()).min(MAX_PARAM_5);
    let cost = |k: u32| count * (u64::from(k) + 1) + (sum >> k);
    let mut best = (near, cost(near));
    for k in [near.saturating_sub(1), (near + 1).min(MAX_PARAM_5)] {
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
        let mut scratch: [Vec<i32>; MAX_FIXED_ORDER] = std::array::from_fn(|_| vec![0; 64]);
        let ramp: Vec<i32> = (0..64).collect();
        assert_eq!(best_order(&ramp, &mut scratch), 2, "a ramp has a zero second difference");
        let parabola: Vec<i32> = (0..64).map(|i| i * i).collect();
        assert_eq!(best_order(&parabola, &mut scratch), 3);
        let noise: Vec<i32> = (0..64).map(|i| ((i * 7919) % 201) - 100).collect();
        assert_eq!(best_order(&noise, &mut scratch), 0, "noise is left as it is");
    }

    /// Each order's residual is left where the order search put it: what
    /// the fixed polynomial of that order misses, from its `order`th sample.
    #[test]
    fn a_residual_is_what_the_predictor_misses() {
        let mut scratch: [Vec<i32>; MAX_FIXED_ORDER] = std::array::from_fn(|_| vec![0; 8]);
        let squares = [1, 4, 9, 16, 25, 36, 49, 64];
        assert_eq!(best_order(&squares, &mut scratch), 3, "a parabola has a zero third difference");
        assert_eq!(scratch[0][..7], [3, 5, 7, 9, 11, 13, 15]);
        assert_eq!(scratch[1][..6], [2, 2, 2, 2, 2, 2]);
        assert_eq!(scratch[2][..5], [0, 0, 0, 0, 0]);
        assert_eq!(scratch[3][..4], [0, 0, 0, 0]);
    }

    /// The parameter found is the one of least cost by the estimate, as
    /// trying every parameter finds it, on sums from a residual at the
    /// floor to a wide sample's wild one, and the bits are the cost at it.
    #[test]
    fn the_parameter_tracks_the_mean() {
        assert_eq!(best_param(0, 100), (0, 100));
        // A mean of 8 wants a parameter near 3.
        let (k, _) = best_param(800, 100);
        assert!((2..=3).contains(&k), "{k}");
        // A mean folded value past 2^31, which only a wide sample's wild
        // predictor makes, still gets a parameter the 5-bit field holds.
        let (k, _) = best_param(u64::from(u32::MAX) * 100, 100);
        assert_eq!(k, MAX_PARAM_5);
        let (k, _) = best_param(100 << 20, 100);
        assert!((19..=20).contains(&k), "{k}");
        let cost = |sum: u64, count: u64, k: u32| count * (u64::from(k) + 1) + (sum >> k);
        let mut seed = 0x9E37_79B9u64;
        for count in [1u64, 15, 30, 60, 240, 960, 1920] {
            for e in 0..40 {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let sum = (seed % (count * 3 + 1)) << e;
                let (k, bits) = best_param(sum, count);
                assert_eq!(bits, cost(sum, count, k));
                let least = (0..=MAX_PARAM_5).map(|k| cost(sum, count, k)).min().unwrap();
                assert_eq!(bits, least, "sum {sum} count {count}: parameter {k}");
            }
        }
    }

    #[test]
    fn wasted_bits_are_shifted_out() {
        let mut ch = Channel::new(64, 10);
        for (i, s) in ch.signal.iter_mut().enumerate() {
            *s = ((i as i32 % 7) - 3) << 8;
        }
        assert!(ch.prepare(16));
        ch.analyse_fixed();
        assert_eq!(ch.wasted, 8);
        assert_eq!(ch.bps, 8);
        assert!(ch.signal.iter().all(|&s| (-3..=3).contains(&s)));
    }

    /// A sample outside the width is caught by the fold that finds the
    /// width, at either edge, where the two edge values themselves pass.
    #[test]
    fn a_sample_outside_the_width_is_caught_by_preparing() {
        let mut ch = Channel::new(64, 10);
        for (i, s) in ch.signal.iter_mut().enumerate() {
            *s = (i as i32 % 7) - 3;
        }
        for (value, fits) in [(1 << 15, false), ((1 << 15) - 1, true), (-1 << 15, true), ((-1 << 15) - 1, false), (i32::MIN, false), (i32::MAX, false)] {
            ch.signal[5] = value;
            assert_eq!(ch.prepare(16), fits, "{value}");
        }
    }

    /// A tone the fixed predictors leave much of is nearly all predicted by
    /// the fitted one, which is kept; noise is left as it was. Weighed, a
    /// channel's estimate is under its fixed coding's bits by about what the
    /// fit saves, and the fixed order counted in one partition is never
    /// under the fixed coding chosen and within a tenth of it.
    #[test]
    fn the_fit_is_kept_only_where_it_wins() {
        let n = 960;
        let window = Window::new(n);
        let two_tones = |ch: &mut Channel| {
            for (i, s) in ch.signal.iter_mut().enumerate() {
                *s = (12000.0 * (i as f64 * 0.35).sin() + 6000.0 * (i as f64 * 0.9).sin()).round() as i32;
            }
        };
        let count = |ch: &mut Channel| {
            let autoc = ch.autocorrelation(&window).expect("long enough to fit");
            ch.weigh(Some(&autoc));
            ch.count();
        };
        // Coded without the fit, for the fixed bits.
        let mut ch = Channel::new(n, 10);
        two_tones(&mut ch);
        ch.prepare(16);
        ch.analyse_fixed();
        ch.code();
        let fixed = ch.best.bits;
        assert!(matches!(ch.best.kind, Kind::Fixed { .. }), "{:?}", ch.best.kind);
        let measured = ch.measure(ch.fixed_order);
        assert!(measured >= fixed && measured * 10 <= fixed * 11, "measured {measured} against fixed {fixed}");

        let mut ch = Channel::new(n, 10);
        two_tones(&mut ch);
        ch.prepare(16);
        count(&mut ch);
        assert!(ch.counted.is_some());
        assert!(ch.coarse() < fixed, "counted {} against fixed {fixed}", ch.coarse());
        ch.finish();
        assert!(matches!(ch.best.kind, Kind::Lpc(_)), "{:?}", ch.best.kind);
        assert!(ch.best.bits * 5 < fixed * 4, "fitted {} against fixed {fixed}", ch.best.bits);
        assert!(ch.best.bits <= ch.coarse() && ch.coarse() * 10 <= ch.best.bits * 11, "counted {} against fitted {}", ch.coarse(), ch.best.bits);
        assert_eq!(ch.best.residual.len() + ch.best.kind.order(), n);

        let mut ch = Channel::new(n, 10);
        let mut state = 0x9E37_79B9u32;
        for s in &mut ch.signal {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            *s = (state >> 21) as i32 - 1024;
        }
        ch.prepare(16);
        count(&mut ch);
        ch.finish();
        assert!(matches!(ch.best.kind, Kind::Fixed { order: 0 }), "{:?}", ch.best.kind);
        let measured = ch.measure(0);
        assert!(measured >= ch.best.bits && measured * 10 <= ch.best.bits * 11, "measured {measured} against fixed {}", ch.best.bits);
    }
}
