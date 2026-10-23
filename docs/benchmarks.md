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

They were run on 2026-10-22 on a six-core Intel i5-8500T under WSL2, one
thread, the machine idle, with rustc 1.98.1 at the x86-64 baseline (SSE2, no
`target-cpu`), on commit e3173f7 of branch `own-encoder`, the encoder with
its linear predictor of up to order 8. A time on this machine moves by up to a
tenth from one run to the next, so a time ratio within a tenth of 1.00 is on
the line; a bytes ratio is exact. A worst block is one block of thousands
and the machine's as much as the encoder's, even as the least over passes,
so it is read as an order of magnitude.

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
  at its default level.** The recordings are 0.40 to 0.94 of its time a
  block: 16-bit stereo 0.57 to 0.71, mono 0.40, 24-bit 0.59 to 0.94, the
  two at 96 kHz and 24 bits the nearest to the line and the indie rock
  master at 48 kHz and 24 bits next, 0.86. A block of a recording is 0.05%
  to 0.42% of one core. The made sounds are 0.40 to
  0.84: 16-bit stereo 0.40 to 0.74, mono 0.44 to 0.46, 8-bit 0.47 to 0.66,
  24-bit 0.56 to 0.84; silence 0.09 to 0.21.
- **The late blocks are earlier too.** On the recordings this encoder's
  99th-percentile block is 0.48 to 0.93 of libFLAC's, 30 to 177 µs against
  63 to 191, and its worst block 140 to 399 µs, at most 2% of the 20 ms the
  block is, where libFLAC's worst is 183 to 571. A 99th-percentile block is
  1.8 to 3.0 times the mean for both encoders: the sound's, not the
  encoder's, the loud and the busy blocks costing more Rice bits to write.
- **The bytes are libFLAC's at its default level, within 1.1%.** Every
  made sound comes to the same bytes within a byte a block but music at
  96 kHz, six bytes over of 3454, and the tone, 0.94 to 1.00 of them at
  five shapes and 1.07 at 96 kHz. Eight recordings are within 0.2%; the two
  pianos at 96 kHz are 1.011 and 1.010, the Raven 1.006 and the train
  1.007. Against libFLAC's smallest level, 8, which tries every order to 12
  and every stereo pairing in 1.6 to 3.6 times the default's time (1.3 to
  2.8 on the recordings), the made sounds are 1.00 to 1.01 and the
  recordings 1.00 to 1.02.
- **What is over the line is the fit's single precision.** The fit's
  autocorrelation is summed in `f32` lanes, 1.2 µs a channel where `f64`
  lanes are 2.5 to 3, and on a long block of a quiet, resonant signal its
  rounding misfits the predictor: the two 96 kHz pianos, the Raven and the
  made tone at 96 kHz. Summed in double for one run, the Goldberg Aria is
  1.002 of libFLAC's bytes in place of 1.011, the Fantasy 1.004 in place of
  1.010, the Raven 1.001 in place of 1.006 and the made tone 0.95 in place
  of 1.07, for 8 µs more a block at 48 kHz and 20 at 96 kHz; the train stays
  at 1.006, which is not precision. Taking the precision back without the
  time is the first item of [plan.md](plan.md).
- **Against libFLAC's fastest level, 0, which fits no predictor and codes
  stereo as it comes**, this encoder takes 1.08 to 2.20 of its time on the
  stereo recordings and 0.69 on the mono one, for 0.49 to 0.99 of its bytes:
  the two stereo speeches 0.49 and 0.79, the music 0.78 to 0.97, noise
  0.99. On the made
  sounds it is 1.07 to 1.88 on stereo music, tone and noise, 0.78 to 1.13
  on speech, 0.86 to 0.90 on mono and 0.16 to 0.46 on silence.
- **A loud modern master is most of its PCM at 24 bits.** The indie rock
  track, mixed and mastered loud in 2022 and the one popular record in the
  set, is 79% of its PCM at 48 kHz 24-bit for both encoders, where the 1962
  speech at the same shape is 31% and the orchestra at 16 bits 39%: under a
  dense mix the low bits of a 24-bit sample are the mix's own noise floor,
  which no predictor takes out and Rice writes as it is. The same track
  dithered to 16 bits, measured aside and not in the tables, is 69% of its
  PCM, 2645 bytes a block against libFLAC's 2638 (1.003), in 0.64 of its
  time where at 24 bits it is 0.86: what a 24-bit capture of such a record
  pays over a 16-bit one, a third more bytes and the 32-bit residual lanes,
  buys the noise floor.
- **Where the time goes at 16-bit stereo**, measured when the fit was
  written: without it a block of music took 24 µs, speech 18, noise 22; the
  fit is 1.5 µs a channel for the two channels written, 1.2 of it the
  autocorrelation, its residual 1.1 µs a channel through 16-bit lanes and
  its partitions about 1. Noise pays the autocorrelation before the fit is
  declined, so noise costs more than speech, whose fit is taken. At 24 bits
  the residual runs in 32-bit lanes, 3 µs a channel, which is the 0.84 on
  made music there, the 0.92 on the 78 rpm transfer and the 0.86 on the
  indie rock master.

## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.65}$ | $\color{green}{0.61}$ | $\color{green}{0.74}$ | $\color{green}{0.46}$ | $\color{green}{0.65}$ | $\color{green}{0.84}$ |
| speech | $\color{green}{0.46}$ | $\color{green}{0.40}$ | $\color{green}{0.54}$ | $\color{green}{0.44}$ | $\color{green}{0.47}$ | $\color{green}{0.56}$ |
| tone | $\color{green}{0.67}$ | $\color{green}{0.62}$ | $\color{green}{0.71}$ | $\color{green}{0.45}$ | $\color{green}{0.66}$ | $\color{green}{0.67}$ |
| noise | $\color{green}{0.60}$ | $\color{green}{0.58}$ | $\color{green}{0.72}$ | $\color{green}{0.45}$ | $\color{green}{0.60}$ | $\color{green}{0.71}$ |
| silence | $\color{green}{0.18}$ | $\color{green}{0.17}$ | $\color{green}{0.21}$ | $\color{green}{0.09}$ | $\color{green}{0.18}$ | $\color{green}{0.17}$ |

### Bytes a block, this encoder over libFLAC

Under 1.00 is a smaller frame than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | $\color{green}{0.97}$ | $\color{green}{0.95}$ | $\color{red}{1.07}$ | 1.00 | $\color{green}{0.99}$ | $\color{green}{0.94}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, against libFLAC at its two ends

### Time a block, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, which fits no linear predictor and codes stereo as it comes.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.17}$ | $\color{red}{1.29}$ | $\color{red}{1.88}$ | $\color{green}{0.87}$ | $\color{red}{1.52}$ | $\color{red}{1.86}$ |
| speech | $\color{green}{0.89}$ | $\color{green}{0.78}$ | $\color{red}{1.13}$ | $\color{green}{0.87}$ | $\color{green}{0.97}$ | $\color{red}{1.07}$ |
| tone | $\color{red}{1.58}$ | $\color{red}{1.34}$ | $\color{red}{1.71}$ | $\color{green}{0.90}$ | $\color{red}{1.55}$ | $\color{red}{1.64}$ |
| noise | $\color{red}{1.35}$ | $\color{red}{1.18}$ | $\color{red}{1.68}$ | $\color{green}{0.86}$ | $\color{red}{1.50}$ | $\color{red}{1.52}$ |
| silence | $\color{green}{0.36}$ | $\color{green}{0.33}$ | $\color{green}{0.46}$ | $\color{green}{0.16}$ | $\color{green}{0.35}$ | $\color{green}{0.33}$ |

### Bytes a block, this encoder over libFLAC at level 8

Under 1.00 is a smaller frame than libFLAC's smallest level, which tries every order to 12 and every stereo pairing.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.01}$ | $\color{red}{1.01}$ | 1.00 | $\color{red}{1.01}$ | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | $\color{green}{0.99}$ | $\color{green}{0.95}$ | $\color{red}{1.19}$ | 1.00 | $\color{green}{0.99}$ | $\color{green}{0.95}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, each shape apart

libFLAC is at level 5. The time is one block's, the least of five passes;
a block is 20 ms of sound, so a share of one core is that time over 20 ms.

### 48 kHz stereo 16-bit, 960 frames a block, 3840 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 43.9 | $\color{green}{28.4}$ | $\color{green}{0.65}$ | 0.14% | 1838 | $\color{red}{1839}$ | 1.00 | 47.9% |
| speech | 38.8 | $\color{green}{17.9}$ | $\color{green}{0.46}$ | 0.09% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 43.7 | $\color{green}{29.4}$ | $\color{green}{0.67}$ | 0.15% | 1008 | $\color{green}{982}$ | $\color{green}{0.97}$ | 25.6% |
| noise | 44.6 | $\color{green}{26.9}$ | $\color{green}{0.60}$ | 0.13% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 27.2 | $\color{green}{4.9}$ | $\color{green}{0.18}$ | 0.02% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 39.4 | $\color{green}{24.0}$ | $\color{green}{0.61}$ | 0.12% | 1702 | 1702 | 1.00 | 48.3% |
| speech | 35.4 | $\color{green}{14.3}$ | $\color{green}{0.40}$ | 0.07% | 1063 | 1063 | 1.00 | 30.1% |
| tone | 37.8 | $\color{green}{23.4}$ | $\color{green}{0.62}$ | 0.12% | 920 | $\color{green}{873}$ | $\color{green}{0.95}$ | 24.7% |
| noise | 38.0 | $\color{green}{22.1}$ | $\color{green}{0.58}$ | 0.11% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 26.1 | $\color{green}{4.5}$ | $\color{green}{0.17}$ | 0.02% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 78.1 | $\color{green}{57.4}$ | $\color{green}{0.74}$ | 0.29% | 3454 | $\color{red}{3460}$ | 1.00 | 45.1% |
| speech | 66.0 | $\color{green}{35.7}$ | $\color{green}{0.54}$ | 0.18% | 2226 | 2226 | 1.00 | 29.0% |
| tone | 70.6 | $\color{green}{50.3}$ | $\color{green}{0.71}$ | 0.25% | 1651 | $\color{red}{1764}$ | $\color{red}{1.07}$ | 23.0% |
| noise | 74.9 | $\color{green}{53.9}$ | $\color{green}{0.72}$ | 0.27% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 47.0 | $\color{green}{9.9}$ | $\color{green}{0.21}$ | 0.05% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 23.7 | $\color{green}{11.0}$ | $\color{green}{0.46}$ | 0.05% | 950 | 950 | 1.00 | 49.5% |
| speech | 25.2 | $\color{green}{11.2}$ | $\color{green}{0.44}$ | 0.06% | 1148 | 1148 | 1.00 | 59.8% |
| tone | 23.4 | $\color{green}{10.4}$ | $\color{green}{0.45}$ | 0.05% | 512 | 512 | 1.00 | 26.7% |
| noise | 23.7 | $\color{green}{10.8}$ | $\color{green}{0.45}$ | 0.05% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 17.9 | $\color{green}{1.5}$ | $\color{green}{0.09}$ | 0.01% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 43.8 | $\color{green}{28.4}$ | $\color{green}{0.65}$ | 0.14% | 456 | 456 | 1.00 | 23.7% |
| speech | 37.6 | $\color{green}{17.9}$ | $\color{green}{0.47}$ | 0.09% | 250 | 250 | 1.00 | 13.0% |
| tone | 41.8 | $\color{green}{27.4}$ | $\color{green}{0.66}$ | 0.14% | 476 | $\color{green}{471}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 43.7 | $\color{green}{26.4}$ | $\color{green}{0.60}$ | 0.13% | 1002 | $\color{red}{1006}$ | 1.00 | 52.4% |
| silence | 27.4 | $\color{green}{4.9}$ | $\color{green}{0.18}$ | 0.02% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 47.3 | $\color{green}{40.0}$ | $\color{green}{0.84}$ | 0.20% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 42.3 | $\color{green}{23.8}$ | $\color{green}{0.56}$ | 0.12% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 49.5 | $\color{green}{33.2}$ | $\color{green}{0.67}$ | 0.17% | 2321 | $\color{green}{2187}$ | $\color{green}{0.94}$ | 38.0% |
| noise | 47.0 | $\color{green}{33.6}$ | $\color{green}{0.71}$ | 0.17% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 28.2 | $\color{green}{4.9}$ | $\color{green}{0.17}$ | 0.02% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{green}{0.94}$ | $\color{red}{1.01}$ | $\color{red}{2.20}$ | $\color{green}{0.96}$ | $\color{green}{0.34}$ | $\color{red}{1.01}$ |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.71}$ | $\color{red}{1.01}$ | $\color{red}{1.68}$ | $\color{green}{0.88}$ | $\color{green}{0.28}$ | $\color{red}{1.02}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.66}$ | 1.00 | $\color{red}{1.42}$ | $\color{green}{0.78}$ | $\color{green}{0.24}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.86}$ | 1.00 | $\color{red}{1.85}$ | $\color{green}{0.97}$ | $\color{green}{0.28}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | $\color{green}{0.92}$ | 1.00 | $\color{red}{2.03}$ | $\color{green}{0.96}$ | $\color{green}{0.37}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.57}$ | 1.00 | $\color{red}{1.17}$ | $\color{green}{0.88}$ | $\color{green}{0.25}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.59}$ | $\color{red}{1.01}$ | $\color{red}{1.18}$ | $\color{green}{0.79}$ | $\color{green}{0.26}$ | $\color{red}{1.01}$ |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.40}$ | 1.00 | $\color{green}{0.69}$ | $\color{green}{0.97}$ | $\color{green}{0.30}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.59}$ | 1.00 | $\color{red}{1.08}$ | $\color{green}{0.49}$ | $\color{green}{0.23}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.62}$ | $\color{red}{1.01}$ | $\color{red}{1.31}$ | $\color{green}{0.95}$ | $\color{green}{0.28}$ | $\color{red}{1.01}$ |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.68}$ | 1.00 | $\color{red}{1.43}$ | $\color{green}{0.99}$ | $\color{green}{0.27}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.72}$ | 1.00 | $\color{red}{1.48}$ | $\color{green}{0.99}$ | $\color{green}{0.28}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 89.0 | $\color{green}{83.8}$ | $\color{green}{0.94}$ | 191 | $\color{green}{177}$ | 571 | $\color{green}{399}$ | 0.42% |
| Chopin, Fantasy Op. 49, piano | 77.2 | $\color{green}{55.1}$ | $\color{green}{0.71}$ | 147 | $\color{green}{97.1}$ | 438 | $\color{green}{253}$ | 0.28% |
| Shostakovich 5, orchestra, live | 44.7 | $\color{green}{29.4}$ | $\color{green}{0.66}$ | 78.1 | $\color{green}{57.0}$ | 245 | $\color{green}{235}$ | 0.15% |
| Indie rock, 2022, mastered | 53.1 | $\color{green}{45.8}$ | $\color{green}{0.86}$ | 118 | $\color{green}{103}$ | 293 | $\color{green}{290}$ | 0.23% |
| Limehouse Blues, 1924 78 rpm | 86.6 | $\color{green}{80.1}$ | $\color{green}{0.92}$ | 152 | $\color{green}{131}$ | 377 | $\color{green}{339}$ | 0.40% |
| Prison work song, 1947 field recording | 39.0 | $\color{green}{22.1}$ | $\color{green}{0.57}$ | 72.3 | $\color{green}{40.1}$ | 183 | $\color{green}{140}$ | 0.11% |
| The Raven, read, produced | 39.0 | $\color{green}{22.9}$ | $\color{green}{0.59}$ | 73.4 | $\color{green}{41.9}$ | 224 | $\color{green}{198}$ | 0.11% |
| Leviticus 13, read | 25.1 | $\color{green}{10.2}$ | $\color{green}{0.40}$ | 63.3 | $\color{green}{30.3}$ | 302 | $\color{green}{175}$ | 0.05% |
| JFK at Rice, 1962 | 44.0 | $\color{green}{26.2}$ | $\color{green}{0.59}$ | 101 | $\color{green}{64.5}$ | 506 | $\color{green}{314}$ | 0.13% |
| Train station, field recording | 38.9 | $\color{green}{24.1}$ | $\color{green}{0.62}$ | 72.8 | $\color{green}{59.0}$ | 209 | $\color{green}{155}$ | 0.12% |
| Pink noise | 52.0 | $\color{green}{35.6}$ | $\color{green}{0.68}$ | 117 | $\color{green}{62.2}$ | 472 | $\color{green}{186}$ | 0.18% |
| White noise | 47.5 | $\color{green}{34.2}$ | $\color{green}{0.72}$ | 86.6 | $\color{green}{61.9}$ | 308 | $\color{red}{328}$ | 0.17% |

### The bytes, libFLAC at level 5

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5369}$ | $\color{red}{1.011}$ | 46.6% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2502}$ | $\color{red}{1.010}$ | 32.6% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1499}$ | 1.000 | 39.0% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{red}{4566}$ | $\color{red}{1.002}$ | 79.3% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | $\color{green}{8657}$ | 1.000 | 75.1% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | $\color{red}{1421}$ | $\color{red}{1.001}$ | 40.3% |
| The Raven, read, produced | 559 | 3528 | 1362 | $\color{red}{1370}$ | $\color{red}{1.006}$ | 38.8% |
| Leviticus 13, read | 725 | 1764 | 986 | 986 | 1.000 | 55.9% |
| JFK at Rice, 1962 | 836 | 5760 | 1798 | $\color{green}{1797}$ | 1.000 | 31.2% |
| Train station, field recording | 141 | 3528 | 1272 | $\color{red}{1281}$ | $\color{red}{1.007}$ | 36.3% |
| Pink noise | 600 | 5760 | 4901 | $\color{green}{4900}$ | 1.000 | 85.1% |
| White noise | 600 | 5760 | 5387 | 5387 | 1.000 | 93.5% |

