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
//! [`SKIPS`] blocks, as a channel of noise has, and one whose fit has won
//! it by [`WIN`] a sample [`WINS`] blocks running has the fixed path left
//! out for [`SKIPS`] blocks, the fit coded alone whatever the margin says,
//! its correction kept, but for the blocks the pair's schedule has both
//! coded on. A channel not coded on a block is bounded from
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
//! partition are 0.25 on the i5 and 0.21 on the EPYC; the bound 0.3; the fixed predictor's one-partition
//! count 0.3 at order 0,
//! 0.22, 0.27 and 0.33 at orders 1 to 3 and 0.54 at order 4, taken as
//! order 2 twice where one pass of five taps took 0.77; writing a block's
//! two channels is about 3.4 µs at 16 bits and 6 at 24, the Rice codes
//! four, three or two to a word by the parameter. Measured apart at
//! 960 samples: the four fixed orders in one pass, with the binomial taps
//! as differences of differences, did not vectorise and took 4.3 µs
//! against the four passes' 1.1, and the sums alone then the chosen order
//! 1.2, so the four passes stay; the fold and the finest sums are one pass
//! over the finest partitions, on the EPYC 0.21 µs at 960 against 0.31 as
//! a pass of the fold and then a call a partition over what it wrote, 0.41
//! against 0.61 at 1920 and 0.11 against 0.21 over the two partitions of
//! 882, the sums in 32-bit lanes where a bound on the folded value from
//! the sample's width and the predictor's taps says a partition's fits
//! one, which a 16-bit stream's always does, and in 64-bit lanes, 0.26 at
//! 960, where it may not; the search over the orders 0.4 from the finest down, and walked from
//! the coarsest over a pyramid of the sums and stopped three orders past
//! the best, 2% to 4% fewer instructions on the 48 and 96 kHz recordings
//! and 4% to 7% off a made 48 kHz block, for 0.03% of the 1924 record's
//! bytes and a tenth of a byte a block on the 8-bit speech, where a block
//! of 882 at 44.1 kHz has two partitionings and nothing to skip; the one-partition count without its
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
    /// A fit's residual, `signal.len() - order` of them; a fixed
    /// predictor's is folded from the order search's scratch.
    residual: Vec<i32>,
    /// The residual folded to unsigned, as Rice codes it.
    folded: Vec<u32>,
    /// The Rice parameter of each partition, at `partition_order`.
    chosen: Vec<u8>,
    partition_order: u32,
    /// Whether a parameter takes 5 bits rather than 4.
    wide_params: bool,
    /// The bits the subframe takes, as the sums estimate them less the
    /// fractions the estimate drops ([`best_param`]), within a part in a
    /// thousand of the real bits on sound, which never exceed them by more
    /// than half a bit a sample; exact for a constant or a verbatim
    /// subframe.
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
    /// Sums of a folded residual over the finest partitions, and after them
    /// over each coarser partitioning, the pyramid [`search`] walks.
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
    /// The blocks in a row the fit, coded beside the fixed coding, has
    /// beaten it by [`WIN`] a sample on real bits; a constant block ends the
    /// row as it ends `lost`'s, unless the fixed path is being left out on
    /// the row's account.
    won: u8,
    /// The blocks in a row the fit has been coded alone, the fixed path not
    /// run; none once both are coded.
    fit_alone: u8,
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

/// The blocks in a row a fit is skipped on before it is tried again, and
/// the blocks in a row the fixed path is left out for before it is run
/// beside the fit again.
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
/// The blocks in a row a fit must beat the fixed coding by [`WIN`] a sample,
/// both coded, for the fixed path to be left out, the mirror of [`LOSSES`]:
/// on the side channel of a pure tone at 48 kHz the fit wins every block by
/// 0.12 bits a sample, under the margin a fit is coded alone on from its
/// estimate, so both codings were made on every block, the fixed path a
/// seventh of the tone's block, and on an orchestra one carried channel in
/// three had both made by that margin.
const WINS: u8 = 2;
/// The bits a sample a fit must beat the fixed coding by to count as won:
/// half of [`LOSS`], since the tone's margin is just under an eighth, and
/// the fixed path left out costs only where the fixed coding would have
/// won, which two wins by a sixteenth make rare.
const WIN: f64 = 1.0 / 16.0;

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
            sums: vec![0; 2 << MAX_PARTITION_ORDER],
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
            won: 0,
            fit_alone: 0,
            est_fit: 0.0,
            est_fixed: 0.0,
            correction: None,
            fit_over: 0.0,
        }
    }

    /// The bits the subframe takes as coded, estimated within a part in a
    /// thousand on sound and never under the real bits by more than half a
    /// bit a sample; [`Self::at_most`] is what they never exceed.
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
            // or wins on either side of it are not in a row; a skip under
            // way, which the losses or the wins earned, is kept.
            if self.skipped == 0 {
                self.lost = 0;
            }
            if self.fit_alone == 0 {
                self.won = 0;
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
        let Some(fit) = estimate.fit else {
            self.coarse = self.measure(self.fixed_order).min(self.best.bits);
            return;
        };
        let clear = self.fit_clear(&fit, estimate.fixed);
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
            self.keep_fit(bits, over);
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

    /// The most the channel can cost coded: where it is coded, its bits
    /// and half a bit a sample, the most the estimate's fractions can be
    /// under the real bits, and otherwise a bound from one pass over the
    /// samples, the fixed predictor of order 1 in one Rice partition, which
    /// the coding chosen, of the best order in the best partitions, never
    /// exceeds but for the estimate's own error. For a pair not coded this
    /// block to be weighed against the pair that was, on a block where the
    /// other two channels are not coded, and for the frame to be held under
    /// its PCM.
    pub fn at_most(&mut self) -> u64 {
        if self.coded {
            return self.best.bits + (self.signal.len() as u64 >> 1);
        }
        if let Some(bound) = self.bound {
            return bound;
        }
        let n = self.signal.len();
        let bound = if n <= MAX_FIXED_ORDER {
            self.best.bits
        } else {
            // Twice the magnitude is at least the folded value. The bound is
            // on the scale the pair's bits are, the fractions off, so that
            // it moves the pair as often as it did: on the plain estimate
            // against corrected bits it moved it less, and the wide mix
            // cost 0.03%.
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
        let clear = !both && self.fit_clear(&fit, estimate.fixed);
        if clear && let Some(bits) = self.fit_into_trial(&fit) {
            let over = (bits as f64 - fit.bits()) / samples;
            if over <= self.fit_over + SLACK {
                self.keep_fit(bits, over);
            } else {
                self.analyse_fixed();
                self.settle(bits);
            }
        } else {
            self.analyse_fixed();
            self.try_fit(&fit);
        }
    }

    /// Whether `fit`, estimated at `fixed` for the fixed predictor, is to
    /// be coded alone, the fixed path not run: where it is estimated to win
    /// by [`ALONE`] a sample after the channel's correction, or where it
    /// has beaten the fixed coding by [`WIN`] a sample [`WINS`] blocks
    /// running and has been coded alone fewer than [`SKIPS`] blocks since.
    fn fit_clear(&self, fit: &Candidate, fixed: f64) -> bool {
        let samples = self.signal.len() as f64;
        self.correction.is_some_and(|c| fit.bits() + (c + ALONE) * samples < fixed) || (self.won >= WINS && self.fit_alone < SKIPS)
    }

    /// The fit coded alone as the trial, at `bits`, `over` its estimate a
    /// sample, kept where it beats the coding so far.
    fn keep_fit(&mut self, bits: u64, over: f64) {
        self.fit_over = over;
        self.lost = 0;
        self.fit_alone = self.fit_alone.saturating_add(1);
        if bits < self.best.bits {
            std::mem::swap(&mut self.best, &mut self.trial);
        }
    }

    /// The fit coded as the trial, at `bits`, against the fixed coding in
    /// `best`: the better is kept, and the channel's correction, the fit's
    /// excess over its estimate and its runs of losses and wins are brought
    /// to this block.
    fn settle(&mut self, bits: u64) {
        let samples = self.signal.len() as f64;
        let over = (bits as f64 - self.est_fit) / samples;
        self.fit_over = over;
        self.correction = Some((over - (self.best.bits as f64 - self.est_fixed) / samples).max(0.0));
        let fit_less = self.best.bits as f64 - bits as f64;
        self.lost = if -fit_less >= LOSS * samples { self.lost.saturating_add(1) } else { 0 };
        self.won = if fit_less >= WIN * samples { self.won.saturating_add(1) } else { 0 };
        self.fit_alone = 0;
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
        // A folded value is at most twice the residual, which is at most
        // the sample and the prediction, each of the width: the sample
        // once, the prediction the coefficients' magnitudes over the
        // shift, rounded up, and one.
        let coefs: u64 = predictor.coefs[..predictor.order].iter().map(|&c| u64::from(c.unsigned_abs())).sum();
        let most = (((coefs >> predictor.shift) + 2) << self.width) + 2;
        let n = self.signal.len();
        let total = fold(&self.trial.residual, predictor.order, n, most, &mut self.trial.folded, &mut self.sums);
        let order = predictor.order as u64;
        let (k, rice) = best_param(total, n as u64 - order);
        let param = if k > MAX_PARAM_4 { 5 } else { 4 };
        Some(self.fit_header(&predictor) + 2 + 4 + param + rice)
    }

    /// The trial's residual, as [`Self::count_fit`] left it, in the
    /// partitions that cost least: the trial is `fit` coded, and its bits.
    fn search_fit(&mut self, fit: &Candidate) -> u64 {
        let predictor = fit.predictor();
        let bits = self.fit_header(&predictor) + search(&mut self.trial, self.signal.len(), predictor.order, &mut self.sums, &mut self.params);
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

    /// The fixed predictor of the order found, its residual folded from
    /// where the order search left it, kept where it beats the coding so
    /// far.
    fn fixed(&mut self) {
        let n = self.signal.len();
        let order = self.fixed_order;
        let residual = if order == 0 { &self.signal[..] } else { &self.scratch[order - 1][..n - order] };
        // A folded value is at most twice the residual, which is at most
        // the sample's width and the order's binomial taps, two to the
        // order in all.
        let most = 1u64 << (self.width + order as u32);
        let header = self.header() + (order as u64) * u64::from(self.bps);
        let bits = header + partition(residual, order, n, most, &mut self.best, &mut self.sums, &mut self.params);
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

/// Choose the partitioning and the Rice parameters of `coding` over
/// `residual`, the `order` samples of a block of `n` short, and return the
/// bits the residual coding takes, as [`search`] returns them; `most` is
/// as [`fold`] takes it.
fn partition(residual: &[i32], order: usize, n: usize, most: u64, coding: &mut Coding, sums: &mut [u64], params: &mut [u8]) -> u64 {
    fold(residual, order, n, most, &mut coding.folded, sums);
    search(coding, n, order, sums, params)
}

/// The finest partitioning of a residual over a block of `n` with `order`
/// warm-up samples: as many partitions as the block's factors of two and
/// [`MIN_PARTITION`] allow, to [`MAX_PARTITION_ORDER`].
fn finest(n: usize, order: usize) -> u32 {
    MAX_PARTITION_ORDER.min(n.trailing_zeros()).min((n / (order + 1).max(MIN_PARTITION)).ilog2())
}

/// Fold `residual`, the `order` samples of a block of `n` short, to
/// unsigned as Rice codes it, into `folded`, and sum it over the finest
/// partitions into `sums`, for [`search`], a partition at a time in the one
/// pass, the sums in four lanes of 32 bits where `most`, a bound on a
/// folded value, times the partition fits one, which a 16-bit stream's
/// always does, and in lanes of 64 bits otherwise; returns the sum over the
/// whole, which is the one partition's. The fold was a pass of its own
/// before, and the sums a call a partition over what it wrote, 32 calls of
/// 30 values at 960: on the EPYC, over 960 samples, 0.21 µs against 0.31,
/// over 1920 0.41 against 0.61 and over the two partitions of 882, 0.11
/// against 0.21; the 64-bit lanes 0.26, 0.51 and 0.15. The lanes chosen
/// on the values' own bits, or-folded as they are folded, took 0.25 over
/// 960; the bound in hand costs nothing.
fn fold(residual: &[i32], order: usize, n: usize, most: u64, folded: &mut Vec<u32>, sums: &mut [u64]) -> u64 {
    // Sized first and written through slices, which the compiler
    // vectorises where it leaves `extend` scalar.
    folded.resize(residual.len(), 0);
    let finest = finest(n, order);
    let parts = 1usize << finest;
    let size = n >> finest;
    if most * size as u64 <= u64::from(u32::MAX) {
        fold_parts::<u32>(residual, order, size, folded, &mut sums[..parts])
    } else {
        fold_parts::<u64>(residual, order, size, folded, &mut sums[..parts])
    }
}

/// A lane the finest sums are taken in, of 32 or 64 bits.
trait Lane: Copy + Default + std::ops::AddAssign {
    fn lane(folded: u32) -> Self;
    fn whole(self) -> u64;
}

impl Lane for u32 {
    fn lane(folded: u32) -> Self {
        folded
    }
    fn whole(self) -> u64 {
        u64::from(self)
    }
}

impl Lane for u64 {
    fn lane(folded: u32) -> Self {
        u64::from(folded)
    }
    fn whole(self) -> u64 {
        self
    }
}

/// [`fold`] in lanes of `L`, a partition of `size` into each of `sums`, the
/// first `order` short.
#[inline]
fn fold_parts<L: Lane>(residual: &[i32], order: usize, size: usize, folded: &mut [u32], sums: &mut [u64]) -> u64 {
    for (i, s) in sums.iter_mut().enumerate() {
        let start = if i == 0 { 0 } else { i * size - order };
        let end = (i + 1) * size - order;
        let (fours, rest) = folded[start..end].as_chunks_mut::<4>();
        let (from_fours, from_rest) = residual[start..end].as_chunks::<4>();
        let mut lanes = [L::default(); 4];
        for (four, from) in fours.iter_mut().zip(from_fours) {
            for j in 0..4 {
                let f = zigzag(from[j]);
                four[j] = f;
                lanes[j] += L::lane(f);
            }
        }
        let mut total = L::default();
        for (o, &r) in rest.iter_mut().zip(from_rest) {
            let f = zigzag(r);
            *o = f;
            total += L::lane(f);
        }
        for lane in lanes {
            total += lane;
        }
        *s = total.whole();
    }
    sums.iter().sum()
}

/// The partitionings tried past the best so far before the search stops,
/// from the coarsest: the bits over the partition orders mostly fall and
/// then rise, and the finest orders are the costliest to count, half the
/// search at the finest alone. Measured on the EPYC against the whole
/// search: stopped one order past the best, 4% to 6% fewer instructions on
/// the 48 and 96 kHz recordings for 0.26% more bytes on the 1924 record,
/// 0.06% on the orchestra and 0.5% on the made 8-bit speech, whose bits
/// over the orders fall again past a rise; two past, 3% to 5% for 0.10%
/// and 0.3%; three past, 2% to 4% and 4% to 7% of a made 48 kHz block for
/// 0.03% on the 1924 record and a tenth of a byte a block on the 8-bit
/// speech, every other line the same, which is within the bar.
const RISES: u32 = 3;

/// Choose the partitioning and the Rice parameters of `coding`'s residual,
/// the `order` samples of a block of `n` short, folded and summed over the
/// finest partitions by [`fold`], and return the bits the residual coding
/// takes, estimated from the sums less the fractions the estimate drops
/// ([`best_param`]), which the partitioning is chosen on as each parameter
/// is: on the 8-bit noise the estimate with the fractions took two to
/// eight partitions on the side channel for a gain that was the
/// fractions', where libFLAC took one. The finest sums are folded into the
/// coarser partitionings' first, a pyramid in `sums` after the finest, and
/// the orders are walked from one partition up, stopping [`RISES`] orders
/// past the best.
fn search(coding: &mut Coding, n: usize, order: usize, sums: &mut [u64], params: &mut [u8]) -> u64 {
    let finest = finest(n, order);
    let parts = 1usize << finest;
    // The sums of partitioning `p` start at `at(p)`: the finest's where the
    // fold wrote them, each coarser one's after the one before.
    let at = |p: u32| if p == finest { 0 } else { 2 * parts - (2usize << p) };
    for p in (0..finest).rev() {
        let (from, to) = (at(p + 1), at(p));
        for i in 0..1usize << p {
            sums[to + i] = sums[from + 2 * i] + sums[from + 2 * i + 1];
        }
    }
    let mut best = (u64::MAX, 0, false);
    let mut rises = 0;
    for p in 0..=finest {
        let parts = 1usize << p;
        let size = n >> p;
        let base = at(p);
        let mut total = 0;
        let mut wide = false;
        for (i, &sum) in sums[base..base + parts].iter().enumerate() {
            let count = if i == 0 { size - order } else { size };
            let (k, bits) = best_param(sum, count as u64);
            wide |= k > MAX_PARAM_4;
            total += bits;
            params[i] = k as u8;
        }
        total += 2 + 4 + (parts as u64) * if wide { 5 } else { 4 };
        if total < best.0 {
            best = (total, p, wide);
            coding.chosen[..parts].copy_from_slice(&params[..parts]);
            rises = 0;
        } else {
            rises += 1;
            if rises >= RISES {
                break;
            }
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
/// and the bits they take with it: the estimate from the sum,
/// `count * (k + 1) + (sum >> k)`, which the real bits never exceed, less the fractions
/// it drops, which the real bits never exceed by more than half a bit a
/// sample. The parameter is the one near the mean's logarithm or one
/// either side of it, whichever is cheapest by that, none past what a
/// 5-bit field holds. No division: the
/// logarithms' difference is within one of the mean's. The quotients' bits
/// are the sum of each value shifted down, under the sum shifted down by
/// the fractions dropped, about half a bit a sample less half a bit over
/// two to the parameter, `(1 - 2^-k) / 2` a value: the same at every parameter a
/// loud residual is in reach of, and on a quiet one, at a parameter of 1 to
/// 3, enough to favour the smaller parameter and the finer partitioning
/// wrongly, which on 8-bit noise was a parameter under libFLAC's on the mid
/// channel and 0.46% of the bytes, and on the 96 kHz tone 2.3%. The half
/// bit a sample is the same at every parameter and is left out of the
/// weighing. The three weights come from one quotient, one share of the
/// count and one multiplication, the neighbours' by constant shifts, two
/// variable shifts where the three costs took three; over the 63
/// partitions of a 960-frame block the correction is 0.7% of the
/// orchestra's instructions and 0.9% of the Fantasy's, within the bench's
/// noise. The step from the difference by the costs' convexity, a quotient
/// and two comparisons, was slower on the EPYC, `search` going from seven
/// per cent of a block to eleven, since the machine runs the independent
/// weights four instructions a cycle and a chain not, as a walk had lost on
/// arm64 before.
#[inline]
fn best_param(sum: u64, count: u64) -> (u32, u64) {
    if sum < count {
        return (0, count + sum);
    }
    let near = (sum.ilog2() - count.ilog2()).min(MAX_PARAM_5);
    let half = count >> 1;
    if near == 0 {
        // A mean under two: parameter 0 or 1.
        let (w0, w1) = (count + sum + half, 2 * count + (sum >> 1) + (count >> 2));
        return if w1 < w0 { (1, w1 - half) } else { (0, count + sum) };
    }
    // The quotient one step down, this one and the one up; the count's
    // share at each, `count >> (k + 1)`; and the weights, the plain cost
    // with the share, near first, then down and up on a strictly smaller
    // weight, as the trials went.
    let q_down = sum >> (near - 1);
    let q = q_down >> 1;
    let share_down = count >> near;
    let m = count * u64::from(near);
    let w_down = m + q_down + share_down;
    let w = m + count + q + (share_down >> 1);
    let w_up = if near < MAX_PARAM_5 { m + 2 * count + (q >> 1) + (share_down >> 2) } else { u64::MAX };
    let (k, w) = if w_down < w { (near - 1, w_down) } else { (near, w) };
    let (k, w) = if w_up < w { (near + 1, w_up) } else { (k, w) };
    (k, w - half)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bits the estimate from the sum has over the quotients' real
    /// bits at parameter `k` where the values' low bits are spread evenly:
    /// the mean fraction dropped, `(1 - 2^-k) / 2`, over `count` values,
    /// which is none at 0, where nothing is dropped.
    fn floor_slack(count: u64, k: u32) -> u64 {
        (count >> 1) - (count >> (k + 1))
    }

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

    /// The parameter found is the one of least cost by the corrected
    /// estimate, as trying every parameter finds it, on sums from a
    /// residual at the floor to a wide sample's wild one, and the bits are
    /// that estimate at it; the plain estimate is never under the real bits
    /// of values whose low bits are anything, the corrected one is within a
    /// part in a thousand of them on values spread evenly and never under
    /// them by more than half a bit a sample.
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
        let corrected = |sum: u64, count: u64, k: u32| cost(sum, count, k) - floor_slack(count, k);
        let mut seed = 0x9E37_79B9u64;
        for count in [1u64, 15, 30, 60, 240, 960, 1920] {
            for e in 0..40 {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let sum = (seed % (count * 3 + 1)) << e;
                let (k, less) = best_param(sum, count);
                assert_eq!(less, corrected(sum, count, k));
                let least = (0..=MAX_PARAM_5).map(|k| corrected(sum, count, k)).min().unwrap();
                assert_eq!(less, least, "sum {sum} count {count}: parameter {k}");
            }
        }
        // Values spread evenly over 0..2^12 and the same values all even,
        // all a multiple of eight: the plain estimate is over the real bits
        // of each at every parameter, and the corrected one is within a part
        // in a thousand of the evenly spread values' real bits.
        let real = |values: &[u32], k: u32| values.iter().map(|&u| u64::from(u >> k) + 1 + u64::from(k)).sum::<u64>();
        let spread: Vec<u32> = (0..960u32).map(|i| i.wrapping_mul(2_654_435_761).wrapping_mul(i | 1) >> 20).collect();
        for shift in [0, 1, 3] {
            let values: Vec<u32> = spread.iter().map(|&u| (u >> shift) << shift).collect();
            let sum = values.iter().map(|&u| u64::from(u)).sum::<u64>();
            for k in 0..=14 {
                assert!(cost(sum, 960, k) >= real(&values, k), "shift {shift} parameter {k}");
            }
            let (k, less) = best_param(sum, 960);
            let bits = real(&values, k);
            assert!(bits <= less + 480, "shift {shift}: real {bits} over the corrected {less} by more than half a bit a sample");
            if shift == 0 {
                assert!(less.abs_diff(bits) * 1000 <= bits, "corrected {less} against real {bits} at {k}");
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
        assert_eq!(ch.best.folded.len() + ch.best.kind.order(), n);

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
