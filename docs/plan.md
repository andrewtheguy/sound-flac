# The plan of the encoder, and what was decided

What the encoder is for, what is decided about it, and what is taken up
next, in which order and why. What it is now is in the module docs of
`src/lib.rs`, `src/subframe.rs` and `src/lpc.rs`; the tables it is measured
by are in [benchmarks.md](benchmarks.md). Written on 2026-10-22, on the
first measurement over recordings, at commit e3173f7 of branch
`own-encoder`; brought to the run of the fit in double precision the same
day, to the night of 2026-10-23, when its six items were measured, to the
day after, when the time against libFLAC's level 0 was taken up, and to
that afternoon, when it was taken up again: the fixed path left out where
the fit clearly wins, and the autocorrelation at the floor; and to the end
of that day, when the fourth block was weighed on a count in place of its
four fixed codings.

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
and the day the level-0 column was taken up twice and the fourth block once,
made again that night with the bench coding every block into an empty
buffer, the encoder unchanged: 0.34 to 0.58 of libFLAC's time a block on
the recordings for its bytes within 0.2%, the late blocks 0.32 to 0.56 of
its; the made sounds 0.27 to 0.55 for the same bytes within two bytes, the
pure tone 1.02 at 44.1 kHz and 1.05 at 24 bits. A block of a recording is
0.04% to 0.24% of one core.
Where the time goes at 16-bit stereo, on the orchestra, by the parts
measured apart: a carried block, three in four, 17.6 µs, is two
autocorrelations 2.8, their fits 0.6, the pair's residuals 2.2 and
partitions 1.6, the two fixed codings 3.8 where the fit is not clear, the
folds and bounds 1.2, the range check and the stereo split 1.6, the bits
out 3.4 and the CRC 0.5, and on two of the three one channel has its fixed
coding made whatever the fit is estimated at, 1.9, to bring its correction
up to date; the fourth block, 22.5 µs, has three autocorrelations, four
fits and four one-partition counts in their place, and is the late block,
by a quarter to a third of a carried one where it was by half before. On
arm64, the same day, the recordings are 0.31 to 0.48 of libFLAC's time for
the same bytes to a tenth of a byte, and against level 0 0.71 to 1.23 where
x86 is 0.61 to 1.33.

## Against libFLAC's level 0

The tables' one red column was the time against libFLAC's fastest level,
which fits no predictor and codes stereo as it comes. The two are not doing
the same work: level 0's block of silence alone costs 12.6 µs where this
encoder's costs 3, and the ratio goes under 1.00 on sound only where this
encoder's own work, the fit and the two codings, comes under what level 0
spends on its two fixed codings and its frame. What was measured to bring
it down is under the measurements below. The first day took the time bought
with bytes, a byte a block, by fitting only the carried pair between full
choices; the afternoon after took the rest with the bytes within 0.02%:
the 16-bit stereo recordings went from 1.21 to 1.79 of level 0's time to
0.81 to 1.04, the orchestra and the Fantasy still over by 0.01 and 0.04,
the mono reading from 0.83 to 0.53, and the recordings at 24 bits from 1.46
to 1.79 to 0.74 to 1.28; the made stereo music and tone from 1.25 to 1.89
to 0.81 to 1.11. The end of the day took the fourth block down and with it
the means by up to 8%, the 16-bit stereo recordings to 0.80 to 1.01, the
orchestra now under at 0.99, the 1962 speech to 0.65 and the four at 24
bits to 1.07 to 1.19, in a run where libFLAC's own times were within 3% of
the run before's. The tables were made again that night, the encoder
unchanged and the bench clearing its buffer before every block: libFLAC's
level 0 came out 5% to 17% faster on x86 and its level 5 1% to 13%, this
encoder 1% to 10%, so the column there reads 0.85 to 1.11 on the 16-bit
stereo recordings, 0.70 on the 1962 speech, 0.61 on the mono reading and
1.16 to 1.33 on the four at 24 bits, while on arm64, where no time moved
past 3%, it reads as before, 0.71 to 1.23 on the recordings. What is left
over 1.00 is the wide sample
and the long block, where the fit's own passes are the most of the block,
and the tone, whose fourth block fits two channels for nothing.

## Open items, in order

1. **The partitions' search.** A coding's fold is 0.14 µs and its finest
   sums 0.09 in 32-bit lanes, and the search over the partition orders 0.4:
   63 Rice parameters each tried at three values from the logarithms'
   difference. Dropping the lowest trial cost 1% to 4% of the bytes, and
   stopping the search at the first rise from the coarsest order cost
   0.07% on the orchestra, two rises nothing in bytes and nothing in time
   (below). What is left is the search's own arithmetic.
2. **The writer**, about 3.4 µs a block for two channels, four Rice codes
   a word joined as pairs, with the accumulator in locals. Measured apart
   on 1920 codes, every grouping tried comes to 3.65 to 4.1 µs and one code
   at a time 4.3 to 5.0: the work is each code's arithmetic, two or three
   shifts by a variable count at three µops each on the x86-64 baseline,
   which has no single-µop variable shift. Taking the quotients, remainders
   and lengths of four codes as arrays, which the compiler shifts in one
   vector, gained nothing over the pairs joined as a tree; eight codes a
   word lost.
3. **The tone's coefficients.** A pure tone's frame is its coefficients'
   rounding, 1.02 of libFLAC's bytes at 44.1 kHz and 1.05 at 24 bits, where
   libFLAC chooses the pair on every channel's real bits. The rounding's
   error as order times the mean square over twelve times four to the shift
   was measured both in the order choice and in the pairing's gain alone,
   and cost the Raven 1.3% and 1.6% and the pianos 0.5% to 0.7%: libFLAC's
   quantization carries each coefficient's rounding into the next, so on
   smooth sound the errors cancel and the model overstates them. A model of
   the carried error is the item, if a tone is worth it.
4. **The fresh block**, every fourth, still the late block: over a carried
   block it has the third autocorrelation, two more fits and the four
   one-partition counts, about 3 µs at 960 samples and 6 at 1920 (below,
   the fresh block weighed). What would take that is a fit from fewer
   lags, or the side's autocorrelation from the left's and the right's,
   which needs their cross-correlation, one more pass of the same cost.

## Measured and taken

- **The fresh block weighed on a one-partition count** (2026-10-23, late
  afternoon, item 4 of the plan as it stood): on the block that chooses the
  pair afresh, the four fixed codings, each an order search and a partition
  search at 1.9 µs, are replaced by a count: the fixed order the
  autocorrelation chooses has its residual taken and folded in one pass,
  0.22 to 0.54 µs by the order, and counted in one Rice partition, a true
  count of the channel's bits within the partitioning's gain, and the fit's
  estimated saving comes off it as it came off the fixed coding's real bits
  before. The two channels chosen are then coded the fit first as a carried
  block codes them. Coded the fit first on every block, a channel's
  correction went stale, both codings being made only where the fit was
  not clear, and the orchestra cost 0.41% more bytes; so each channel of the
  pair has both coded once a cycle whatever the estimate, the first on the
  first carried block and the second on the next, where the cost of 1.9 µs
  falls on blocks that are not the fresh one. A block with a constant channel, the
  desktop playing the same sound on both channels or on one, took the
  full choice on every block before; it now takes the carried path like
  any other, the constant coded by the check and the pair moved to it by
  the bound, which took the made speech, whose two channels are one
  signal, from 1.16 to 1.37 of its time to 0.75 to 0.88, and the 1962
  speech to 0.86, for 0.05% of the made speech's bytes and 0.16% at 8
  bits, the fit-first cost its exact path never paid. Measured in the
  tables' run against the commit before, by the block's place in the
  cycle: the fourth block went from 27.1 to 22.5 µs on the orchestra, 22.2
  to 18.3 on the Raven, 21.2 to 16.7 on the wide mix and 57.9 to 45.1 on
  the Aria, against a carried block of 17.6, 14.0, 13.9 and 35.4; the two
  blocks after the choice rose by about 1 µs at 16 bits and 4 on the Aria;
  the means fell by up to 8% on the stereo music on both machines and stood
  within 2% on the stereo speech and the mixes, and the bytes were 1.0000
  to 1.0001 of before on every recording. Both
  codings on the fourth block instead of the carried ones left the fourth
  block where it was, 26.4 on the orchestra; refreshing every other cycle
  cost the orchestra 0.02% for 2.5% of the mean and left the late block the
  same, and was not taken. Where two channels are one signal the fourth
  block codes the three equivalent channels in turn, each coming out over
  its estimate as the fit's saving is overstated; taking the channel's
  correction off its saving when weighing cost the Raven 0.05% and the
  Aria 0.09%, and taking another pair after the coding only past an eighth
  of a bit a sample the Aria 0.1%, so the pair is chosen again as
  estimated and that block stays, 19 µs against 12 on the 1962 speech. The
  count without its residual stored measured no faster than with it, and
  order 4 in one pass of five taps 0.77 µs against 0.54 as order 2 twice,
  which is how it is taken.
- **The fit first on a carried block, the fixed path left out where the
  fit clearly wins** (2026-10-23, the level-0 ask taken up again): on the
  three blocks in four between full choices, the carried pair's two
  channels are each coded from their autocorrelation alone, and the two
  other channels are not looked at past a one-pass constant check. The
  fit's estimate and the best fixed predictor's are taken from the one
  autocorrelation, with the fixed order chosen from it too; where the fit
  is estimated to win by an eighth of a bit a sample after the channel's
  correction, it is coded and the order search and the fixed partitions
  are not run, and otherwise both are coded and compared as before. The
  correction is what the channel's last block with both codings showed: the
  estimates are from the windowed samples and blind to the coefficients'
  rounding, so the fit's advantage is overstated, by 0.2 bits a sample and
  more on three blocks in five of the orchestra at 16 bits and by about
  none on the Aria at 24, and a fit coded alone whose real bits run a
  quarter of a bit a sample past what its estimate and the channel's last
  fit said has the fixed coding made after all. The margin was swept over
  0, 1/16, 1/8 and 1/4 a sample on eight recordings: at 0 the bytes were
  1.0002 to 1.0008 of before on the orchestra, the Raven and the mix, at
  1/8 at most 1.0002 and at 1/4 at most 1.0001, the mono reading taking
  0.01% at 1/8 and nothing at 1/4, for 3% to 8% more time at 1/4 than at
  1/8 on the reading and the train. 1/8 is taken. Another pair is still
  taken on a carried block where it is certainly cheaper: a channel not
  coded is bounded from one pass over its first differences, the fixed
  predictor of order 1 in one Rice partition, which what it would be coded
  at never exceeds, and the pair moves where two bounds come under the pair
  coded; without it the wide stereo mix was 1.0006, with it 1.0001, for no
  measurable time. A channel on its own, mono and more than two, is coded
  the fit first on every block, there being no pair to choose. Against the
  tables of the morning, the time a block on the eight recordings fell to
  0.60 to 0.81 and the bytes rose by 0.00% to 0.02%, the autocorrelation
  below counted in.
- **The autocorrelation in one pass without a check** (the same day, the
  item 4 below closed): the two passes of five and four lags indexed each
  pair of samples and checked it, two bounds checks and three shuffles on
  every two samples, 24 µops where the floating-point work was 8, and ran
  front-end bound at 2.4 µs measured apart and 2.8 in the encoder for 8640
  multiply-adds whose floor at the baseline's two floating-point ports is
  4320 cycles, 1.4 µs at this machine's 3.1 GHz. Seen through a window of
  the two samples and the eight after them, stepped by two, every lag's
  pair is in bounds by the window's length and the compiler lays the nine
  lags out two to a register, a sample broadcast against the pairs after
  it: 1.37 µs, the floor, and the same bytes to the bit. Four lanes a lag
  were 1.46 to 1.68 in the same form, and three passes of three lags 1.7
  to 2.1: once the checks are gone the floating-point work is the whole
  cost and lanes do not change it. The one pass is not inlined, so that
  its nine sums have the registers to themselves whatever the caller holds,
  which is where the one pass lost before. The recordings fell by 4% to
  13% and the made sounds by 5% to 15%.
- **The frame's CRC as two halves** (the same day): one chain of lookups
  eight bytes at a time was its loads' latency, 0.77 µs on a 2300-byte
  frame; the two halves of the frame as independent chains, the first
  carried past the second's length by multiplication modulo the polynomial
  with a table of the sixteen powers, 0.47. Slicing by sixteen in one chain
  was 0.62. **The Rice word joined as pairs**, the two pairs of codes joined
  and then the two, a chain of two variable shifts rather than three, 3.65
  µs on 1920 codes against 3.83. **The constant check as a fold**: every
  sample against the first, leaving at the first that differs, is a loop
  the compiler cannot vectorise, and it went scalar over 960 samples on
  four channels a block; the samples' differences from the first are now
  or-folded in the one pass that folds their bits for the wasted bits and
  their magnitudes for the width the samples need, by which a side channel
  that is 17 bits wide by the stream and 16 in its samples takes the 16-bit
  residual pass, 1.1 µs against the split path's 1.6.
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
  eighth blocks where the fit is tried again. Nothing to take. That was
  read when every block chose its pair; since the carried pair, the
  encoder's own late block is the fourth, at 1.5 times its mean, and the
  bench read it right only from that afternoon, when each pass was given
  an encoder that had seen no block (the morning's bench ran one encoder
  on through the passes, and on a recording whose blocks are not a
  multiple of four the fourth block moved by one each pass, so a block's
  least over the passes was its cheaper kind).
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

- **The partitions' search stopped early** (2026-10-23, afternoon): the
  sums folded into a pyramid first and the orders walked from the coarsest,
  stopping at the first order that did not beat the best: 3% to 12% of a
  block, for 0.07% more bytes on the orchestra, 0.02% on the made speech
  and 0.01% on the Aria and the rock master, the bits over the orders not
  falling and rising once. Stopped at the second rise, the bytes were the
  same but for one part in ten thousand on the orchestra, and the time was
  the same within the noise: the pyramid and the walk from the coarse end
  cost what the fine orders skipped had cost. Out.
- **Four lanes a lag in the autocorrelation**, taken up again on the
  user's ask: in the checked two-pass form it was 3.7 µs against 2.4,
  and in the window form without checks 1.46 to 1.68 against two lanes'
  1.37 in one pass and 1.6 in two; eight lanes in three passes 2.4. The
  lanes were never the cost; the checks and the shuffles were. Out, the
  one pass of two lanes taken above.
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
- **The nine lags in one pass, inlined**: 2.1 µs against 2.3 measured
  apart, a tenth slower in the encoder on every sound that fits, the same
  bytes; the pass spilled its sums inside the encoder's own register use.
  Taken that afternoon, not inlined and without the bounds checks,
  as above.
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
