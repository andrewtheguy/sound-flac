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
four fixed codings; and to 2026-10-24, when the rows still on libFLAC's
side of the line were taken up in the order of how far over they were: the
tone's bytes and time, then the time against level 0 at 24 bits and 96 kHz;
and to the evening of 2026-10-23 by the machine's clock, when the fresh
block was taken up and the pair came to be chosen on the four channels'
counts; and to the morning of 2026-10-24, when the x86 run moved from the
desktop to an idle machine and a fifteenth recording was added; and to
that afternoon, when the open items were planned again from the EPYC's
own profile, the cells still over libFLAC first, with the arm64 run held
until the round is called ready, and to that afternoon's end, when the
first of them, the Rice parameter's estimate, was taken.
The days before that are the count the sessions kept, which ran ahead of
the machine's clock: by the commits, which that clock stamps in Pacific
time, all of it ran from the evening of 2026-10-22 to the afternoon of
2026-10-23, the night of 2026-10-23 here being noon of that day and
2026-10-24 its afternoon; the evening of 2026-10-23 is the clock's, and
follows them.

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
plan are brought to the run. From the afternoon of 2026-10-24 a round of
changes is measured on the EPYC alone, run by run, and the arm64 run is
made once at the round's end, when the user calls it ready, so that the
arm64 box, which builds other things, is not slowed by a run a change;
its section of the tables is of that run.

## Where it stands

From the tables of the morning of 2026-10-24, the encoder of v0.0.8
unchanged, the x86 run moved from the desktop i5 under WSL2, which was
never quiet, to an idle KVM machine on an AMD EPYC Rome, and a
fifteenth recording added, one voice talking over soft music: 0.24 to
0.47 of libFLAC's time a block on the recordings for its bytes within
0.2%, the late blocks 0.20 to 0.58 of its; the made sounds 0.05 to 0.44
for the same bytes within two bytes, the pure tone under libFLAC's
bytes, 0.90 to 1.00. A block of a recording is 0.02% to 0.15% of one
core. The move is the machine's, not the encoder's: on the EPYC this
encoder's block is 0.64 to 0.68 of its i5 block on every recording and
libFLAC's default level 0.74 to 0.84 of its on the stereo recordings,
its fastest level 0.80 to 0.96, so every ratio reads a fifth or a
quarter better than v0.0.8's run on the i5 for the same work, and the
bytes are that run's on every line of every coder. The machine repeats
a time within a twentieth, two runs with different compilers agreeing
within 6% on every line, where the i5 moved by a tenth. On arm64, where
nothing moved and the run is the evening before's within 2% on every
line, the recordings are 0.25 to 0.42 of libFLAC's time; this encoder's
block is 1.24 to 1.46 of its EPYC block there, libFLAC's default level
1.30 to 1.61 on the stereo recordings and its level 0 0.84 to 1.20. Where
the time goes at 16-bit stereo, on the orchestra, by the parts measured
apart on the i5, whose carried block was 16.9 µs where the EPYC's is
10.9: a carried block, three in four, is the two
autocorrelations with their windowing 3.6 µs, their fits 0.7, the pair's
residuals 2.3 and partitions 1.9, the folds and bounds 1.2, the stereo
split 0.7, the bits out 4.4 and the CRC 0.7, about 16 in all where the fit
is clear on both channels, and a fixed coding of 1.7 for each channel whose
fit is not, which on the orchestra is two carried channels in three, as on
two of the three one channel has its fixed coding made whatever the fit is
estimated at, to bring its correction up to date: about 18 on the mean
carried block, 17.1 timed whole; the fourth block, 21.9 µs, has three
autocorrelations, four fits and, where the estimates do not settle the
pair, the two other channels' residuals and folds in their place, and is
the late block, by a fifth to a third of a carried one. At 24 bits the fit's residual
runs as two 16-bit passes at twice the cost, 10 µs of the Aria's 39 on
the i5, and the writer takes 6 µs of pink noise's 24. On arm64 the
recordings are 0.25 to 0.42 of libFLAC's time for the same bytes to a
tenth of a byte, and against level 0 0.90 to 1.12 on the 16-bit stereo
recordings where the EPYC is 0.58 to 0.87 and under 1.00 on every
recording. The Windows run, which was made on demand on the desktop as a
quieter reading of the same machine, is retired with the move: the
desktop is no longer where the x86 run is made.

Profiled on the EPYC the afternoon after the run, with perf's cpu-clock at
10 kHz over this encoder alone, the bench built with line tables and the
blocks 15% longer under the sampler, the parts of a block as shares of the
encoder's own samples: on the orchestra the two autocorrelations with
their windowing are 19%, the fits from them 6%, the pair's residuals 14%,
the partitions, the fold, the finest sums and the search over the orders,
20%, the fixed order search and the two bounds 9%, the Rice codes out 17%,
the frame's CRC 5%, the stereo split with the header 4% and the four
channels' preparing 4%. On the Raven and the talk at 44.1 kHz the
partitions are 6% to 7%, a block of 882 frames having one factor of two
and two partitionings to try where 960 has six, and the autocorrelation
is 22%, the residuals 16% to 18%, the writer 19% to 20%. On the Fantasy
at 96 kHz the residuals are 18% and the partitions 17%; on the Aria at 24
bits the residual, two 16-bit passes, 27%, the writer 16% and the CRC 7%;
on the 1924 record the residual 22%, the writer 21% and the CRC 10%; on
pink noise the writer 23%, the partitions 17%, the CRC 14% and the
autocorrelation 11%; on 24-bit noise the writer 29% and the CRC 20%; on
the 48 kHz tone the fixed path 15.5%, mostly the order search run on the
side channel every block beside a fit that wins within the margin. The
i5's timers put the writer at a quarter of a 16-bit block; here it is a
sixth to a fifth, Zen 2 shifting by a variable count in one µop where the
i5 takes three, and the partitions and the CRC are the larger shares.
Nothing in the profile but the autocorrelation is at an instruction
floor. The one made cell over libFLAC's bytes by more than a byte, 8-bit
noise at 1.0046, was read the same afternoon with the flac tool's
analysis of this encoder's frames: the same pairing and fixed order as
libFLAC's on every block, and a Rice parameter one under its on the quiet
mid, the estimate's floor bias, under the open items. The open items
below are planned from this profile, and are measured on the EPYC alone
until the round is called ready for the arm64 run.

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
past 3%, it reads as before, 0.71 to 1.23 on the recordings. The day after took the rows still over: the
tone's time, 1.10 to 1.40 of level 0's on the made stereo tones, to 0.85
to 1.15, its order chosen with its coefficients' rounding weighed so that
its carried block no longer makes both codings; and the four 24-bit
recordings with sound in them, 1.16 to 1.33, to 1.01 to 1.13, by the
range check folded into the width's fold, the Rice codes grouped three or
two to a word where four no longer fit, and the fixed order search stopped
at the first rise, which took the 16-bit stereo recordings to 0.78 to
1.01 and the made stereo music to 0.78 to 1.07 as well. Level 0 itself was
6% to 17% slower in that run than in the one before, so part of the
column's move is its; on arm64, where the controls held within 2%, the
stereo recordings read 0.98 to 1.19 from 0.98 to 1.23 and the made noise
and speech 0.57 to 0.76. What is left over 1.00 is the wide sample at 96
kHz, where the fit's two passes over a 24-bit residual are a quarter of
the block, the long block of 16-bit music, and the tone and 8-bit music,
where both codings were made on most blocks. Late that evening, after
v0.0.7, the arm64 cells of that column were looked into, 1.08 on 48 kHz
music and 1.43 on its tone, 1.45 and 1.51 at 8 bits, and found to be the
machine's, level 0 a fifth faster on the Altra and this encoder the same
length on both, and the two changes below came of it: the fit skipped on
8-bit music and the tone's one channel, where it keeps losing, took 8-bit
music to 0.76 of level 0's time on x86 and 0.82 on arm64 from 1.11 and
1.45; the autocorrelation fused on aarch64 took every sounded block
there a tenth down and 48 kHz music under level 0, 0.97 from 1.08. What
is left over 1.00 is 96 kHz, the long block, 1.11 on its music on both
machines, the 24-bit four at 1.10 to 1.24 on x86 and 1.02 to 1.07 on
arm64, and the tone past 44.1 kHz, 1.00 to 1.29 on x86 and 1.01 to 1.27
on arm64, where the side channel's fit wins by an eighth of a bit and
both codings are still made. The morning after, the x86 run moved to an
idle EPYC Rome machine, where the column reads under 1.00 on every
recording, 0.58 to 0.87 on the 16-bit stereo ones and 0.78 to 0.98 on
the 24-bit four, and on every made sound but the 96 kHz tone, 1.01: the
EPYC runs this encoder half as fast again as the i5 and level 0's fixed
path about as fast, so on x86 the column is now the machine's to read,
and what is left of it is the Altra's, where the run is unchanged.

## Open items, in order

Planned again on the afternoon of 2026-10-24 from the EPYC's own profile,
under *Where it stands*: the cells still over libFLAC first, then the parts
of a block the profile puts at a fifth and the late block; the one cell in
bytes, the 8-bit noise's, was the first item and is taken, below. Each is measured on the EPYC alone, bytes on every made
sound and recording and time a block against the run before; the arm64 run
is made once, when the round is called ready.

1. **The partitions' machinery, a fifth of a 48 or 96 kHz block**: the
   fold 4%, the finest sums 5% to 6% and the search over the orders and
   the parameters 6% to 9%, 19.5% of the orchestra's block, 16% to 17% of
   the Fantasy's, the made music's, the tone's and pink noise's, 14% of
   the Aria's and 18% of 24-bit noise's. At 44.1 kHz it is 6% to 7%: a
   block of 882 frames has one factor of two, so there are two
   partitionings to try, where 960 has six and 1920 seven. Two levers. The
   fold writes every folded value and reads them back a partition at a
   time through a call for each, 32 calls of 30 values at 960; folding and
   summing each finest partition in the one pass, the sums in 32-bit lanes
   as now, saves the second read and the calls. The search weighs 63
   partitions at 960, three costs each, a multiplication and a shift a
   cost; the cost is convex in the parameter, and the three trials come to
   one quotient q = sum >> near and two comparisons, the parameter
   stepping up where count < (q + 1) >> 1 and down where (sum >> (near −
   1)) − q > count, which is the trials' answer exactly and without a
   branch, where the walk measured before lost. Measured apart first on
   1920 residuals at the parameters of 16 and 24 bits, then in the
   encoder; the same bytes on every line is the test, neither changing a
   choice.
2. **The fixed path left out where the fit keeps winning** (the tone's
   time against level 0, 1.01 at 96 kHz on the EPYC and 1.11 to 1.27 past
   44.1 kHz on the Altra, and a part of every recording's block). On the
   side channel of the tone the fit wins by about an eighth of a bit a
   sample and is never clear by the margin after the correction, so both
   codings are made on every block: the fixed order search is 12% of the
   48 kHz tone's block and the fixed path 15.5% in all, against 4% to 9%
   elsewhere. On the orchestra both are coded on two carried channels in
   three, one in three by the schedule that refreshes the correction and
   one in three by the margin. The skip rule has a mirror: a channel whose
   fit, both coded, has beaten the fixed coding by `LOSS` a sample
   `LOSSES` blocks running has the fixed path left out for `SKIPS` blocks,
   its correction kept, and both coded again after; the schedule's two
   blocks a cycle stay, so the correction is never older than a cycle.
   What it can cost is the blocks inside the run where the fixed coding
   would have won by a sliver, which the loss skip, its mirror image,
   measured at 0.01% on the recordings; what it buys is the fixed path on
   the tone's side channel and on the margin's third of the orchestra's
   carried channels, about a tenth of the tone's block and a twentieth of
   the orchestra's. Bytes first, on every recording and made sound, the
   tones' watched, since the rounding of their coefficients is what the
   fixed path catches there.
3. **The fourth block, the late block** (1.31 of a carried block on the
   orchestra on the EPYC, 14.3 against 10.9 µs, 1.34 on the Aria, 1.26 on
   the Fantasy, 1.33 on the Raven): its excess over a carried block is the
   third autocorrelation, two more fits and, where the estimates do not
   settle the pair, the two other channels' residuals and folds, about
   0.9, 0.5 and 1.8 µs on the orchestra. The excess can leave the block:
   each of the two carried blocks before it codes the pair as now and
   counts one of the two other channels besides, autocorrelation, fit and
   the one-partition count, so that at the fresh block every channel has
   real bits from the block before or the one before that, the pair's from
   its coding; the pair is chosen on those at the start of the block, with
   no work of its own, and coded as a carried block codes it. The late
   block then falls to a carried block and a count, about 1.15 of a
   carried one, and the mean stands. The counts are a block or two old
   where the fresh block's were of the block itself: the pair carried
   three blocks was measured to cost 0.00% to 0.09% of the bytes, and this
   is a choice a block late, so under that, to be measured on every
   recording; on the pianos, where left and right against mid and side are
   within a twentieth of a bit a sample and the choice hops, a hop costs
   nothing in time here where the fresh block coded it, so the margin on
   a move can be none. With a third channel's count in hand on the third
   block, the pair can move mid-cycle on a count rather than only on the
   bound, which is where part of the recordings' 1.001 against libFLAC's
   bytes went.
4. **The residual of a fit, 14% to 27% of the block**: 14% on the
   orchestra, 16% to 18% at 44.1 kHz and on the Fantasy, 22% on the 1924
   record and 27% on the Aria, where a 24-bit sample's residual runs as
   two 16-bit passes. The 16-bit loop widens each tap and multiplies it
   apart, `pmaddwd` taking eight products at a time but two taps together
   only as the compiler lays them; the samples laid out once a block as
   pairs of adjacent 16-bit values would give each `pmaddwd` two taps of
   two samples, about half the inner loop's instructions, and the split
   path takes it twice over. Measured apart first, at orders 2, 4 and 8
   over 960 and 1920 samples, 16-bit and split, against the loops as they
   are: the compiler's codegen is the whole question, and the tree of
   pairs tried before came out as the same loop. To be counted with it, in
   a copy of the tree: how often a 16-bit stereo recording's side channel
   is 17 bits wide in its samples and takes the split pass, which is where
   the loud masters' time would go.
5. **The CRC inside the writer's pass** (the frame's CRC 4% to 7% at 16
   bits, 7% to 14% on the 24-bit recordings and noise, 20% of 24-bit
   noise's block): the two chains of table lookups are their loads'
   latency, and they run after the writer, whose own work is a dependent
   chain of shifts through the accumulator. Taking each word's eight bytes
   through the tables as the writer stores it would run the loads under
   the shifts, one pass over the frame instead of two; the header's bytes
   before the first word and the alignment after the last go through the
   table as now. What it can buy is the CRC's share where the writer's
   chain leaves the load ports idle, which is a measurement, apart on
   frames of 1500 to 8700 bytes and then in the encoder on pink noise and
   the 1924 record.
6. **The stereo split and the four channels' preparing in one pass**, 8%
   to 9% of a stereo block: the split writes left, right, mid and side in
   one pass and each channel's `prepare` reads its signal back for the
   wasted bits, the constant check and the width, four passes over what
   was just written. One pass that writes the four and folds the four
   channels' bits, differences from the first and magnitudes as it goes
   keeps them in registers; a channel's shift for wasted bits, seldom,
   stays a pass of its own. On the mono reading the deinterleave and the
   header are 13.6% of a 5 µs block, the strided copy being a copy of one
   channel: there the samples can be prepared straight from the block.
   About 3% of a stereo block to measure, the same bytes.
7. **The autocorrelation**, the largest part at 16 bits, 19% to 23%, is at
   the floor the x86-64 baseline sets: 18 floating-point operations on
   every two samples on two ports, with no fused multiply-add at the
   baseline, where the EPYC has one and the Altra uses it. Nothing to take
   short of the users building with `target-cpu`, which is theirs; the
   third autocorrelation on the fresh block leaves with item 3, and
   nothing else in the block asks for one.

## Measured and taken

- **The Rice parameter and the partitioning chosen on the estimate less
  the fractions it drops** (the afternoon of 2026-10-24, item 1 of the plan
  as it stood, the 8-bit noise's bytes): `best_param` weighed a parameter
  at count·(k + 1) + (sum >> k), and the quotients' real bits are under
  that by the fractions the shift drops, about count·(1 − 2^−k)/2, nothing
  at a parameter of 0, a quarter of a bit a sample at 1 and nearly half
  from 4 on, so that on a quiet residual the smaller parameter and the
  finer partitioning were favoured by the bias and not by bits; read with
  the flac tool, the 8-bit noise's mid channel was at a parameter under
  libFLAC's and its side in two to eight partitions where libFLAC had one,
  the pairing and the fixed order the same. The weight is now the estimate
  with the count's share, count >> (k + 1), the half bit a sample that is
  the same at every parameter left out; the bits a coding reports are the
  estimate less the fractions, within a part in a thousand of the real
  bits on sound and never under them by more than half a bit a sample, and
  the most a coded channel can take, for the pair's move and the frame's
  guard against outgrowing its PCM, is its bits and that half bit. The
  three weights come from one quotient, one share and one multiplication,
  the neighbours' by constant shifts. On the EPYC: the 8-bit noise from
  1006.1 bytes a block to 1001.6, 1.0001 of libFLAC's where it was 1.0046;
  the 96 kHz tone from 1481 to 1447, 0.876 of libFLAC's and under its level
  8, where it was 0.90; every other made sound the same bytes but 8-bit
  speech, 0.2 of a byte a block over, 0.08%; the recordings within 0.02%,
  the wide mix 1.0002, the Fantasy, the prison song, the talk, the loud
  mix and the rock master 1.0001, the Raven and the train 0.9999. Counted
  exactly over 15 s of every recording before the change, the estimate's
  choice was within 0.013% of the optimum, so nothing more was there to
  find. The time: 0.7% more instructions on the orchestra and 0.9% on the
  Fantasy by the machine's counters, the bench's block within 1.5% either
  way, which is the run's noise. On the way: the parameter chosen on the
  corrected estimate but reported plain cost the mid 40 bits on the plain
  scale and the pair choice went wrong, 1003.4 bytes; reported corrected
  with the pair's bound still plain, the bound moved the pair less and the
  wide mix cost 0.03%; and the three costs each with the correction as two
  shifts cost 2% to 5% of a 48 or 96 kHz block, which the one quotient
  and one multiplication took back.
- **The fit skipped where it keeps losing** (late the evening of
  2026-10-23 by the machine's clock, after the v0.0.7 release; item 2 as
  it stood, the fixed path at 8 bits): the double coding on 8-bit music
  was not the fixed path's but the fit's. Instrumented, the fixed coding
  wins every block of 8-bit music and the fit never does, losing by a
  quarter of a bit a sample while estimated to win by a tenth, the
  channel's correction at 0.4 to 0.5 making it not clear; so on every
  block the autocorrelation, the Levinson recursion, the fit's residual
  and its partitions ran for nothing, 11 µs of a 20 µs block. On one
  channel of the 48 kHz tone the same, by an eighth to a quarter of a bit.
  A relative margin, the item as written, would not have touched it: the
  fit was estimated to win. What was taken is the noise rule widened: a
  channel whose fit, coded beside the fixed coding, has lost to it by an
  eighth of a bit a sample two blocks running skips the fit for seven
  blocks and tries it again. The margin matters: on the recordings the
  fit loses on a tenth to a quarter of the blocks where both are coded,
  but mostly within a sixteenth of a bit, and the next block's fit wins;
  skipping on any loss cost the prison song 0.74% of its bytes and the
  Raven 0.16%, on a loss past a sixteenth 0.01% to 0.03%, and on a loss
  past an eighth twice running nothing past 0.01%, with the time the same
  as on any loss; past a quarter the tone was not caught. Bytes the same
  on every made sound but 8-bit speech, 0.12%, 0.3 of a byte a block. On
  arm64, level 0 as the control within 2.6%, 8-bit music came to 0.61 of
  its time, the mono tone 0.57, the 8-bit tone 0.81, the 48 kHz tone 0.83,
  8-bit speech 0.71, and every other cell within 0.4%; the recordings
  within 0.5%, the orchestra 0.97.
- **The autocorrelation's multiply and add fused** (the same evening,
  the second lever of the arm64 level 0 cells): the lag loop is `a * b +
  c` nine times on every two samples, two instructions each at the x86-64
  baseline, which has no fused multiply-add, and one `fmla` on aarch64,
  where every core has it, if the code asks for it, which plain `a * b +
  c` may not, the one rounding differing from the two in the last bit.
  Through `f64::mul_add` where the target has the instruction, by
  `cfg!`, and the two operations where it does not, since there the fused
  form is a library call: on the Ampere Altra every sounded cell came to
  0.90 to 0.94 of its time, the recordings 0.91 to 0.93, the 99th
  percentile the same, with the bytes the same on every line, the fit's
  autocorrelation being the largest part of a block there, 5.2 µs of the
  48 kHz tone's 21 against 3.9 on the i5. The x86 build is the same code
  as before. Measured first with the helper under the lag loop's
  `#[inline(never)]` by mistake, a call a multiply-add, 1.8 times the
  time: the attribute goes with the loop, not the helper.

- **The four channels counted on the fresh block, the pair chosen on the
  counts** (the evening of 2026-10-23 by the machine's clock, the session
  after the one labelled 2026-10-24; item 1 as it stood): the fresh block's excess over
  a carried one was not the third autocorrelation and the two fits, 2.6 µs
  of the 5 on the orchestra, but the choice made again on real bits:
  counted, the pair stood on 85% of the fresh blocks and yet 1.9 pairs
  were coded on the mean one, nine blocks in ten hopping to another pair
  and most hopping back, since a channel's one-partition count is over
  its real bits by the partitioning's gain and its fit's estimated saving
  is over the real one. On the estimates alone, without the counts, the
  loop coded 3.4 pairs a block, nearly every channel, and the Fantasy
  came to 0.9983 of its bytes, the Aria 0.9989, the train 0.9996 and the
  96 kHz tone 0.990, nothing rising: that is libFLAC's choice, every pair
  by its bits. Without the loop at all, the fresh block fell to a carried
  block's height, 18.5 µs against 21.7 on the orchestra, 37.5 against
  43.8 on the Aria, for 0.12% of the Fantasy's bytes and 0.05% of the
  Aria's. The model's bias against real bits is the channel's, not the
  block's: a bias taken from the channels just coded and put on the others
  found no pair the loop without it did not, and each channel's last bias
  carried between blocks cost the Raven 0.27%. So the choice is made on
  real bits once: all four channels have the fit's residual taken and
  counted in one Rice partition, the fold and the finest sums the
  partition search needs anyway, and the fixed predictor counted beside it
  where the fit is not clear; the pair whose two counts are least is
  taken and its two channels finished, the search over the partitions and
  the fixed path where the fit was not clear. The estimates are kept for
  the gate: where they set one pair under every other by a quarter of a
  bit a sample, it is taken and the two other channels are not counted.
  Measured over every fresh block of twelve recordings and the made
  sounds, no count changed the choice past a fifth of a bit, and an eighth
  is not safe, since on 8-bit music the estimates' pair is wrong on every
  block and wrong on all of the 8% they separate by a tenth; the gate
  settles every block of the made 16-bit and 24-bit music, half the made
  speech, a third of the noise and a quarter to a third of the mixes, and
  none of the piano, where left and right against mid and side are within
  a twentieth of a bit a sample on 99% of the blocks and the estimates'
  pair is wrong on half of them, which is the 0.04 bit a sample the counts
  buy there. Bytes as the loop on the estimates alone, the Fantasy 0.9983,
  the Aria 0.9989, the train 0.9996, the Raven and the prison song 0.9999,
  the 96 kHz tone 0.990, every other sound the same; the fresh block 21.7
  µs on the orchestra as before, 36.8 against 33.1 on the Fantasy and 48.4
  against 43.8 on the Aria, the made music's back at a carried block's
  height, and the means within 3% either way on the recordings on both
  machines. The weighing's four counts, the kept estimates and the loop
  are gone with it.
- **The coefficients' rounding weighed in the order choice, exactly**
  (2026-10-24, item 3 of the plan as it stood, the tone's coefficients):
  each order's predictor is quantized as it would be written and the
  squared error its rounding adds is the quadratic form of the
  coefficients' errors over the autocorrelation, which is exact over the
  signal as the autocorrelation sees it, the windowed block with zeros
  beyond, since the fit's error is least there and the quantized
  predictor's is the fit's and the form; the order is chosen on the two
  together, and the candidate carries its quantized predictor, so nothing
  is quantized again to code it. The statistical model tried the night
  before, order times the mean square over twelve times four to the shift,
  overstated the error on smooth sound because libFLAC's quantization
  carries each coefficient's rounding into the next and the errors cancel;
  the form sees the carried errors as they are. On the samples themselves
  the fit's own error at the block's edges adds a cross term of either
  sign, which the unit test shows at eleven times the form on a bare tone
  and the channel's correction absorbs. The order with the fewest bits
  before rounding is quantized first and another only where its bits
  before rounding are under the best with it: 1.1 to 1.3 orders an
  estimate on the recordings, all eight on a tone and at 8 bits, where the
  rounding is large at every order; quantizing only the first lost every
  byte the tone gained, so the comparison of the tied orders is what buys
  them. Written first with the library's `powi`, `log2` and `round`, which
  are calls at the x86-64 baseline, it cost 9% to 16% of a 16-bit block;
  with the exponent and the power from the float's bits and the rounding
  as a truncating conversion, 2% on x86, within the noise, and on arm64,
  with the range check below, 0.4% less on made music and 7% less on the
  tone. The tone's bytes went from 1008 to 997 a block at 48 kHz, 940 to
  920 at 44.1, 1576 to 1496 at 96 and 2440 to 2283 at 24 bits, 0.99, 0.98,
  0.95 and 0.94 of libFLAC's, and the tone's carried block no longer makes
  both codings; the recordings came to the same bytes within two parts in
  ten thousand, the Raven 0.9994, the Aria and the Fantasy 1.0001, the
  8-bit speech 1.0004. The shift one less, tried where a coefficient is
  clamped at the precision's edge, which a low tone's first is, changed no
  byte of any sound and is out: the carry takes most of a clamp's error
  into the next coefficient.
- **The range check folded into the channels' preparing** (the same day):
  the two passes over the block for its least and greatest sample, 0.8 µs
  at 960 frames of a pair and 2 at 1920, are gone; the fold that finds
  the width the samples need says whether one is outside the stream's,
  and the first such is found only to name it in the error. The mid and
  side are taken wrapping, the block not yet refused.
- **The Rice codes grouped by the parameter** (the same day, item 2):
  four codes to a word is the fastest grouping to a parameter of 14,
  where four of 15 bits leave three for the quotients; from 15, where
  four seldom fit and each group fell to its pairs after the four lengths
  were summed for nothing, three to a word take 0.73 to 0.77 of four's
  time on 1920 exponential codes measured apart and pairs 0.88 to 0.93;
  from 20, where three of 21 bits do not fit, pairs take 0.92 of four's
  and three 1.1. So four to 14, three to 19 and two past it, which is
  where a 24-bit block's parameters sit: on arm64 the 1924 record went to
  0.92 of its time, the rock master to 0.96, pink noise to 0.96 and white
  noise to 0.91, and nothing at 16 bits moved.
- **The fixed order search stopped at the first rise** (the same day): the
  residual's size over the fixed orders falls and then rises, the
  differences shortening what is smooth and lengthening what is noise, so
  the orders are taken from 0 until one is not smaller than the one
  before. The bytes were the same on every made sound and recording, and
  a block of noise, where order 0 wins, takes one difference pass where it
  took four: with the writer's grouping, white noise at 24 bits went to
  0.82 of its time on arm64 and 0.81 on x86, the made noise to 0.80 to
  0.87 by the shape, the speeches to 0.90 to 0.96, the mono reading to
  0.95, the orchestra to 0.97.
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

- **The Rice parameter stepped from the logarithms' difference**, once
  more (the same afternoon, with the correction): the corrected cost's
  convexity gives the step up or down from one quotient and two
  comparisons, without the trials' multiplications, and on the EPYC it was
  slower, `search` going from 6.9% of the made 48 kHz music's block to
  10.6% and the orchestra taking 1.6% more instructions against the three
  weights' 0.7%: the independent weights run four instructions a cycle
  and the step's chain does not. Out, as the walk on arm64 was.
- **The Rice bits counted exactly** (the afternoon of 2026-10-24, with
  the flac tool's residual dump over 15 s of every recording, this
  encoder's own frames read back): the exact optimum over the
  partitionings and the parameters is within 0.013% of what the estimate
  chooses, the orchestra the 0.013%, the Aria and the Fantasy within
  0.001%; the three trials on the sum find it, and an exact count, a pass
  a trial, would buy nothing a pass could pay for. The 8-bit noise is the
  one sound where the estimate's floor bias moves a choice, item 1 of the
  open items.
- **The Rice parameter walked** (2026-10-24, item 1 as it stood): the
  cost of a partition, count times the parameter and one plus the
  quotients, is convex in the parameter, so from the logarithms'
  difference the parameter was stepped up while the next cost less and
  down while the one below did, each step a comparison of two quotients
  where a trial costs a multiplication. The same bytes on every sound, the
  three trials having found the least everywhere, and 1.01 to 1.05 of the
  time a block on arm64 on every made sound and recording, the walk's
  branches costing more than the multiplications they replaced. Out.
- **The residual as a tree of pairs, and in `f64` lanes** (2026-10-24,
  for the wide sample): the compiler emits the same 36-instruction loop
  for four samples whether the taps are summed in turn or as pairs, and
  lanes of `f64`, exact to 53 bits for a 24-bit sample against a 13-bit
  coefficient, took 7.9 µs against the split passes' 1.3, the floor being
  a call at the baseline. Out.
- **The frame's CRC in four chains**: 1.17 to 1.31 of the two halves'
  time on frames of 1500 to 8657 bytes, the four quarters' loop costing
  more than the latency it hid. Out.
- **The quantization's shift one less where a coefficient clamps**: no
  byte changed on any made sound or recording; the carried rounding
  nearly cancels a clamp's error in the next coefficient. Out.
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
