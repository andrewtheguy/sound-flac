# The plan of the encoder, and what was decided

What the encoder is for, what is decided about it, and what is taken up
next, in which order and why. What it is now is in the module docs of
`src/lib.rs`, `src/subframe.rs` and `src/lpc.rs`; the tables it is measured
by are in [benchmarks.md](benchmarks.md). Written on 2026-10-22, on the
first measurement over recordings, at commit e3173f7 of branch
`own-encoder`.

## What it is for

The wlshare daemon's and the remotex gateway's, which stream a desktop's
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
  what the mean says, 0.05% to 0.42% on the recordings, and it is a
  desktop's share beside its video encoder.
- **Bytes are the link's.** A frame's bytes are its bitrate, 30% to 50% of
  the PCM on acoustic music and speech, 69% on a loud modern master at
  16 bits and 79% at 24, 75% to 95% on noise and old transfers. Fewer
  is better everywhere, and worth more than time: the time is already not
  felt.
- **The shapes are the users'.** wlshare codes 8-bit and 16-bit at 8 to
  96 kHz, remotex 16-bit at 44.1 kHz; the crate takes 4 to 24 bits, 1 to 8
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

From the tables of 2026-10-22: 0.40 to 0.94 of libFLAC's time a block on
the recordings for its bytes within 1.1%, the late blocks 0.48 to 0.93 of
its; the made sounds 0.40 to 0.84 for the same bytes within a byte, the tone
at 96 kHz 1.07. What is over the line is the fit's single precision on a
long block of a quiet, resonant signal, and the pairing chosen before the
fit. Where the time goes at 16-bit stereo: the fixed predictors on four
channels and the pairing about 18 µs, the fit 1.5 µs a channel on two, its
residual 1.1, the partitions 1, the rest the bits out and the CRC; at 24
bits the residual runs in 32-bit lanes, 3 µs a channel.

## Open items, in order

1. **The fit's precision, without `f64`'s time.** The autocorrelation in
   `f32` lanes misfits the Goldberg Aria to 1.011 of libFLAC's bytes,
   the Fantasy to 1.010, the Raven to 1.006 and the made tone at 96 kHz to
   1.07, where `f64` lanes give 1.002, 1.004, 1.001 and 0.95 for 8 µs more
   a block at 48 kHz and 20 at 96 kHz. To measure, in this order: (a) the
   lanes in `f32` with the signal's mean taken out first, since a DC offset
   is what swamps the small lags of a quiet signal; (b) `f32` lanes flushed
   to `f64` every 64 or 128 samples, measured at 2.5 to 3 µs a channel
   before, to be measured again as a flush and not a block; (c) `f64` lanes
   only where the block is long, 96 kHz, where the time is a smaller share
   of the block. Taken where the bytes come back for under 2 µs a channel
   at 48 kHz.
2. **The fit before the pairing, cheaply.** The pairing is chosen on the
   fixed predictors' bits over left, right, mid and side, then two channels
   are fitted. libFLAC fits all four and chooses after, and where a fit
   wins on side but not on right, or by a different margin, the pairing
   differs: the train station's 1.007 and part of the pianos' is this. To
   measure: the Levinson error estimate, not the bits, of a fit on all four
   channels, 1.5 µs each, then the pairing on the lesser of each channel's
   fixed and fitted estimate, then the residual of the two chosen only.
   Costs 3 µs a block on stereo for the bytes it buys; taken if it buys
   0.3% or more on the recordings that are over.
3. **The block before as a start.** The sound of one block is mostly the
   sound of the last: a predictor fitted to the block before, tried on this
   block for its residual (1.1 µs a channel) without a fit (1.5 µs), is
   taken where it is within the margin of this block's fit estimate; or the
   fit is skipped where the block before declined it and this block's
   order-1 fixed residual is no smaller than its order-0, which is noise.
   Buys 1.5 to 3 µs a block on stationary sound and noise, which is what
   makes noise the dearest made sound at 16 bits (30 µs against speech's
   19). Costs nothing in bytes where the margin holds, to be measured on
   the recordings, whose sound changes.
4. **The late blocks.** A 99th-percentile block is 1.8 to 3.0 times the
   mean, for both encoders, and this encoder's worst is 140 to 399 µs. To
   find which blocks those are, by what in them, with the bench logging
   the worst blocks' positions: if they are the loud ones, writing more
   Rice bits, the bits writer is the item; if they are where a predictor
   escapes to the 64-bit residual path, or where a partition order search
   runs to its end, that path is. Taken where the 99th percentile falls
   without the mean rising.
5. **The 24-bit residual.** At 24 bits the predictor's residual runs in
   32-bit lanes at 3 µs a channel, where 16-bit lanes take 1.1; a 24-bit
   stream from a desktop is 16-bit samples shifted up as often as not, which
   wasted bits already finds. Where the samples after the shift fit 16
   bits, the narrow path should take them: 0.84 on made music and 0.92 on
   the 78 rpm transfer are this. The indie rock master, 0.86 at 24 bits,
   is not: its low bits are the mix's noise floor, there is nothing to
   shift, and its 3 µs a channel in 32-bit lanes is the middling path's
   own cost, to be looked at on its own if that path is touched.
6. **The pairing's cost.** Four channels get the fixed predictors' bits,
   about 18 µs of a 16-bit stereo block's 28. The bits of order 2 alone,
   or the sum of absolute residuals in place of the Rice bits, may choose
   the same pairing on nearly every block for a third of the time; measured
   on the recordings, taken if the pairing changes on under 1% of blocks
   and the bytes within 0.1%.
## Measured and not taken

- **AVX2 by runtime dispatch** on the fixed path: nothing gained at the
  baseline's lanes, and it needs `unsafe`. Out.
- **`f64` autocorrelation throughout**: the bytes of item 1 for 8 to 20 µs
  a block, a third of the block's time at 48 kHz. Out as it stands; item 1
  looks for the bytes without the time.
- **Fitting the tone when the fixed predictor's estimate is better**: the
  Levinson error estimate predicts the Rice bits to ±0.04 bit a sample at
  16 bits, and skipping the fit on that estimate changed no byte at any
  margin from none to never-skip. The skip stays.
- **flacenc**, the pure-Rust encoder tried first: 250 µs a block at its
  default, 75 fixed only, against libFLAC's 46 and this encoder's 28.
