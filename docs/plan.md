# The plan of the encoder, and what was decided

What the encoder is for, what is decided about it, and what is taken up
next, in which order and why. What it is now is in the module docs of
`src/lib.rs`, `src/subframe.rs` and `src/lpc.rs`; the tables it is measured
by are in [benchmarks.md](benchmarks.md). Written on 2026-10-22, on the
first measurement over recordings, at commit e3173f7 of branch
`own-encoder`; brought to the run of the fit in double precision, its first
item, the same day.

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
  what the mean says, 0.05% to 0.43% on the recordings, and it is a
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

From the tables of 2026-10-22, the fit in double precision: 0.46 to 1.01 of
libFLAC's time a block on the recordings for its bytes within 0.6%, the
late blocks 0.54 to 0.90 of its; the made sounds 0.42 to 0.94 for the same
bytes within a byte, the tone at 24 bits 1.05. What is over the line in
bytes is the pairing chosen before the fit, the train's 1.006, and the
tone's quantized coefficients; in time, the two 96 kHz 24-bit recordings on
the line. Where the time goes at 16-bit stereo: the fixed predictors on four
channels and the pairing about 18 µs, the fit 3.2 µs a channel on two, 2.9
of it the autocorrelation in double, its residual 1.1, the partitions 1, the
rest the bits out and the CRC; at 24 bits the residual runs in 32-bit lanes,
3 µs a channel.

## Open items, in order

1. **The fit before the pairing, cheaply.** The pairing is chosen on the
   fixed predictors' bits over left, right, mid and side, then two channels
   are fitted. libFLAC fits all four and chooses after, and where a fit
   wins on side but not on right, or by a different margin, the pairing
   differs: the train station's 1.006 and the Fantasy's 1.004 are this. To
   measure: the Levinson error estimate, not the bits, of a fit on all four
   channels, 3.2 µs each, then the pairing on the lesser of each channel's
   fixed and fitted estimate, then the residual of the two chosen only.
   Costs 6 µs a block on stereo for the bytes it buys; taken if it buys
   0.3% or more on the recordings that are over, or if item 2 pays for it.
2. **The block before as a start.** The sound of one block is mostly the
   sound of the last: a predictor fitted to the block before, tried on this
   block for its residual (1.1 µs a channel) without a fit (3.2 µs), is
   taken where it is within the margin of this block's fit estimate; or the
   fit is skipped where the block before declined it and this block's
   order-1 fixed residual is no smaller than its order-0, which is noise.
   Buys 3 to 6 µs a block on stationary sound and noise, which is what
   makes noise the dearest made sound at 16 bits (31 µs against speech's
   22). Costs nothing in bytes where the margin holds, to be measured on
   the recordings, whose sound changes.
3. **The late blocks.** A 99th-percentile block is 1.5 to 2.3 times the
   mean, for both encoders, and this encoder's worst is 104 to 298 µs. To
   find which blocks those are, by what in them, with the bench logging
   the worst blocks' positions: if they are the loud ones, writing more
   Rice bits, the bits writer is the item; if they are where a predictor
   escapes to the 64-bit residual path, or where a partition order search
   runs to its end, that path is. Taken where the 99th percentile falls
   without the mean rising.
4. **The 24-bit residual.** At 24 bits the predictor's residual runs in
   32-bit lanes at 3 µs a channel, where 16-bit lanes take 1.1; a 24-bit
   stream from a desktop is 16-bit samples shifted up as often as not, which
   wasted bits already finds. Where the samples after the shift fit 16
   bits, the narrow path should take them: 0.91 on made music and 1.00 on
   the 78 rpm transfer are this. The indie rock master, 0.91 at 24 bits,
   is not: its low bits are the mix's noise floor, there is nothing to
   shift, and its 3 µs a channel in 32-bit lanes is the middling path's
   own cost, to be looked at on its own if that path is touched.
5. **The pairing's cost.** Four channels get the fixed predictors' bits,
   about 18 µs of a 16-bit stereo block's 35. The bits of order 2 alone,
   or the sum of absolute residuals in place of the Rice bits, may choose
   the same pairing on nearly every block for a third of the time; measured
   on the recordings, taken if the pairing changes on under 1% of blocks
   and the bytes within 0.1%.
6. **The tone's order.** A pure tone is predicted away by order 2 and every
   higher order ties on the error estimate, which takes the coefficients as
   exact; what the frame then costs is their rounding to the precision,
   and the made tone's bytes moved by a tenth either way with every change
   to the fit's arithmetic, 1.05 of libFLAC's at 24 bits today. To measure:
   where orders tie, or where the error estimate is under a bit a sample,
   the residual of the two lowest tied orders written and the smaller kept,
   1.1 to 3 µs a channel on those blocks only, which are few in a stream.
   Last, since a tone is brief.

## Measured and taken

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
  The bits estimate is clamped at none a sample as libFLAC's is, so the
  orders that predict a tone exactly tie and the lowest is taken.

## Measured and not taken

- **AVX2 by runtime dispatch** on the fixed path: nothing gained at the
  baseline's lanes, and it needs `unsafe`. Out.
- **Fitting the tone when the fixed predictor's estimate is better**: the
  Levinson error estimate predicts the Rice bits to ±0.04 bit a sample at
  16 bits, and skipping the fit on that estimate changed no byte at any
  margin from none to never-skip. The skip stays.
- **flacenc**, the pure-Rust encoder tried first: 250 µs a block at its
  default, 75 fixed only, against libFLAC's 46 and this encoder's 28 before
  the fit went to double.
