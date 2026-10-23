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
and twelve recordings from archive.org, public domain or CC0, whole, in
blocks of 20 ms. A made sound's time is one block's, the least of five passes over the
fifty, each pass timed as a whole after one to warm up. A recording's is the
least of three passes' mean block, each block timed on its own, and with it
the least of the passes' 99th-percentile and worst blocks, the latency a
stream sees; its bytes are the frames' over the recording, a block's share.
`tools/tables.py` makes the tables below from the log:

```sh
tools/samples.sh copy      # the recordings into tmp/samples, from the store
cargo run --release --manifest-path tools/bench/Cargo.toml > bench.log
python3 -I tools/tables.py bench.log
```

They were run on 2026-10-23 on a six-core Intel i5-8500T under WSL2, one
thread, the machine idle, with rustc 1.98.1 at the x86-64 baseline (SSE2, no
`target-cpu`), on branch `own-encoder` at the commit that carries them: the
encoder with its stereo pair chosen on the fit as well as the fixed
predictors, the fit skipped on noise, and the time of a block cut by a
quarter to a half, the night's work of [plan.md](plan.md). A time on this machine moves by up to a
tenth from one run to the next, so a time ratio within a tenth of 1.00 is on
the line; a bytes ratio is exact. A worst block is one block of thousands
and the machine's as much as the encoder's, even as the least over passes,
so it is read as an order of magnitude. The bench also logs `worst_at`, the
block that was slowest at its least over the passes, and dumps every
block's least time and bytes when asked (`BLOCKS=dir`), which is how the
late blocks below were read.

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
the repository holds a recording.

| Recording | Item | Shape | Seconds | What it is |
|---|---|---|---:|---|
| Bach, Goldberg Aria, piano | `OpenGoldbergVariations` | 96 kHz stereo 24-bit | 299 | Kimiko Ishizaka's studio recording, CC0: a quiet piano, long decays |
| Chopin, Fantasy Op. 49, piano | `musopen-chopin-complete-works-flac` | 96 kHz stereo 16-bit | 738 | Musopen's recording, CC0: piano from quiet to loud |
| Shostakovich 5, orchestra, live | `dso20060204old` | 48 kHz stereo 16-bit | 324 | a concert recording, one movement, the hall in it |
| Indie rock, 2022, mastered | `1.-golf-club-stay-awhile` | 48 kHz stereo 24-bit | 224 | Golf Club's "Stay Awhile", CC0 by the band: guitars, drums and voices in a loud modern master, the one popular record |
| Limehouse Blues, 1924 78 rpm | `TheColumbiansCollection1924-1929DirectedByBenSelvin` | 96 kHz stereo 24-bit | 197 | a 78 rpm transfer: dance band under surface noise |
| Prison work song, 1947 field recording | `negroprisonsongs` | 44.1 kHz stereo 16-bit | 252 | Lomax's Parchman recording: voices and axes outdoors |
| The Raven, read, produced | `TheRavenByEdgarAllanPoe` | 44.1 kHz stereo 16-bit | 559 | a radio theatre reading with music and effects |
| Leviticus 13, read | `LeviticusKJV` | 44.1 kHz mono 16-bit | 725 | one voice reading, a room's quiet between verses |
| JFK at Rice, 1962 | `NasaAudioHighlightReels` | 48 kHz stereo 24-bit | 836 | a speech through a public-address system, a crowd |
| Train station, field recording | `aporee_53029_60598` | 44.1 kHz stereo 16-bit | 141 | a passenger train arriving, the station's ambience |
| Pink noise | `TenMinutesOfWhiteNoisePinkNoiseAndBrownianNoise` | 48 kHz stereo 24-bit | 600 | noise falling 3 dB an octave, predictable a little |
| White noise | `TenMinutesOfWhiteNoisePinkNoiseAndBrownianNoise` | 48 kHz stereo 24-bit | 600 | nothing to predict: the most a stream can cost |

## What the tables say

- **Every recording and every made sound takes less of a core than libFLAC
  at its default level, by a quarter to a half and more.** The recordings
  are 0.43 to 0.87 of its time a block: 16-bit stereo 0.66 to 0.78, mono
  0.43, 24-bit 0.47 to 0.87, the two at 96 kHz and 24 bits, which were on
  the line, 0.84 and 0.87. A block of a recording is 0.04% to 0.35% of one
  core. The made sounds are 0.41 to 0.91: music 0.46 to 0.83, speech 0.41 to
  0.51, the tone 0.47 to 0.91, noise 0.30 to 0.50, silence 0.09 to 0.15.
  Before this night the recordings were 0.46 to 1.01 and the made sounds
  0.42 to 0.94, for the same bytes within half a per cent.
- **The bytes are libFLAC's at its default level, within 0.2%.** Every
  recording is 1.000 but the two pianos at 1.001 and 1.002, and the train
  station is 0.995; before the pair was chosen on the fit it was 1.006, the
  Fantasy 1.004. Every made sound comes to the same bytes within a byte a
  block but the pure tone, 1.02 at 44.1 kHz, 0.95 at 96 kHz and 1.05 at
  24 bits: a tone is predicted away by order 2 and what its frame then
  costs is the rounding of the coefficients, which goes with every change
  to the fit's arithmetic and which no estimate tried this night could
  foresee (the measurements are in [plan.md](plan.md)). Against libFLAC's
  smallest level, 8, which tries every order to 12 and every stereo pairing
  in three to five times this encoder's time, the made sounds are 1.00 to
  1.03 but the tone at 96 kHz, 1.07, and at 24 bits, 1.06, and the
  recordings 1.00 to 1.01.
- **The late blocks are the machine's more than the encoder's.** Each
  block's least time over the passes, which the bench now dumps, puts this
  encoder's own 99th-percentile block within a quarter of its mean on every
  recording, and its worst block at 1.2 to 1.7 times the mean; a single
  pass's 99th percentile is 1.5 to 2.3 times the mean for both encoders,
  which is scheduling and cache, not the sound. What is the encoder's own is
  the loud block, writing more Rice bits, and on white noise every eighth
  block, where the fit is tried again after being skipped (33 µs against a
  mean of 21). On the recordings this encoder's 99th-percentile block is
  0.49 to 0.76 of libFLAC's, 20 to 108 µs against 40 to 136, and its worst
  block 100 to 398 µs, at most 2% of the 20 ms the block is, where
  libFLAC's worst is 181 to 944.
- **The pair is chosen on the fit as well as the fixed predictors, from
  three autocorrelations.** Every channel is coded with its fixed predictor
  first and its fit is weighed from its autocorrelation; the pair is chosen
  on each channel's real fixed bits less the bits its fit is estimated to
  save, the two chosen are coded, and the pair is chosen again with their
  real bits, until it stands. The estimate is good at differences and poor
  at levels: chosen on estimates alone the 8-bit speech was 1.63 and the
  tones 1.05 to 1.10. The mid's autocorrelation is derived from left's,
  right's and side's, so three passes serve four channels, and where a
  channel is one value the pair stands on the fixed bits and only the two
  chosen are fitted. Where the fit is estimated not to win on a channel of
  fixed order 0, which is noise and a quiet desktop, it is skipped for seven
  blocks: white noise fell from 37 to 22 µs, the made noise from 30 to 17.
- **Where the time goes at 16-bit stereo**, measured with timers in the
  encoder on the orchestra, 30 µs a block: the autocorrelations 8.3 for
  three and the fits from them 1.2; the four channels' fixed orders and
  their sums 4.5 and their partitions 3.7; the two fits' residuals 3.3, the
  side channel at 17 bits in a split 16-bit path, and their partitions 2;
  the range check and the stereo split 1.6; writing the bits 4.1, four Rice
  codes a word; the CRC 1. The order search's differences are the fixed
  residuals, so none is computed twice; the finest partitions are summed in
  64-bit lanes, since a partition of 30 samples never filled a block of 32
  and was summed scalar before. At 24 bits the residual of a fit runs in the
  same split path, two 16-bit passes, where it ran in 64-bit scalars: the
  Goldberg Aria fell from 84 to 66 µs and the 78 rpm transfer from 86 to 70.
- **Against libFLAC's fastest level, 0, which fits no predictor and codes
  stereo as it comes**, this encoder takes 1.06 to 2.09 of its time on the
  stereo recordings, 0.92 on the 1962 speech and 0.79 on the mono reading,
  for 0.49 to 0.99 of its bytes: the two stereo speeches 0.49 and 0.78, the
  music 0.78 to 0.96, noise 0.99. On the made sounds it is 1.35 to 2.37 on
  stereo music and tone, 0.74 to 1.05 on speech, 0.82 to 1.15 on noise,
  0.17 to 0.94 on mono and 0.23 to 0.36 on silence.
- **A loud modern master is most of its PCM at 24 bits.** The indie rock
  track, mixed and mastered loud in 2022 and the one popular record in the
  set, is 79% of its PCM at 48 kHz 24-bit for both encoders, where the 1962
  speech at the same shape is 31% and the orchestra at 16 bits 39%: under a
  dense mix the low bits of a 24-bit sample are the mix's own noise floor,
  which no predictor takes out and Rice writes as it is. The same track
  dithered to 16 bits, measured aside on 2026-10-22 and not in the tables,
  was 69% of its PCM, 2645 bytes a block against libFLAC's 2638 (1.003).

## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.67}$ | $\color{green}{0.64}$ | $\color{green}{0.83}$ | $\color{green}{0.46}$ | $\color{green}{0.70}$ | $\color{green}{0.72}$ |
| speech | $\color{green}{0.44}$ | $\color{green}{0.41}$ | $\color{green}{0.51}$ | $\color{green}{0.48}$ | $\color{green}{0.42}$ | $\color{green}{0.50}$ |
| tone | $\color{green}{0.78}$ | $\color{green}{0.71}$ | $\color{green}{0.91}$ | $\color{green}{0.47}$ | $\color{green}{0.76}$ | $\color{green}{0.84}$ |
| noise | $\color{green}{0.44}$ | $\color{green}{0.41}$ | $\color{green}{0.50}$ | $\color{green}{0.30}$ | $\color{green}{0.41}$ | $\color{green}{0.49}$ |
| silence | $\color{green}{0.12}$ | $\color{green}{0.12}$ | $\color{green}{0.15}$ | $\color{green}{0.09}$ | $\color{green}{0.12}$ | $\color{green}{0.12}$ |

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
| music | $\color{red}{1.56}$ | $\color{red}{1.35}$ | $\color{red}{2.08}$ | $\color{green}{0.88}$ | $\color{red}{1.65}$ | $\color{red}{1.68}$ |
| speech | $\color{green}{0.86}$ | $\color{green}{0.74}$ | $\color{red}{1.05}$ | $\color{green}{0.90}$ | $\color{green}{0.89}$ | $\color{green}{0.94}$ |
| tone | $\color{red}{1.85}$ | $\color{red}{1.55}$ | $\color{red}{2.37}$ | $\color{green}{0.94}$ | $\color{red}{1.84}$ | $\color{red}{2.00}$ |
| noise | $\color{green}{0.99}$ | $\color{green}{0.82}$ | $\color{red}{1.15}$ | $\color{green}{0.58}$ | $\color{green}{0.95}$ | $\color{red}{1.06}$ |
| silence | $\color{green}{0.24}$ | $\color{green}{0.23}$ | $\color{green}{0.36}$ | $\color{green}{0.17}$ | $\color{green}{0.24}$ | $\color{green}{0.23}$ |

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
| music | 42.2 | $\color{green}{28.3}$ | $\color{green}{0.67}$ | 0.14% | 1838 | 1838 | 1.00 | 47.9% |
| speech | 35.6 | $\color{green}{15.8}$ | $\color{green}{0.44}$ | 0.08% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 40.8 | $\color{green}{31.6}$ | $\color{green}{0.78}$ | 0.16% | 1008 | 1008 | 1.00 | 26.2% |
| noise | 40.2 | $\color{green}{17.6}$ | $\color{green}{0.44}$ | 0.09% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 24.8 | $\color{green}{3.0}$ | $\color{green}{0.12}$ | 0.02% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 36.1 | $\color{green}{23.2}$ | $\color{green}{0.64}$ | 0.12% | 1702 | 1702 | 1.00 | 48.2% |
| speech | 30.9 | $\color{green}{12.6}$ | $\color{green}{0.41}$ | 0.06% | 1063 | 1063 | 1.00 | 30.1% |
| tone | 35.3 | $\color{green}{25.2}$ | $\color{green}{0.71}$ | 0.13% | 920 | $\color{red}{940}$ | $\color{red}{1.02}$ | 26.6% |
| noise | 34.7 | $\color{green}{14.1}$ | $\color{green}{0.41}$ | 0.07% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 23.7 | $\color{green}{2.9}$ | $\color{green}{0.12}$ | 0.01% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 71.5 | $\color{green}{59.0}$ | $\color{green}{0.83}$ | 0.29% | 3454 | 3454 | 1.00 | 45.0% |
| speech | 61.1 | $\color{green}{31.1}$ | $\color{green}{0.51}$ | 0.16% | 2226 | 2226 | 1.00 | 29.0% |
| tone | 68.7 | $\color{green}{62.2}$ | $\color{green}{0.91}$ | 0.31% | 1651 | $\color{green}{1576}$ | $\color{green}{0.95}$ | 20.5% |
| noise | 68.8 | $\color{green}{34.1}$ | $\color{green}{0.50}$ | 0.17% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 43.4 | $\color{green}{6.7}$ | $\color{green}{0.15}$ | 0.03% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 22.5 | $\color{green}{10.4}$ | $\color{green}{0.46}$ | 0.05% | 950 | 950 | 1.00 | 49.5% |
| speech | 22.7 | $\color{green}{10.8}$ | $\color{green}{0.48}$ | 0.05% | 1148 | $\color{green}{1147}$ | 1.00 | 59.8% |
| tone | 22.1 | $\color{green}{10.5}$ | $\color{green}{0.47}$ | 0.05% | 512 | 512 | 1.00 | 26.7% |
| noise | 22.6 | $\color{green}{6.8}$ | $\color{green}{0.30}$ | 0.03% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 16.4 | $\color{green}{1.5}$ | $\color{green}{0.09}$ | 0.01% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 39.4 | $\color{green}{27.4}$ | $\color{green}{0.70}$ | 0.14% | 456 | 456 | 1.00 | 23.7% |
| speech | 35.5 | $\color{green}{14.9}$ | $\color{green}{0.42}$ | 0.07% | 250 | 250 | 1.00 | 13.0% |
| tone | 39.1 | $\color{green}{29.6}$ | $\color{green}{0.76}$ | 0.15% | 476 | $\color{green}{471}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 39.0 | $\color{green}{15.9}$ | $\color{green}{0.41}$ | 0.08% | 1002 | $\color{red}{1006}$ | 1.00 | 52.4% |
| silence | 25.0 | $\color{green}{3.0}$ | $\color{green}{0.12}$ | 0.02% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 44.8 | $\color{green}{32.4}$ | $\color{green}{0.72}$ | 0.16% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 38.9 | $\color{green}{19.3}$ | $\color{green}{0.50}$ | 0.10% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 44.2 | $\color{green}{37.1}$ | $\color{green}{0.84}$ | 0.19% | 2321 | $\color{red}{2440}$ | $\color{red}{1.05}$ | 42.4% |
| noise | 42.8 | $\color{green}{20.9}$ | $\color{green}{0.49}$ | 0.10% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 26.3 | $\color{green}{3.1}$ | $\color{green}{0.12}$ | 0.02% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{green}{0.84}$ | 1.00 | $\color{red}{2.09}$ | $\color{green}{0.95}$ | $\color{green}{0.30}$ | 1.00 |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.78}$ | 1.00 | $\color{red}{1.89}$ | $\color{green}{0.87}$ | $\color{green}{0.30}$ | $\color{red}{1.01}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.75}$ | 1.00 | $\color{red}{1.69}$ | $\color{green}{0.78}$ | $\color{green}{0.26}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.78}$ | 1.00 | $\color{red}{1.75}$ | $\color{green}{0.96}$ | $\color{green}{0.26}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | $\color{green}{0.87}$ | 1.00 | $\color{red}{2.01}$ | $\color{green}{0.96}$ | $\color{green}{0.33}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.67}$ | 1.00 | $\color{red}{1.43}$ | $\color{green}{0.88}$ | $\color{green}{0.30}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.70}$ | 1.00 | $\color{red}{1.48}$ | $\color{green}{0.78}$ | $\color{green}{0.30}$ | 1.00 |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.43}$ | 1.00 | $\color{green}{0.79}$ | $\color{green}{0.97}$ | $\color{green}{0.31}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.47}$ | 1.00 | $\color{green}{0.92}$ | $\color{green}{0.49}$ | $\color{green}{0.18}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.66}$ | 1.00 | $\color{red}{1.41}$ | $\color{green}{0.94}$ | $\color{green}{0.29}$ | 1.00 |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.78}$ | 1.00 | $\color{red}{1.66}$ | $\color{green}{0.99}$ | $\color{green}{0.28}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.50}$ | 1.00 | $\color{red}{1.06}$ | $\color{green}{0.99}$ | $\color{green}{0.19}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 78.7 | $\color{green}{65.7}$ | $\color{green}{0.84}$ | 136 | $\color{green}{104}$ | 944 | $\color{green}{305}$ | 0.33% |
| Chopin, Fantasy Op. 49, piano | 70.2 | $\color{green}{55.1}$ | $\color{green}{0.78}$ | 116 | $\color{green}{88.8}$ | 372 | $\color{red}{390}$ | 0.28% |
| Shostakovich 5, orchestra, live | 41.4 | $\color{green}{31.1}$ | $\color{green}{0.75}$ | 71.5 | $\color{green}{51.9}$ | 181 | $\color{green}{126}$ | 0.16% |
| Indie rock, 2022, mastered | 47.3 | $\color{green}{37.1}$ | $\color{green}{0.78}$ | 83.5 | $\color{green}{60.7}$ | 253 | $\color{green}{145}$ | 0.19% |
| Limehouse Blues, 1924 78 rpm | 80.6 | $\color{green}{70.4}$ | $\color{green}{0.87}$ | 131 | $\color{green}{108}$ | 423 | $\color{green}{234}$ | 0.35% |
| Prison work song, 1947 field recording | 36.1 | $\color{green}{24.3}$ | $\color{green}{0.67}$ | 60.5 | $\color{green}{41.1}$ | 191 | $\color{green}{167}$ | 0.12% |
| The Raven, read, produced | 36.8 | $\color{green}{25.7}$ | $\color{green}{0.70}$ | 66.1 | $\color{green}{44.7}$ | 433 | $\color{green}{227}$ | 0.13% |
| Leviticus 13, read | 20.9 | $\color{green}{9.0}$ | $\color{green}{0.43}$ | 39.5 | $\color{green}{20.1}$ | 336 | $\color{green}{100}$ | 0.04% |
| JFK at Rice, 1962 | 38.6 | $\color{green}{18.3}$ | $\color{green}{0.47}$ | 70.7 | $\color{green}{35.2}$ | 270 | $\color{red}{398}$ | 0.09% |
| Train station, field recording | 36.2 | $\color{green}{23.7}$ | $\color{green}{0.66}$ | 58.6 | $\color{green}{40.0}$ | 213 | $\color{green}{109}$ | 0.12% |
| Pink noise | 44.7 | $\color{green}{35.0}$ | $\color{green}{0.78}$ | 78.0 | $\color{green}{58.9}$ | 266 | $\color{green}{258}$ | 0.17% |
| White noise | 44.3 | $\color{green}{22.1}$ | $\color{green}{0.50}$ | 78.2 | $\color{green}{42.1}$ | 293 | $\color{green}{223}$ | 0.11% |

### The bytes, libFLAC at level 5

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5316}$ | $\color{red}{1.001}$ | 46.2% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2482}$ | $\color{red}{1.002}$ | 32.3% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1499}$ | 1.000 | 39.0% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{green}{4554}$ | 1.000 | 79.1% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | $\color{green}{8657}$ | 1.000 | 75.1% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | 1420 | 1.000 | 40.2% |
| The Raven, read, produced | 559 | 3528 | 1362 | 1362 | 1.000 | 38.6% |
| Leviticus 13, read | 725 | 1764 | 986 | 986 | 1.000 | 55.9% |
| JFK at Rice, 1962 | 836 | 5760 | 1798 | $\color{green}{1797}$ | 1.000 | 31.2% |
| Train station, field recording | 141 | 3528 | 1272 | $\color{red}{1273}$ | 1.000 | 36.1% |
| Pink noise | 600 | 5760 | 4901 | $\color{green}{4900}$ | 1.000 | 85.1% |
| White noise | 600 | 5760 | 5387 | 5387 | 1.000 | 93.5% |

