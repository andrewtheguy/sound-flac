# The plan of the encoder, and what was decided

What the encoder is for, what is decided about it, and what is taken up
next, in which order and why. What it is now is in the module docs of
`src/lib.rs`, `src/subframe.rs` and `src/lpc.rs`; the tables it is measured
by are in [benchmarks.md](benchmarks.md). Written on 2026-10-22, on the
first measurement over recordings, at commit e3173f7 of branch
`own-encoder`; brought to the run of the fit in double precision the same
day, to the night of 2026-10-23, when its six items were measured, and to
the day after, when the time against libFLAC's level 0 was taken up.

## What it is for

The wlshare daemon's and the zDesk gateway's, which stream a desktop's
sound as it is made: one FLAC frame of every 20 ms block, coded the moment
the block is whole and sent. That is the one use, and it sets what counts.

- **A frame stands alone.** Every frame is numbered zero and codes its block
  and nothing before it, so a listener takes each as it comes and a frame
  lost on the way costs its own 20 ms and nothing after. Nothing in a frame
  may depend on another frame; what the encoder remembers between blocks is
  its own business, and the decoder never sees it.
- **The block is given.** The users choose it, rate/50, so 20 ms, and the
  encoder codes what it is handed: no lookahead, no waiting for a second
  block, no choosing a block size. A sample that has arrived is 20 ms from
  being heard at the most, before the link.
- **The cost that matters is one block's time, the late block more than the
  mean.** The sound is paced, so the encoder is idle 99% of the time
  whatever it does; what it must not do is take long on one block, since a
  late frame is late sound. A block is at most 2% of its 20 ms today
  ([benchmarks.md](benchmarks.md), the block times). The share of a core is
  what the mean says, 0.04% to 0.35% on the recordings, and it is a
  desktop's share beside its video encoder.
- **Bytes are the link's.** A frame's bytes are its bitrate, 30% to 50% of
  the PCM on acoustic music and speech, 69% on a loud modern master at
  16 bits and 79% at 24, 75% to 95% on noise and old transfers. Fewer
  is better everywhere, and worth more than time: the time is already not
  felt.
- **The shapes are the users'.** wlshare codes 8-bit and 16-bit at 8 to
  96 kHz, zDesk 16-bit at 44.1 kHz; the crate takes 4 to 24 bits, 1 to 8
  channels. The tables cover 44.1, 48 and 96 kHz, mono and stereo, 8, 16
  and 24 bits.

## Scope, final

- **Encode only, in plain Rust, no `unsafe`, no dependency beyond
  `thiserror`.** No decoder, no C, no SIMD intrinsics: the lanes the
  compiler finds at the x86-64 baseline are the lanes there are, and
  `target-cpu` is the users' build decision, not the crate's.
- **What a frame may hold**: a constant, verbatim or fixed subframe, or a
  linear predictor of up to order 8 at a precision libFLAC's rule gives the
  shape, Rice partitions to order 8 as far as the block's factors of two
  allow (order 6 at 960 frames, 1 at 882, 7 at 1920), with the 5-bit
  parameters where a 4-bit one does not fit, wasted bits shifted out, and
  the four stereo pairings. That
  is the FLAC subset libFLAC's default level writes, and every decoder reads.
- **Nothing between frames that a decoder would have to know.** The encoder
  may carry what it learned from the block before, a predictor or a
  pairing to try first, but never a dependency.
- **Not taken, and why**: threads (one block is 10 to 80 µs; handing it to
  another core costs more latency than it saves and takes a core the video
  has); lookahead or variable blocks (latency); libFLAC's exhaustive search
  of level 8 (2.6 to 3.6 times the time for 0 to 2% fewer bytes); orders
  above 8 or precision search (level 8's gain, which is that small); a
  Rice parameter per sample's history or any adaptive coding FLAC has not
  (a frame must be plain FLAC).

## How it is measured

Every change to the tuning is measured before it is taken, on 20 ms blocks,
against what it replaces and against libFLAC at level 5, in time a block and
bytes a block, on both sets of [benchmarks.md](benchmarks.md): the made
sounds, for what a single thing costs, and the recordings, for what a stream
costs. For the recordings the 99th-percentile and worst blocks are read
beside the mean, since they are the latency. A change is taken where it
buys bytes for little time or time for no bytes, and the tables and this
plan are brought to the run.

## Where it stands

From the tables of 2026-10-23, after the night the items below were measured
and the day the level-0 column was taken up: 0.44 to 0.77 of libFLAC's time
a block on the recordings for its bytes within 0.2%, the late blocks 0.45 to
0.84 of its; the made sounds 0.40 to 0.74 for the same bytes within a byte,
the pure tone 1.02 at 44.1 kHz and 1.05 at 24 bits. A block of a recording
is 0.05% to 0.32% of one core. Where the time went at 16-bit stereo, on the
orchestra, with timers after the night, 30 µs: three autocorrelations 8.3 µs
and the fits from them 1.2, the four channels' fixed orders and sums 4.5 and
their partitions 3.7, the two fits' residuals 3.3 and partitions 2, the
range check and the stereo split 1.6, the bits out 4.1, the CRC 1. The day
took the autocorrelations to two on three blocks in four, the bits out to
3.6 and the finest sums to half: 27 µs.

## Open items, in order

1. **The partitions' search.** Six codings a block; a coding's fold is
   0.14 µs and its finest sums 0.09 in 32-bit lanes, and the search over
   the partition orders 0.4: 63 Rice parameters each tried at three values
   from the logarithms' difference. Dropping the lowest trial cost 1% to 4%
   of the bytes, so the three stay; what is left is the search's own
   arithmetic, or fewer partition orders where the coarse ones have lost.
2. **The writer**, 3.6 µs a block for two channels, four Rice codes a word
   with the accumulator in locals. The work is the codes' arithmetic, not
   the word's flush: a branchless writer that stored eight bytes on every
   word and advanced by the whole bytes was slower, 2.1 µs a channel against
   1.7 measured apart.
3. **The tone's coefficients.** A pure tone's frame is its coefficients'
   rounding, 1.02 of libFLAC's bytes at 44.1 kHz and 1.05 at 24 bits, where
   libFLAC chooses the pair on every channel's real bits. The rounding's
   error as order times the mean square over twelve times four to the shift
   was measured both in the order choice and in the pairing's gain alone,
   and cost the Raven 1.3% and 1.6% and the pianos 0.5% to 0.7%: libFLAC's
   quantization carries each coefficient's rounding into the next, so on
   smooth sound the errors cancel and the model overstates them. A model of
   the carried error is the item, if a tone is worth it.
4. **The autocorrelation**, 2.8 µs a channel at 960 samples in two passes of
   two double lanes, is near the x86-64 baseline's floor for its 8640
   multiply-adds: measured apart, one pass of all nine lags was 2.1 against
   2.3, four lanes a lag 3.7 and eight 2.9, but in the encoder the one pass
   was a tenth slower on every sound that fits, so the two passes stay. The
   window's multiply fused into the first pass, or AVX2 where a user builds
   for it, is what is left.

## Against libFLAC's level 0

The tables' one red column is the time against libFLAC's fastest level,
which fits no predictor and codes stereo as it comes: 1.1 to 2.1 of its time
on stereo music and tone, for 0.78 to 0.96 of its bytes on music and 0.49
to 0.78 on speech. The two are not doing the same work. Of this encoder's
30 µs on the orchestra, the fit and the stereo choice are about 15, and the
fixed path on four channels, the split, the writer and the CRC the other
15; level 0 runs the fixed path on two channels, and this encoder's would be
about 9 µs there, under level 0's 18, of which 12.6 is its frame's fixed
cost (its block of silence). The ratio goes under 1.00 only by dropping the
fit, which is the bytes. What was tried to bring it down without dropping
the fit is under the measurements below: carrying the predictor cost bytes,
fitting only the carried pair between full choices was taken, and the items
that cost no bytes were taken or measured out.

## Measured and taken

- **Only the carried pair fitted between full choices** (2026-10-23, the
  level-0 ask): every fourth block all four channels are fitted and the
  pair chosen on their estimates as before; on the blocks between, only
  the pair the last block was coded with is fitted, the other two channels
  stand on their fixed bits, and another pair is taken where its fixed
  codings alone are cheaper. One autocorrelation and two fits fewer a
  block, 12% to 15% of the time on 16-bit stereo recordings, for 0.00% to
  0.09% of the bytes: the Raven 1.0007, the prison song 1.0009, the
  orchestra 1.0002, the Fantasy 1.0001, the train 1.0001, a byte a block at
  the most. The bytes were the same within 0.05% chosen afresh every
  eighth or sixteenth block, so it is not the period but the blocks between
  that cost, where the pair cannot move to one whose fit would win. Giving
  the two channels not fitted an estimate from their last coding's saving
  over its fixed coding, and fitting a pair that estimate favoured before
  choosing it, bought no byte back and cost most of the time saved. This
  is time bought with bytes, a byte a block, which the rule above does not
  do on its own; it was taken on the ask to bring the level-0 column down
  and is undone by setting `RECHOOSE` to 0, every block then choosing afresh.
- **The writer's accumulator in locals**: the Rice codes of a partition go
  through one call that holds the accumulator and its free bits in locals
  through the loop, 1.7 µs a channel against 1.9 measured apart, the same
  bytes. **The finest partition sums in 32-bit lanes** where the largest
  folded value times the partition fits one, which a 16-bit stream's always
  does, 0.09 µs a coding against 0.18 in 64-bit lanes, the fold's or-fold
  giving the bound for nothing. Together 2% to 5% of a block against the
  control.
- **The pair chosen on the fit** (item 1 of the plan of 2026-10-22, and
  item 5 with it): every channel is coded with its fixed predictor first,
  its residual being what the order search left; its fit is weighed from
  its autocorrelation and the bits it is estimated to save come off its
  real fixed bits; the pair is chosen on that, coded, and chosen again with
  the coded channels' real bits until it stands. The train station went
  from 1.006 of libFLAC's bytes to 1.000, the Fantasy from 1.004 to 1.002,
  the prison song and the Raven to 1.000; nothing rose. On the way: chosen
  on the estimates alone, with libFLAC's estimate clamped at none a sample,
  the 8-bit speech was 1.63, every quiet channel looking free beside a
  constant side; with a floor of a bit a sample fitted to the Laplacian the
  speech came back but the pure tones went 1.05 to 1.10, all channels of a
  tone tying at the floor where the real cost is the rounding of their
  coefficients, which goes with amplitude; real bits less the estimated
  gain settled both, and re-choosing on real bits gave the 48 kHz tone
  libFLAC's 1008 back where its fits lose on the samples. The mid's
  autocorrelation is derived from left's, right's and side's, scaled for
  each one's wasted bits, so three passes serve four channels for no byte;
  where a channel is one value the pair stands on the fixed bits and only
  the pair is fitted, 23 to 16 µs on the made speech.
- **The fit skipped on noise** (item 2): where the last fit weighed on a
  channel was not estimated to win and this block's fixed order is 0, the
  fit is skipped, and tried again every eighth block, since a tone high
  enough that the differences do not shorten it is noise by this test.
  White noise fell from 37 to 22 µs a block and the made noise from 30 to
  17, for no byte. The margin's "declined" does not serve: on noise the fit
  is within the margin above the fixed predictor, tried, and loses, every
  block. Carrying the block before's predictor into this block as a start
  was not measured, the skip taking the whole of what it could save.
- **The late blocks** (item 3): the bench logs the block that was slowest
  at its least over the passes and dumps every block's least time and
  bytes. Read that way, this encoder's own 99th-percentile block is within
  a quarter of its mean on every recording and its worst 1.2 to 1.7 times
  it, where a single pass's 99th percentile is 1.5 to 2.3 times the mean
  for both encoders: the spread the tables show is the machine's. The
  encoder's own late blocks are the loud ones, and on white noise the
  eighth blocks where the fit is tried again. Nothing to take.
- **The 24-bit residual** (item 4), and the 17-bit side channel with it:
  a sample of 17 to 24 bits is split into its upper 16 bits and its lower
  bits, each part's sum taken in 16-bit lanes as the 16-bit path takes
  them, and the two combined exactly under the shift. The 32-bit lanes the
  side channel ran in have no multiply at the baseline and took four times
  the 16-bit path's time, 13% of a 16-bit stereo block; the 24-bit samples
  ran in 64-bit scalars. The Goldberg Aria fell from 84 to 66 µs a block,
  the 78 rpm transfer from 86 to 70, made 24-bit music from 43 to 32.
- **The time of a block, where the timers pointed**: the fixed residual
  was computed twice, once as the order search's differences and once to
  code, 3 µs a block, now once; the range check folded over a pair and the
  stereo split indexed four arrays, scalar both, 3.75 µs, now two folds and
  one zipped pass, 1.6; the finest partitions' sums ran in 32-bit lanes
  over blocks of 32 that a partition of 30 or 60 samples never fills, so
  scalar, now 64-bit lanes over fours; the writer took two Rice codes a
  word, now four, with no mask on a value made to fit, 6.5 µs to 4.1.
- **The fit in double precision** (item 1 of the first plan): the
  autocorrelation in `f32` lanes misfit the Goldberg Aria to 1.011 of
  libFLAC's bytes, the Fantasy to 1.010, the Raven to 1.006 and the made
  tone at 96 kHz to 1.07; exact sums give 1.002, 1.004, 1.001 and 1.00.
  Measured on the way, per channel at 960 samples against `f32`'s 1.1 µs:
  one pass of eight `f64` lanes +2.5 µs (+5 at 1920, the accumulators
  spilling); `f32` lanes flushed to `f64` every 64, 128 or 256 samples +0.6
  to +1.0 for a part of the precision, the flush breaking the loop's
  codegen more than its own cost; two passes of two `f64` lanes, five lags
  then four, +1.7 (+3 at 1920), taken. Taking the mean out first was not
  measured, exact sums making it moot. A white-noise correction of the zero
  lag, tried for the tone, cost bytes on every recording from one part in
  ten million and moved the tone's bytes either way, so none is applied.

## Measured and not taken

- **The predictor carried between blocks** (2026-10-23): each channel kept
  the quantized predictor it was last coded with where a fit won, the block
  tried the carried pair with its carried predictors on its samples first,
  and fitted afresh where a carried predictor lost, where another pair's
  fixed bits alone were cheaper, and every Nth block. Refitted every eighth
  block: 0.70 to 0.83 of the time on the recordings for 0.1% to 1.7% more
  bytes, the Leviticus reading 1.7%, the Raven 1.6%, the Fantasy 1.0%; every
  fourth, still 0.9% on the Raven and the reading. The staleness is the
  coefficients': speech and piano move within 20 ms. Adaptive triggers in
  place of the cadence, refitting where the fixed coding's bits moved by
  more than a fraction since the block before or where the carried
  predictor's saving fell by more than a fraction from the fresh fit's,
  were swept over 2% to 10% and 5% to 30%: the Fantasy still paid 0.7% to
  1.6%, the Raven 0.1% to 0.4%, and the orchestra and the 24-bit speech
  never took the fast path (a side channel without a predictor, the saving
  moving block to block) and got slower from the recalls thrown away, 1.07
  to 1.13. Out: bytes for time at a rate the rule does not take, and the
  late block no better, since the refit block costs the whole.
- **The four fixed orders in one pass** (item 1 of the night's plan): the
  binomial taps as differences of differences over a window of five, all
  four residuals written and summed in the pass. It did not vectorise, 4.3
  µs a channel against the four passes' 1.1 measured apart; the sums alone
  in one pass and then the chosen order's residual, 1.2. The four passes
  stay.
- **A branchless writer**, eight bytes stored on every word and the pointer
  advanced by the whole bytes: 2.1 µs a channel against 1.9 for the
  current and 1.7 with the accumulator in locals. The branch on the word's
  edge is not the cost. **Pairs a word** with locals, 2.2.
- **The nine lags in one pass**: 2.1 µs against 2.3 measured apart, a tenth
  slower in the encoder on every sound that fits, the same bytes. Out.
- **The tone's order** (item 6): the next order tied with the fit at the
  floor, tried on the samples beside it, changed no tone's byte at any
  shape. The tone's loss is in the pairing, not the order. Out.
- **The coefficients' rounding in the estimate**, as order times the mean
  square over twelve times four to the shift: in the order choice it cost
  the Raven 1.3%, the Fantasy 0.7% and the orchestra 0.5%; in the pairing's
  gain alone the Raven 1.6%, and the 24-bit tone went to 1.09. Out, as said
  under the open items.
- **The lowest Rice parameter trial dropped**, the parameter from the
  logarithms' difference and the one above tried alone: 1% to 4% more
  bytes on every sound but silence, for no time. The three trials stay.
- **The estimate clamped at none a sample**, libFLAC's, as the pairing's
  measure: the 8-bit speech at 1.63. The floor of a bit a sample stays, and
  the pairing stands on real bits.
- **AVX2 by runtime dispatch** on the fixed path: nothing gained at the
  baseline's lanes, and it needs `unsafe`. Out.
- **Fitting the tone when the fixed predictor's estimate is better**: the
  Levinson error estimate predicts the Rice bits to ±0.04 bit a sample at
  16 bits, and skipping the fit on that estimate changed no byte at any
  margin from none to never-skip. The skip stays.
- **flacenc**, the pure-Rust encoder tried first: 250 µs a block at its
  default, 75 fixed only, against libFLAC's 46 and this encoder's 28 before
  the fit went to double.
