# What the encoder was measured at

The numbers the encoder's decisions were taken on, against libFLAC, which the
crate linked until v0.0.4. How a block becomes a frame is in the module docs
of `src/lib.rs`, `src/subframe.rs` and `src/lpc.rs`, each carrying the
numbers of its own part; what is taken up next, and why, in
[plan.md](plan.md).

The tables are made by `tools/bench`, a cargo project beside the crate that
is not a member of it, so the crate itself keeps to `thiserror`: it builds
v0.0.4 of this repository from its tag for libFLAC at its default level 5 as
that release ran it, a stream of its own for every frame, and libFLAC's sys
crate for its levels 0 and 8. It codes two sets with each of the three and
with this crate: five made sounds at six shapes, fifty 20 ms blocks of each,
and fourteen recordings, twelve from archive.org, public domain or CC0, and
two of the user's own, whole, in blocks of 20 ms. A made sound's time is one block's, the least of five passes over the
fifty, each pass timed as a whole after one to warm up. A recording's is the
least of three passes' mean block, each block timed on its own, and with it
the least of the passes' 99th-percentile and worst blocks, the latency a
stream sees; its bytes are the frames' over the recording, a block's share.
`tools/tables.py` makes the tables below from the log:

```sh
tools/samples.sh copy      # the recordings into tmp/samples, from the store
cargo run --release --manifest-path tools/bench/Cargo.toml > bench.log
python3 -I tools/tables.py bench.log
python3 -I tools/tables.py bench.log bench-arm64.log   # with an arm64 run's log, the last section too
```

They were run on 2026-10-23 on a six-core Intel i5-8500T under WSL2, one
thread, the machine idle, with rustc 1.98.1 at the x86-64 baseline (SSE2, no
`target-cpu`), on branch `own-encoder` at the commit that carries them: the
encoder with its stereo pair chosen on the fit as well as the fixed
predictors every fourth block and carried between, the fit skipped on noise,
and, from the afternoon's work of [plan.md](plan.md), a carried block's two
channels coded the fit first with the fixed path left out where the fit
clearly wins, the autocorrelation in one pass at the floor of the baseline's
floating-point ports, and the frame's CRC in two halves. A time on this
machine moves by up to a tenth from one run to the next (libFLAC's own times
were within 7% of the morning's in this run, either way), so a time ratio
within a tenth of 1.00 is on the line; a bytes ratio is exact. A worst block
is one block of thousands and the machine's as much as the encoder's, even
as the least over passes, so it is read as an order of magnitude. The bench
also logs `worst_at`, the block that was slowest at its least over the
passes, and dumps every block's least time and bytes when asked
(`BLOCKS=dir`), which is how the late blocks below were read; since this
run every pass starts from an encoder that has seen no block, so that what
the encoder carries from block to block, the fourth-block choice, falls on
the same blocks in every pass and the least over the passes is the block's
own. Before that an encoder ran on through the passes, and on a recording
whose blocks are not a multiple of four the choice moved by a block each
pass, so a block's least was its cheaper kind and the 99th-percentile and
worst columns of the morning's tables understated the encoder's own late
blocks by about a third.

The arm64 run of the last section was made the same day from the same
commit on a two-core Ampere Altra (Neoverse N1) in a Debian 13 LXC container,
one thread, the machine idle, with rustc 1.98.1 at the aarch64 baseline, whose
NEON the compiler uses on its own: the tree is copied there, the recordings
are kept there from the morning's run, the bench run, and its log brought
back as the second log of `tools/tables.py`. The morning's first run there
was two to three times slow on the made sounds while the file system was
still compressing the recordings just copied, and was run again once the
machine was idle; the recordings came to the same bytes and the same times
within 0.2 µs in both runs, and the made sounds run twice more agreed to
0.1 µs, so a time on that machine is repeatable where this one's moves by a
tenth.

Throughout, a *ratio* is this encoder over libFLAC, so under 1.00 is less
time or fewer bytes. A block is 20 ms of sound, so *of one core* is its time
over 20 ms, the share of a core the encoder takes coding the stream as it
comes; *of the PCM* is the frame's bytes over the block's. Green is this
encoder's side of the line and red the other; a number on the line is left
plain.

## The made sounds

Made, not recorded, so the run is the same on any machine and the sounds
are each one thing: fifty blocks, a second of each, at 48 kHz, 44.1 kHz and
96 kHz stereo 16-bit, 48 kHz mono, and 48 kHz stereo at 8 and 24 bits.

- `music`: a three-note chord with five harmonics a note and a slow vibrato,
  a bass note under it, a little room noise, and a right channel that is the
  left mixed with another note.
- `speech`: one voice-like tone of twelve harmonics, its pitch and loudness
  moving, with breath noise, the same on both channels.
- `tone`: a pure 1 kHz sine at −6 dBFS, a notification's beep, at 0.7 of it
  on the right. A resonance is the ill-conditioned fit, where a predictor's
  coefficients are large and its rounding tells.
- `noise`: white noise at −20 dBFS on each channel, nothing to predict.
- `silence`: zeros.

## The recordings

Recordings from archive.org, public domain by age or by mark, or given to
the public domain by their makers under CC0, lossless as uploaded, listed in
`tools/samples.txt` with the licence each item states. `tools/samples.sh
fetch` downloads them into the artifact store, one folder an item, decodes a
FLAC to a WAV beside it and writes `SHA256SUMS`; `tools/samples.sh copy`
brings the WAVs to `tmp/samples`, which is gitignored, for a run. The store
is `/mnt/dasdata/backup/sound-flac-artifacts/samples/` on this machine
(`SOUND_FLAC_ARTIFACTS` names another), 2.0 GB with the FLACs; nothing in
the repository holds a recording. Two more are the user's own music, not
licensed for publication: two dense mixes at 44.1 kHz stereo 16-bit, WAVs at
the root of the store named by their track numbers, not in
`tools/samples.txt` and not fetched. The tables carry their numbers, named
for what they sound like, and the repository nothing else of them.

| Recording | Item | Shape | Seconds | What it is |
|---|---|---|---:|---|
| Bach, Goldberg Aria, piano | `OpenGoldbergVariations` | 96 kHz stereo 24-bit | 299 | Kimiko Ishizaka's studio recording, CC0: a quiet piano, long decays |
| Chopin, Fantasy Op. 49, piano | `musopen-chopin-complete-works-flac` | 96 kHz stereo 16-bit | 738 | Musopen's recording, CC0: piano from quiet to loud |
| Shostakovich 5, orchestra, live | `dso20060204old` | 48 kHz stereo 16-bit | 324 | a concert recording, one movement, the hall in it |
| Indie rock, 2022, mastered | `1.-golf-club-stay-awhile` | 48 kHz stereo 24-bit | 224 | Golf Club's "Stay Awhile", CC0 by the band: guitars, drums and voices in a loud modern master, the one popular record |
| Mix at −13 LUFS, wide stereo | the user's own | 44.1 kHz stereo 16-bit | 250 | a studio mix integrated at −12.6 LUFS over a 6.4 LU range, peaks at full scale, the side channel 8 dB under the mix: 66% of its PCM |
| Mix at −9 LUFS, loud master | the user's own | 44.1 kHz stereo 16-bit | 225 | a louder master, −9.2 LUFS over 4.6 LU, peaks at full scale, the side channel 6.6 dB under the mix: 72% of its PCM |
| Limehouse Blues, 1924 78 rpm | `TheColumbiansCollection1924-1929DirectedByBenSelvin` | 96 kHz stereo 24-bit | 197 | a 78 rpm transfer: dance band under surface noise |
| Prison work song, 1947 field recording | `negroprisonsongs` | 44.1 kHz stereo 16-bit | 252 | Lomax's Parchman recording: voices and axes outdoors |
| The Raven, read, produced | `TheRavenByEdgarAllanPoe` | 44.1 kHz stereo 16-bit | 559 | a radio theatre reading with music and effects |
| Leviticus 13, read | `LeviticusKJV` | 44.1 kHz mono 16-bit | 725 | one voice reading, a room's quiet between verses |
| JFK at Rice, 1962 | `NasaAudioHighlightReels` | 48 kHz stereo 24-bit | 836 | a speech through a public-address system, a crowd |
| Train station, field recording | `aporee_53029_60598` | 44.1 kHz stereo 16-bit | 141 | a passenger train arriving, the station's ambience |
| Pink noise | `TenMinutesOfWhiteNoisePinkNoiseAndBrownianNoise` | 48 kHz stereo 24-bit | 600 | noise falling 3 dB an octave, predictable a little |
| White noise | `TenMinutesOfWhiteNoisePinkNoiseAndBrownianNoise` | 48 kHz stereo 24-bit | 600 | nothing to predict: the most a stream can cost |

## What the tables say

- **Every recording and every made sound takes less than half of what
  libFLAC at its default level takes of a core, most of them a little
  over a third.** The recordings are 0.31 to 0.59 of its time a block:
  16-bit stereo 0.39 to 0.47, mono 0.31, 24-bit 0.37 to 0.59, the two at
  96 kHz and 24 bits 0.51 and 0.59. A block of a recording is 0.04% to
  0.25% of one core. The made sounds are 0.25 to 0.49: music 0.31 to 0.47,
  speech 0.31 to 0.45, the tone 0.38 to 0.49, noise 0.25 to 0.37, silence
  0.07 to 0.10. The morning they were 0.44 to 0.77 and 0.40 to 0.74, the
  night before 0.43 to 0.87 and 0.41 to 0.91, and before that night 0.46 to
  1.01 and 0.42 to 0.94, for the same bytes within half a per cent.
- **The bytes are libFLAC's at its default level, within 0.2%.** Every
  recording is 1.000 but the two pianos at 1.001 and 1.002, the prison song,
  the Raven and the two private mixes at 1.001, a byte to six a block, which is what fitting only the carried pair
  between full choices and coding it the fit first cost them (the
  measurements are in [plan.md](plan.md)); before the pair was chosen on the
  fit the train station was 1.006 and the Fantasy 1.004. Every made sound
  comes to the same bytes within a byte a block but the pure tone, 1.02 at
  44.1 kHz, 0.95 at 96 kHz and 1.05 at 24 bits: a tone is predicted away by
  order 2 and what its frame then costs is the rounding of the coefficients,
  which goes with every change to the fit's arithmetic and which no estimate
  tried could foresee. Against libFLAC's smallest level, 8, which tries
  every order to 12 and every stereo pairing in four to seven times this
  encoder's time, the made sounds are 1.00 to 1.03 but the tone at 96 kHz,
  1.07, and at 24 bits, 1.06, and the recordings 1.00 to 1.01.
- **The late block is the fourth block, where the pair is chosen afresh.**
  Each block's least time over the passes, each pass from an encoder that
  has seen no block, puts this encoder's own 99th-percentile block at 1.5
  times its mean and its worst at about twice, on every recording: the
  block that codes all four channels with their fixed predictors and fits
  them from three autocorrelations costs 1.5 times the block that codes
  the carried pair, 26 µs against 17.5 on the orchestra, 57 against 35 on
  the Aria, 21 against 14 on the Raven and the wide mix. A single pass's
  99th percentile is higher still for both encoders, which is scheduling
  and cache, not the sound. On the recordings this encoder's 99th-percentile
  block is 0.38 to 0.69 of libFLAC's, 16 to 98 µs against 43 to 141, and
  its worst block 102 to 263 µs, at most 1.3% of the 20 ms the block is,
  where libFLAC's worst is 144 to 648. What is the encoder's own beyond the
  fourth block is the loud block, writing more Rice bits, and on white noise
  every eighth block, where the fit is tried again after being skipped.
- **The pair is chosen on the fit as well as the fixed predictors every
  fourth block, and carried between.** On the fourth block every channel
  is coded with its fixed predictor first and its fit is weighed from its
  autocorrelation, the mid's derived from left's, right's and side's so
  three passes serve four channels; the pair is chosen on each channel's
  real fixed bits less the bits its fit is estimated to save, the two
  chosen are coded, and the pair is chosen again with their real bits,
  until it stands. The estimate is good at differences and poor at levels:
  chosen on estimates alone the 8-bit speech was 1.63 and the tones 1.05 to
  1.10. On the three blocks between, the pair is coded again, each channel
  the fit first: where the fit is estimated to beat the best fixed
  predictor by an eighth of a bit a sample after a correction the channel
  learned the last time both were coded, the fit alone is coded and the
  order search and the fixed partitions are not run, and otherwise both are
  and compared. The correction is what the estimates miss, the window and
  the coefficients' rounding: on the orchestra at 16 bits the fit is coded
  alone on a third of the carried channels, on the Raven on three in five,
  on the mixes nine in ten and on the Aria at 24 bits nearly all. The two
  other channels are not looked at past a constant check and a bound from
  one pass over their differences, and the pair moves where the bound says
  another is certainly cheaper, 160 times in 12 000 blocks of the wide mix
  and once in the Raven's. A channel on its own, mono or more than two, is
  coded the fit first on every block. Where the fit is estimated not to win
  on a channel of fixed order 0, which is noise and a quiet desktop, it is
  skipped for seven blocks.
- **Where the time goes at 16-bit stereo**, on the orchestra, by the parts
  measured apart: a carried block, 17.5 µs, is the two autocorrelations 2.8,
  the fits from them 0.6, the pair's two residuals 2.2 and their partitions
  1.6, the two fixed codings 3.8 where the fit is not clear, which on the
  orchestra is two carried channels in three, the folds and the two bounds
  1.2, the range check and the stereo split 1.6, the bits out 3.4 and the
  CRC 0.5; the fourth block, 26 µs, has the four fixed codings 7.6, three
  autocorrelations 4.2 and four fits 1.2 in place of the two, two and two.
  The night before, 30 µs a block all alike, the three autocorrelations were
  8.3, in two passes of two lanes that checked every pair of samples; the
  one pass without a check is at the floor the baseline's two
  floating-point ports set, 1.4 µs a channel. At 24 bits the residual of a
  fit runs in two 16-bit passes, and a 17-bit side channel whose samples fit
  16 bits takes the 16-bit pass.
- **Against libFLAC's fastest level, 0, which fits no predictor and codes
  stereo as it comes**, this encoder takes 0.81 to 1.04 of its time on the
  16-bit stereo recordings, the orchestra at 1.01 and the Fantasy at 96 kHz
  the two over 1.00, 0.74 on the 1962 speech and 0.75 on white noise, 0.53
  on the mono reading, and 1.11 to 1.28 on the four at 24 bits with sound
  in them, for 0.49 to 0.99 of its bytes: the two stereo speeches 0.49 and
  0.78, the music 0.78 to 0.96, noise 0.99. On the made sounds it is 0.81
  to 1.11 on stereo music and tone, 0.60 to 0.85 on speech, 0.60 to 0.77
  on noise, 0.47 to 0.71 on mono and 0.13 to 0.22 on silence. The morning it was 1.21 to 1.79 on the stereo recordings and
  1.25 to 1.89 on stereo music and tone, and the night before 1.06 to 2.09
  and 1.35 to 2.37. The two are not doing the same work: level 0's block of
  silence alone costs 12.6 µs where this encoder's costs 2, and what this
  encoder does that level 0 does not, the fit and the stereo choice, is
  most of its carried block. What was measured to bring the column down,
  and what is left, is in [plan.md](plan.md).
- **A loud modern master is most of its PCM at 24 bits.** The indie rock
  track, mixed and mastered loud in 2022 and the one popular record in the
  set, is 79% of its PCM at 48 kHz 24-bit for both encoders, where the 1962
  speech at the same shape is 31% and the orchestra at 16 bits 39%: under a
  dense mix the low bits of a 24-bit sample are the mix's own noise floor,
  which no predictor takes out and Rice writes as it is. The same track
  dithered to 16 bits, measured aside on 2026-10-22 and not in the tables,
  was 69% of its PCM, 2645 bytes a block against libFLAC's 2638 (1.003).
- **On arm64 the encoder stands where it does on x86, a little further on
  its side of libFLAC's default level, and against level 0 where x86 is.**
  The same run on the two-core Ampere Altra, in the last section: the
  recordings are 0.32 to 0.48 of libFLAC's time a block where x86 is 0.31
  to 0.59, the made sounds 0.27 to 0.47 against 0.25 to 0.49 and silence
  0.07 to 0.09 on both. This encoder's mean block is the i5's or shorter
  on every recording, by up to 6 µs, while libFLAC's is longer on every
  stereo recording and 15% shorter on the mono reading; this encoder's
  99th-percentile block is lower there on every recording, 8 to 65 µs
  against 16 to 98. Against level 0 it is 0.99 to 1.24 of its time on the
  stereo recordings but white noise and the 1962 speech, 0.79 and 0.81,
  where x86 is 0.74 to 1.28, and 0.75 on the mono reading where x86 is
  0.53: level 0's fixed path is cheap on the Altra.
  Nothing in the encoder is written for either machine. LLVM vectorizes the
  residual of a fit in four 16-bit lanes with NEON's widening multiply where
  SSE2 has its own, the autocorrelation in two f64 lanes and the fixed
  predictors' residual sums in eight 16-bit lanes; the Rice coding and the
  bit writer are scalar on both. This encoder's bytes a block are the same
  on both machines to a tenth of a byte on every made sound and every
  recording, so a frame does not depend on the machine; libFLAC's differ on
  the 24-bit tone, 2321 against 2315 at level 5 and 2302 against 2502 at
  level 8, its arm64 build taking another arithmetic path through the
  ill-conditioned fit.

## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.38}$ | $\color{green}{0.39}$ | $\color{green}{0.46}$ | $\color{green}{0.31}$ | $\color{green}{0.47}$ | $\color{green}{0.45}$ |
| speech | $\color{green}{0.39}$ | $\color{green}{0.35}$ | $\color{green}{0.45}$ | $\color{green}{0.31}$ | $\color{green}{0.37}$ | $\color{green}{0.43}$ |
| tone | $\color{green}{0.48}$ | $\color{green}{0.46}$ | $\color{green}{0.46}$ | $\color{green}{0.38}$ | $\color{green}{0.49}$ | $\color{green}{0.48}$ |
| noise | $\color{green}{0.32}$ | $\color{green}{0.31}$ | $\color{green}{0.35}$ | $\color{green}{0.25}$ | $\color{green}{0.30}$ | $\color{green}{0.37}$ |
| silence | $\color{green}{0.08}$ | $\color{green}{0.07}$ | $\color{green}{0.10}$ | $\color{green}{0.07}$ | $\color{green}{0.08}$ | $\color{green}{0.07}$ |

### Bytes a block, this encoder over libFLAC

Under 1.00 is a smaller frame than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | 1.00 | $\color{red}{1.02}$ | $\color{green}{0.95}$ | 1.00 | $\color{green}{0.99}$ | $\color{red}{1.05}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, against libFLAC at its two ends

### Time a block, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, which fits no linear predictor and codes stereo as it comes.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.85}$ | $\color{green}{0.81}$ | $\color{red}{1.09}$ | $\color{green}{0.54}$ | $\color{red}{1.07}$ | $\color{red}{1.01}$ |
| speech | $\color{green}{0.70}$ | $\color{green}{0.60}$ | $\color{green}{0.85}$ | $\color{green}{0.56}$ | $\color{green}{0.73}$ | $\color{green}{0.76}$ |
| tone | $\color{red}{1.09}$ | $\color{green}{0.97}$ | $\color{red}{1.09}$ | $\color{green}{0.71}$ | $\color{red}{1.11}$ | $\color{red}{1.10}$ |
| noise | $\color{green}{0.71}$ | $\color{green}{0.60}$ | $\color{green}{0.77}$ | $\color{green}{0.47}$ | $\color{green}{0.67}$ | $\color{green}{0.77}$ |
| silence | $\color{green}{0.15}$ | $\color{green}{0.15}$ | $\color{green}{0.22}$ | $\color{green}{0.13}$ | $\color{green}{0.15}$ | $\color{green}{0.14}$ |

### Bytes a block, this encoder over libFLAC at level 8

Under 1.00 is a smaller frame than libFLAC's smallest level, which tries every order to 12 and every stereo pairing.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.01}$ | $\color{red}{1.01}$ | 1.00 | $\color{red}{1.01}$ | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | $\color{red}{1.02}$ | $\color{red}{1.03}$ | $\color{red}{1.07}$ | 1.00 | $\color{green}{0.99}$ | $\color{red}{1.06}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, each shape apart

libFLAC is at level 5. The time is one block's, the least of five passes;
a block is 20 ms of sound, so a share of one core is that time over 20 ms.

### 48 kHz stereo 16-bit, 960 frames a block, 3840 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 46.2 | $\color{green}{17.6}$ | $\color{green}{0.38}$ | 0.09% | 1838 | 1838 | 1.00 | 47.9% |
| speech | 37.3 | $\color{green}{14.4}$ | $\color{green}{0.39}$ | 0.07% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 43.4 | $\color{green}{20.9}$ | $\color{green}{0.48}$ | 0.10% | 1008 | 1008 | 1.00 | 26.2% |
| noise | 44.5 | $\color{green}{14.1}$ | $\color{green}{0.32}$ | 0.07% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 24.8 | $\color{green}{1.9}$ | $\color{green}{0.08}$ | 0.01% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 38.4 | $\color{green}{15.1}$ | $\color{green}{0.39}$ | 0.08% | 1702 | 1702 | 1.00 | 48.2% |
| speech | 33.1 | $\color{green}{11.5}$ | $\color{green}{0.35}$ | 0.06% | 1063 | 1063 | 1.00 | 30.1% |
| tone | 37.0 | $\color{green}{17.1}$ | $\color{green}{0.46}$ | 0.09% | 920 | $\color{red}{940}$ | $\color{red}{1.02}$ | 26.6% |
| noise | 36.4 | $\color{green}{11.3}$ | $\color{green}{0.31}$ | 0.06% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 23.8 | $\color{green}{1.7}$ | $\color{green}{0.07}$ | 0.01% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 74.7 | $\color{green}{34.0}$ | $\color{green}{0.46}$ | 0.17% | 3454 | 3454 | 1.00 | 45.0% |
| speech | 62.3 | $\color{green}{27.8}$ | $\color{green}{0.45}$ | 0.14% | 2226 | 2226 | 1.00 | 29.0% |
| tone | 75.0 | $\color{green}{34.5}$ | $\color{green}{0.46}$ | 0.17% | 1651 | $\color{green}{1576}$ | $\color{green}{0.95}$ | 20.5% |
| noise | 72.9 | $\color{green}{25.8}$ | $\color{green}{0.35}$ | 0.13% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 42.4 | $\color{green}{4.1}$ | $\color{green}{0.10}$ | 0.02% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 23.9 | $\color{green}{7.3}$ | $\color{green}{0.31}$ | 0.04% | 950 | 950 | 1.00 | 49.5% |
| speech | 25.1 | $\color{green}{7.9}$ | $\color{green}{0.31}$ | 0.04% | 1148 | 1148 | 1.00 | 59.8% |
| tone | 24.0 | $\color{green}{9.1}$ | $\color{green}{0.38}$ | 0.05% | 512 | 512 | 1.00 | 26.7% |
| noise | 24.7 | $\color{green}{6.3}$ | $\color{green}{0.25}$ | 0.03% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 17.5 | $\color{green}{1.3}$ | $\color{green}{0.07}$ | 0.01% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 42.2 | $\color{green}{20.1}$ | $\color{green}{0.47}$ | 0.10% | 456 | 456 | 1.00 | 23.7% |
| speech | 37.1 | $\color{green}{13.8}$ | $\color{green}{0.37}$ | 0.07% | 250 | 250 | 1.00 | 13.0% |
| tone | 40.8 | $\color{green}{20.1}$ | $\color{green}{0.49}$ | 0.10% | 476 | $\color{green}{471}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 41.7 | $\color{green}{12.7}$ | $\color{green}{0.30}$ | 0.06% | 1002 | $\color{red}{1006}$ | 1.00 | 52.4% |
| silence | 24.7 | $\color{green}{1.9}$ | $\color{green}{0.08}$ | 0.01% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 47.5 | $\color{green}{21.6}$ | $\color{green}{0.45}$ | 0.11% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 40.0 | $\color{green}{17.2}$ | $\color{green}{0.43}$ | 0.09% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 47.0 | $\color{green}{22.6}$ | $\color{green}{0.48}$ | 0.11% | 2321 | $\color{red}{2440}$ | $\color{red}{1.05}$ | 42.4% |
| noise | 45.5 | $\color{green}{16.7}$ | $\color{green}{0.37}$ | 0.08% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 25.7 | $\color{green}{1.8}$ | $\color{green}{0.07}$ | 0.01% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{green}{0.51}$ | 1.00 | $\color{red}{1.17}$ | $\color{green}{0.95}$ | $\color{green}{0.18}$ | 1.00 |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.44}$ | 1.00 | $\color{red}{1.04}$ | $\color{green}{0.87}$ | $\color{green}{0.17}$ | $\color{red}{1.01}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.47}$ | 1.00 | $\color{red}{1.01}$ | $\color{green}{0.78}$ | $\color{green}{0.17}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.51}$ | 1.00 | $\color{red}{1.11}$ | $\color{green}{0.96}$ | $\color{green}{0.18}$ | 1.00 |
| Mix at −13 LUFS, wide stereo | 44.1 kHz stereo 16-bit | $\color{green}{0.41}$ | 1.00 | $\color{green}{0.81}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Mix at −9 LUFS, loud master | 44.1 kHz stereo 16-bit | $\color{green}{0.40}$ | 1.00 | $\color{green}{0.81}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | $\color{green}{0.59}$ | 1.00 | $\color{red}{1.28}$ | $\color{green}{0.96}$ | $\color{green}{0.23}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.41}$ | 1.00 | $\color{green}{0.85}$ | $\color{green}{0.88}$ | $\color{green}{0.18}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.42}$ | 1.00 | $\color{green}{0.83}$ | $\color{green}{0.78}$ | $\color{green}{0.18}$ | 1.00 |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.31}$ | 1.00 | $\color{green}{0.53}$ | $\color{green}{0.97}$ | $\color{green}{0.23}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.41}$ | 1.00 | $\color{green}{0.74}$ | $\color{green}{0.49}$ | $\color{green}{0.16}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.39}$ | 1.00 | $\color{green}{0.81}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.56}$ | 1.00 | $\color{red}{1.15}$ | $\color{green}{0.99}$ | $\color{green}{0.21}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.37}$ | 1.00 | $\color{green}{0.75}$ | $\color{green}{0.99}$ | $\color{green}{0.14}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 82.1 | $\color{green}{42.2}$ | $\color{green}{0.51}$ | 141 | $\color{green}{83.0}$ | 491 | $\color{green}{263}$ | 0.21% |
| Chopin, Fantasy Op. 49, piano | 76.0 | $\color{green}{33.7}$ | $\color{green}{0.44}$ | 135 | $\color{green}{63.4}$ | 431 | $\color{green}{226}$ | 0.17% |
| Shostakovich 5, orchestra, live | 44.2 | $\color{green}{20.9}$ | $\color{green}{0.47}$ | 77.0 | $\color{green}{43.2}$ | 204 | $\color{green}{141}$ | 0.10% |
| Indie rock, 2022, mastered | 50.0 | $\color{green}{25.7}$ | $\color{green}{0.51}$ | 86.0 | $\color{green}{49.1}$ | 264 | $\color{green}{222}$ | 0.13% |
| Mix at −13 LUFS, wide stereo | 38.8 | $\color{green}{15.9}$ | $\color{green}{0.41}$ | 71.8 | $\color{green}{31.9}$ | 205 | $\color{green}{133}$ | 0.08% |
| Mix at −9 LUFS, loud master | 40.0 | $\color{green}{16.1}$ | $\color{green}{0.40}$ | 71.8 | $\color{green}{32.4}$ | 198 | $\color{green}{113}$ | 0.08% |
| Limehouse Blues, 1924 78 rpm | 85.8 | $\color{green}{50.6}$ | $\color{green}{0.59}$ | 141 | $\color{green}{97.6}$ | 318 | $\color{green}{238}$ | 0.25% |
| Prison work song, 1947 field recording | 38.6 | $\color{green}{15.9}$ | $\color{green}{0.41}$ | 63.7 | $\color{green}{31.8}$ | 286 | $\color{green}{141}$ | 0.08% |
| The Raven, read, produced | 38.6 | $\color{green}{16.1}$ | $\color{green}{0.42}$ | 71.3 | $\color{green}{32.4}$ | 254 | $\color{green}{122}$ | 0.08% |
| Leviticus 13, read | 23.0 | $\color{green}{7.2}$ | $\color{green}{0.31}$ | 42.7 | $\color{green}{16.1}$ | 204 | $\color{green}{132}$ | 0.04% |
| JFK at Rice, 1962 | 41.3 | $\color{green}{16.7}$ | $\color{green}{0.41}$ | 80.5 | $\color{green}{33.8}$ | 648 | $\color{green}{141}$ | 0.08% |
| Train station, field recording | 38.6 | $\color{green}{15.2}$ | $\color{green}{0.39}$ | 63.1 | $\color{green}{32.7}$ | 144 | $\color{green}{102}$ | 0.08% |
| Pink noise | 48.4 | $\color{green}{26.9}$ | $\color{green}{0.56}$ | 93.2 | $\color{green}{47.2}$ | 308 | $\color{green}{228}$ | 0.13% |
| White noise | 46.9 | $\color{green}{17.4}$ | $\color{green}{0.37}$ | 82.9 | $\color{green}{34.7}$ | 273 | $\color{green}{142}$ | 0.09% |

### The bytes, libFLAC at level 5

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5317}$ | $\color{red}{1.001}$ | 46.2% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2482}$ | $\color{red}{1.002}$ | 32.3% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1499}$ | 1.000 | 39.0% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{red}{4556}$ | 1.000 | 79.1% |
| Mix at −13 LUFS, wide stereo | 250 | 3528 | 2325 | $\color{red}{2327}$ | $\color{red}{1.001}$ | 66.0% |
| Mix at −9 LUFS, loud master | 225 | 3528 | 2543 | $\color{red}{2547}$ | $\color{red}{1.001}$ | 72.2% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | $\color{green}{8657}$ | 1.000 | 75.1% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | $\color{red}{1421}$ | $\color{red}{1.001}$ | 40.3% |
| The Raven, read, produced | 559 | 3528 | 1362 | $\color{red}{1364}$ | $\color{red}{1.001}$ | 38.6% |
| Leviticus 13, read | 725 | 1764 | 986 | $\color{red}{987}$ | 1.000 | 55.9% |
| JFK at Rice, 1962 | 836 | 5760 | 1798 | $\color{green}{1797}$ | 1.000 | 31.2% |
| Train station, field recording | 141 | 3528 | 1272 | $\color{red}{1273}$ | 1.000 | 36.1% |
| Pink noise | 600 | 5760 | 4901 | $\color{green}{4900}$ | 1.000 | 85.1% |
| White noise | 600 | 5760 | 5387 | 5387 | 1.000 | 93.5% |

## On arm64

The same run on an arm64 machine, beside the x86 run above: this encoder's
time a block over libFLAC's on each. A ratio under 1.00 is less of a core
than libFLAC on that machine, and a block's µs are that machine's.

### Time a block on arm64, this encoder over libFLAC at level 5

Under 1.00 is less of a core than libFLAC at its default level, on arm64.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.37}$ | $\color{green}{0.35}$ | $\color{green}{0.38}$ | $\color{green}{0.35}$ | $\color{green}{0.46}$ | $\color{green}{0.40}$ |
| speech | $\color{green}{0.35}$ | $\color{green}{0.32}$ | $\color{green}{0.38}$ | $\color{green}{0.37}$ | $\color{green}{0.36}$ | $\color{green}{0.37}$ |
| tone | $\color{green}{0.46}$ | $\color{green}{0.42}$ | $\color{green}{0.43}$ | $\color{green}{0.44}$ | $\color{green}{0.47}$ | $\color{green}{0.40}$ |
| noise | $\color{green}{0.29}$ | $\color{green}{0.27}$ | $\color{green}{0.31}$ | $\color{green}{0.28}$ | $\color{green}{0.27}$ | $\color{green}{0.32}$ |
| silence | $\color{green}{0.08}$ | $\color{green}{0.08}$ | $\color{green}{0.09}$ | $\color{green}{0.09}$ | $\color{green}{0.08}$ | $\color{green}{0.07}$ |

### Time a block on arm64, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on arm64.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.13}$ | $\color{red}{1.01}$ | $\color{red}{1.27}$ | $\color{green}{0.79}$ | $\color{red}{1.44}$ | $\color{red}{1.14}$ |
| speech | $\color{green}{0.88}$ | $\color{green}{0.76}$ | $\color{green}{0.97}$ | $\color{green}{0.82}$ | $\color{green}{0.95}$ | $\color{green}{0.83}$ |
| tone | $\color{red}{1.41}$ | $\color{red}{1.23}$ | $\color{red}{1.38}$ | $\color{red}{1.02}$ | $\color{red}{1.47}$ | $\color{red}{1.14}$ |
| noise | $\color{green}{0.79}$ | $\color{green}{0.70}$ | $\color{green}{0.87}$ | $\color{green}{0.60}$ | $\color{green}{0.82}$ | $\color{green}{0.81}$ |
| silence | $\color{green}{0.19}$ | $\color{green}{0.19}$ | $\color{green}{0.21}$ | $\color{green}{0.19}$ | $\color{green}{0.19}$ | $\color{green}{0.16}$ |

### The made sounds at 48 kHz stereo 16-bit, machine by machine

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 |
|---|---:|---:|---:|---:|---:|---:|
| music | 46.2 | 47.0 | 17.6 | 17.2 | $\color{green}{0.38}$ | $\color{green}{0.37}$ |
| speech | 37.3 | 39.1 | 14.4 | 13.9 | $\color{green}{0.39}$ | $\color{green}{0.35}$ |
| tone | 43.4 | 44.9 | 20.9 | 20.6 | $\color{green}{0.48}$ | $\color{green}{0.46}$ |
| noise | 44.5 | 43.4 | 14.1 | 12.5 | $\color{green}{0.32}$ | $\color{green}{0.29}$ |
| silence | 24.8 | 23.6 | 1.9 | 2.0 | $\color{green}{0.08}$ | $\color{green}{0.08}$ |

### The recordings, machine by machine

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 | 99th, x86 | 99th, arm64 | Time vs 0, x86 | Time vs 0, arm64 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 82.1 | 94.0 | 42.2 | 38.7 | $\color{green}{0.51}$ | $\color{green}{0.41}$ | 83.0 | 64.6 | $\color{red}{1.17}$ | $\color{red}{1.17}$ |
| Chopin, Fantasy Op. 49, piano | 76.0 | 86.4 | 33.7 | 33.3 | $\color{green}{0.44}$ | $\color{green}{0.39}$ | 63.4 | 56.7 | $\color{red}{1.04}$ | $\color{red}{1.24}$ |
| Shostakovich 5, orchestra, live | 44.2 | 46.4 | 20.9 | 19.8 | $\color{green}{0.47}$ | $\color{green}{0.43}$ | 43.2 | 29.7 | $\color{red}{1.01}$ | $\color{red}{1.23}$ |
| Indie rock, 2022, mastered | 50.0 | 53.7 | 25.7 | 23.1 | $\color{green}{0.51}$ | $\color{green}{0.43}$ | 49.1 | 37.5 | $\color{red}{1.11}$ | $\color{red}{1.13}$ |
| Mix at −13 LUFS, wide stereo | 38.8 | 42.3 | 15.9 | 15.8 | $\color{green}{0.41}$ | $\color{green}{0.37}$ | 31.9 | 24.5 | $\color{green}{0.81}$ | $\color{red}{1.01}$ |
| Mix at −9 LUFS, loud master | 40.0 | 42.2 | 16.1 | 15.7 | $\color{green}{0.40}$ | $\color{green}{0.37}$ | 32.4 | 24.6 | $\color{green}{0.81}$ | $\color{green}{0.99}$ |
| Limehouse Blues, 1924 78 rpm | 85.8 | 97.2 | 50.6 | 44.7 | $\color{green}{0.59}$ | $\color{green}{0.46}$ | 97.6 | 65.4 | $\color{red}{1.28}$ | $\color{red}{1.21}$ |
| Prison work song, 1947 field recording | 38.6 | 41.5 | 15.9 | 15.5 | $\color{green}{0.41}$ | $\color{green}{0.37}$ | 31.8 | 22.8 | $\color{green}{0.85}$ | $\color{red}{1.06}$ |
| The Raven, read, produced | 38.6 | 41.5 | 16.1 | 15.9 | $\color{green}{0.42}$ | $\color{green}{0.38}$ | 32.4 | 24.1 | $\color{green}{0.83}$ | $\color{red}{1.04}$ |
| Leviticus 13, read | 23.0 | 19.6 | 7.2 | 7.0 | $\color{green}{0.31}$ | $\color{green}{0.35}$ | 16.1 | 8.4 | $\color{green}{0.53}$ | $\color{green}{0.75}$ |
| JFK at Rice, 1962 | 41.3 | 43.2 | 16.7 | 15.8 | $\color{green}{0.41}$ | $\color{green}{0.36}$ | 33.8 | 17.6 | $\color{green}{0.74}$ | $\color{green}{0.81}$ |
| Train station, field recording | 38.6 | 40.7 | 15.2 | 14.7 | $\color{green}{0.39}$ | $\color{green}{0.36}$ | 32.7 | 23.1 | $\color{green}{0.81}$ | $\color{red}{1.01}$ |
| Pink noise | 48.4 | 50.4 | 26.9 | 24.2 | $\color{green}{0.56}$ | $\color{green}{0.48}$ | 47.2 | 36.2 | $\color{red}{1.15}$ | $\color{red}{1.19}$ |
| White noise | 46.9 | 49.8 | 17.4 | 16.0 | $\color{green}{0.37}$ | $\color{green}{0.32}$ | 34.7 | 28.3 | $\color{green}{0.75}$ | $\color{green}{0.79}$ |

The bytes a block of this encoder are the same on both machines, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 5 differ between the two machines on tone at 48 kHz stereo 24-bit (2321 against 2315).

The bytes a block of libFLAC at level 0 are the same on both machines, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 8 differ between the two machines on tone at 48 kHz stereo 24-bit (2302 against 2502).

