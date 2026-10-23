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
`target-cpu`), on branch `own-encoder` after commit 7145f72, the encoder
with its linear predictor of up to order 8 fitted in double precision, the
first item of [plan.md](plan.md) taken. A time on this machine moves by up to a
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

- **Every recording and every made sound takes no more of a core than
  libFLAC at its default level, and all but two take less.** The recordings
  are 0.46 to 1.01 of its time a block: 16-bit stereo 0.65 to 0.82, mono
  0.46, 24-bit 0.63 to 1.01, the two at 96 kHz and 24 bits on the line at
  1.00 and 1.01. A block of a recording is 0.05% to 0.43% of one core. The
  made sounds are 0.42 to 0.94: 16-bit stereo 0.47 to 0.82, mono 0.42 to
  0.51, 8-bit 0.51 to 0.69, 24-bit 0.64 to 0.94; silence 0.10 to 0.25.
- **The late blocks are earlier.** On the recordings this encoder's
  99th-percentile block is 0.54 to 0.90 of libFLAC's, 24 to 130 µs against
  44 to 149, and its worst block 104 to 298 µs, at most 1.5% of the 20 ms
  the block is, where libFLAC's worst is 160 to 1616. A 99th-percentile
  block is 1.5 to 2.3 times the mean for both encoders: the sound's, not the
  encoder's, the loud and the busy blocks costing more Rice bits to write.
- **The bytes are libFLAC's at its default level, within 0.6%.** Every
  made sound comes to the same bytes within a byte a block but music at
  96 kHz, five bytes over of 3454, the tone at 96 kHz, six over of 1651,
  and the tone at 24 bits, 1.05. Ten recordings are within 0.2%; the
  Fantasy is 1.004 and the train 1.006. Against libFLAC's smallest level,
  8, which tries every order to 12 and every stereo pairing in 1.5 to 3.2
  times the default's time on sound (1.4 to 2.9 on the recordings, under
  1 on silence), the made sounds
  are 1.00 to 1.02 but the tone at 96 kHz, 1.12, and at 24 bits, 1.06, and
  the recordings 1.00 to 1.01.
- **The fit is in double precision, which bought the pianos and the voice
  their bytes back.** Summed in single precision, as the first table of
  this document was run, the fit's autocorrelation misfit the predictor of
  a quiet, resonant block: the Goldberg Aria was 1.011 of libFLAC's bytes,
  the Fantasy 1.010, the Raven 1.006 and the made tone at 96 kHz 1.07.
  In double, in two passes of two lanes that stay in registers, they are
  1.002, 1.004, 1.001 and 1.00 for 1.7 µs more a channel at 48 kHz and 3 at
  96 kHz; the train stays at 1.006, which is the pairing chosen before the
  fit, the first item of [plan.md](plan.md) now. The pure tone's bytes are
  another matter: a tone is predicted away by order 2, every higher order
  ties on the estimate, and what the frame costs is the quantized
  coefficients' rounding, which the estimate does not see; any change to the
  fit's arithmetic moved the tone's bytes by up to a tenth either way, and
  the 1.05 at 24 bits is one such roll, libFLAC's 1008 and 920 at 48 and
  44.1 kHz being this encoder's too.
- **Against libFLAC's fastest level, 0, which fits no predictor and codes
  stereo as it comes**, this encoder takes 1.16 to 2.32 of its time on the
  stereo recordings and 0.82 on the mono one, for 0.49 to 0.99 of its bytes:
  the two stereo speeches 0.49 and 0.78, the music 0.78 to 0.96, noise
  0.99. On the made sounds it is 1.35 to 2.17 on stereo music, tone and
  noise, 0.84 to 1.17 on speech, 0.79 to 1.02 on mono and 0.18 to 0.56 on
  silence.
- **A loud modern master is most of its PCM at 24 bits.** The indie rock
  track, mixed and mastered loud in 2022 and the one popular record in the
  set, is 79% of its PCM at 48 kHz 24-bit for both encoders, where the 1962
  speech at the same shape is 31% and the orchestra at 16 bits 39%: under a
  dense mix the low bits of a 24-bit sample are the mix's own noise floor,
  which no predictor takes out and Rice writes as it is. The same track
  dithered to 16 bits, measured aside and not in the tables, is 69% of its
  PCM, 2645 bytes a block against libFLAC's 2638 (1.003), in 0.64 of its
  time where at 24 bits it is 0.91: what a 24-bit capture of such a record
  pays over a 16-bit one, a third more bytes and the 32-bit residual lanes,
  buys the noise floor.
- **Where the time goes at 16-bit stereo**, measured when the fit was
  written and when it went to double: without the fit a block of music took
  24 µs, speech 18, noise 22; the fit is 3.2 µs a channel for the two
  channels written, 2.9 of it the autocorrelation (1.2 in single
  precision), its residual 1.1 µs a channel through 16-bit lanes and its
  partitions about 1. Noise pays the autocorrelation before the fit is
  declined, so noise costs more than speech, whose fit is taken. At 24 bits
  the residual runs in 32-bit lanes, 3 µs a channel, which is the 0.91 on
  made music there, the 1.00 on the 78 rpm transfer and the 0.91 on the
  indie rock master.

## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.74}$ | $\color{green}{0.69}$ | $\color{green}{0.82}$ | $\color{green}{0.50}$ | $\color{green}{0.65}$ | $\color{green}{0.91}$ |
| speech | $\color{green}{0.52}$ | $\color{green}{0.47}$ | $\color{green}{0.63}$ | $\color{green}{0.42}$ | $\color{green}{0.51}$ | $\color{green}{0.64}$ |
| tone | $\color{green}{0.73}$ | $\color{green}{0.73}$ | $\color{green}{0.80}$ | $\color{green}{0.51}$ | $\color{green}{0.69}$ | $\color{green}{0.94}$ |
| noise | $\color{green}{0.68}$ | $\color{green}{0.67}$ | $\color{green}{0.74}$ | $\color{green}{0.51}$ | $\color{green}{0.69}$ | $\color{green}{0.74}$ |
| silence | $\color{green}{0.22}$ | $\color{green}{0.19}$ | $\color{green}{0.25}$ | $\color{green}{0.10}$ | $\color{green}{0.21}$ | $\color{green}{0.20}$ |

### Bytes a block, this encoder over libFLAC

Under 1.00 is a smaller frame than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | 1.00 | 1.00 | 1.00 | 1.00 | $\color{green}{0.99}$ | $\color{red}{1.05}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, against libFLAC at its two ends

### Time a block, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, which fits no linear predictor and codes stereo as it comes.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.62}$ | $\color{red}{1.35}$ | $\color{red}{1.98}$ | $\color{green}{0.88}$ | $\color{red}{1.47}$ | $\color{red}{1.96}$ |
| speech | $\color{red}{1.02}$ | $\color{green}{0.84}$ | $\color{red}{1.17}$ | $\color{green}{0.79}$ | 1.00 | $\color{red}{1.15}$ |
| tone | $\color{red}{1.61}$ | $\color{red}{1.53}$ | $\color{red}{1.93}$ | $\color{red}{1.02}$ | $\color{red}{1.58}$ | $\color{red}{2.17}$ |
| noise | $\color{red}{1.40}$ | $\color{red}{1.39}$ | $\color{red}{1.81}$ | $\color{green}{0.96}$ | $\color{red}{1.52}$ | $\color{red}{1.62}$ |
| silence | $\color{green}{0.42}$ | $\color{green}{0.41}$ | $\color{green}{0.56}$ | $\color{green}{0.18}$ | $\color{green}{0.42}$ | $\color{green}{0.39}$ |

### Bytes a block, this encoder over libFLAC at level 8

Under 1.00 is a smaller frame than libFLAC's smallest level, which tries every order to 12 and every stereo pairing.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.01}$ | $\color{red}{1.01}$ | 1.00 | $\color{red}{1.01}$ | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | $\color{red}{1.02}$ | 1.00 | $\color{red}{1.12}$ | 1.00 | $\color{green}{0.99}$ | $\color{red}{1.06}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, each shape apart

libFLAC is at level 5. The time is one block's, the least of five passes;
a block is 20 ms of sound, so a share of one core is that time over 20 ms.

### 48 kHz stereo 16-bit, 960 frames a block, 3840 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 47.2 | $\color{green}{34.8}$ | $\color{green}{0.74}$ | 0.17% | 1838 | 1838 | 1.00 | 47.9% |
| speech | 41.4 | $\color{green}{21.7}$ | $\color{green}{0.52}$ | 0.11% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 45.1 | $\color{green}{32.9}$ | $\color{green}{0.73}$ | 0.16% | 1008 | 1008 | 1.00 | 26.2% |
| noise | 46.1 | $\color{green}{31.4}$ | $\color{green}{0.68}$ | 0.16% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 25.7 | $\color{green}{5.6}$ | $\color{green}{0.22}$ | 0.03% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 40.4 | $\color{green}{28.0}$ | $\color{green}{0.69}$ | 0.14% | 1702 | 1702 | 1.00 | 48.2% |
| speech | 36.9 | $\color{green}{17.2}$ | $\color{green}{0.47}$ | 0.09% | 1063 | 1063 | 1.00 | 30.1% |
| tone | 40.5 | $\color{green}{29.6}$ | $\color{green}{0.73}$ | 0.15% | 920 | 920 | 1.00 | 26.1% |
| noise | 43.1 | $\color{green}{28.9}$ | $\color{green}{0.67}$ | 0.14% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 26.8 | $\color{green}{5.2}$ | $\color{green}{0.19}$ | 0.03% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 78.2 | $\color{green}{64.3}$ | $\color{green}{0.82}$ | 0.32% | 3454 | $\color{red}{3459}$ | 1.00 | 45.0% |
| speech | 64.8 | $\color{green}{40.8}$ | $\color{green}{0.63}$ | 0.20% | 2226 | 2226 | 1.00 | 29.0% |
| tone | 76.7 | $\color{green}{61.4}$ | $\color{green}{0.80}$ | 0.31% | 1651 | $\color{red}{1657}$ | 1.00 | 21.6% |
| noise | 82.4 | $\color{green}{61.4}$ | $\color{green}{0.74}$ | 0.31% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 44.5 | $\color{green}{11.1}$ | $\color{green}{0.25}$ | 0.06% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 25.8 | $\color{green}{12.9}$ | $\color{green}{0.50}$ | 0.06% | 950 | 950 | 1.00 | 49.5% |
| speech | 30.0 | $\color{green}{12.8}$ | $\color{green}{0.42}$ | 0.06% | 1148 | 1148 | 1.00 | 59.8% |
| tone | 26.1 | $\color{green}{13.4}$ | $\color{green}{0.51}$ | 0.07% | 512 | 512 | 1.00 | 26.7% |
| noise | 26.4 | $\color{green}{13.3}$ | $\color{green}{0.51}$ | 0.07% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 18.7 | $\color{green}{1.8}$ | $\color{green}{0.10}$ | 0.01% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 45.1 | $\color{green}{29.1}$ | $\color{green}{0.65}$ | 0.15% | 456 | 456 | 1.00 | 23.7% |
| speech | 36.8 | $\color{green}{18.8}$ | $\color{green}{0.51}$ | 0.09% | 250 | $\color{red}{251}$ | 1.00 | 13.1% |
| tone | 40.8 | $\color{green}{28.3}$ | $\color{green}{0.69}$ | 0.14% | 476 | $\color{green}{471}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 41.6 | $\color{green}{28.7}$ | $\color{green}{0.69}$ | 0.14% | 1002 | $\color{red}{1006}$ | 1.00 | 52.4% |
| silence | 24.8 | $\color{green}{5.1}$ | $\color{green}{0.21}$ | 0.03% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 47.4 | $\color{green}{43.0}$ | $\color{green}{0.91}$ | 0.21% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 39.8 | $\color{green}{25.6}$ | $\color{green}{0.64}$ | 0.13% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 46.3 | $\color{green}{43.4}$ | $\color{green}{0.94}$ | 0.22% | 2321 | $\color{red}{2440}$ | $\color{red}{1.05}$ | 42.4% |
| noise | 48.4 | $\color{green}{35.8}$ | $\color{green}{0.74}$ | 0.18% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 25.6 | $\color{green}{5.1}$ | $\color{green}{0.20}$ | 0.03% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{red}{1.01}$ | 1.00 | $\color{red}{2.32}$ | $\color{green}{0.96}$ | $\color{green}{0.36}$ | 1.00 |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.82}$ | 1.00 | $\color{red}{1.89}$ | $\color{green}{0.87}$ | $\color{green}{0.32}$ | $\color{red}{1.01}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.72}$ | 1.00 | $\color{red}{1.54}$ | $\color{green}{0.78}$ | $\color{green}{0.26}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.91}$ | 1.00 | $\color{red}{1.96}$ | $\color{green}{0.96}$ | $\color{green}{0.31}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | 1.00 | 1.00 | $\color{red}{2.18}$ | $\color{green}{0.96}$ | $\color{green}{0.40}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.65}$ | 1.00 | $\color{red}{1.32}$ | $\color{green}{0.88}$ | $\color{green}{0.29}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.65}$ | 1.00 | $\color{red}{1.33}$ | $\color{green}{0.78}$ | $\color{green}{0.29}$ | $\color{red}{1.01}$ |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.46}$ | 1.00 | $\color{green}{0.82}$ | $\color{green}{0.97}$ | $\color{green}{0.34}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.63}$ | 1.00 | $\color{red}{1.16}$ | $\color{green}{0.49}$ | $\color{green}{0.24}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.65}$ | $\color{red}{1.01}$ | $\color{red}{1.35}$ | $\color{green}{0.95}$ | $\color{green}{0.30}$ | $\color{red}{1.01}$ |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.82}$ | 1.00 | $\color{red}{1.71}$ | $\color{green}{0.99}$ | $\color{green}{0.30}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.79}$ | 1.00 | $\color{red}{1.60}$ | $\color{green}{0.99}$ | $\color{green}{0.30}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 83.2 | $\color{red}{83.7}$ | $\color{red}{1.01}$ | 144 | $\color{green}{130}$ | 306 | $\color{green}{288}$ | 0.42% |
| Chopin, Fantasy Op. 49, piano | 75.6 | $\color{green}{61.8}$ | $\color{green}{0.82}$ | 132 | $\color{green}{103}$ | 690 | $\color{green}{269}$ | 0.31% |
| Shostakovich 5, orchestra, live | 44.6 | $\color{green}{32.0}$ | $\color{green}{0.72}$ | 78.4 | $\color{green}{52.3}$ | 308 | $\color{green}{265}$ | 0.16% |
| Indie rock, 2022, mastered | 49.9 | $\color{green}{45.2}$ | $\color{green}{0.91}$ | 87.3 | $\color{green}{77.8}$ | 312 | $\color{green}{199}$ | 0.23% |
| Limehouse Blues, 1924 78 rpm | 87.1 | $\color{green}{86.8}$ | 1.00 | 149 | $\color{green}{127}$ | 326 | $\color{green}{298}$ | 0.43% |
| Prison work song, 1947 field recording | 38.8 | $\color{green}{25.4}$ | $\color{green}{0.65}$ | 71.7 | $\color{green}{45.7}$ | 196 | $\color{red}{252}$ | 0.13% |
| The Raven, read, produced | 39.3 | $\color{green}{25.8}$ | $\color{green}{0.65}$ | 73.3 | $\color{green}{43.3}$ | 248 | $\color{green}{218}$ | 0.13% |
| Leviticus 13, read | 23.2 | $\color{green}{10.8}$ | $\color{green}{0.46}$ | 44.5 | $\color{green}{24.1}$ | 275 | $\color{green}{129}$ | 0.05% |
| JFK at Rice, 1962 | 40.8 | $\color{green}{25.6}$ | $\color{green}{0.63}$ | 76.8 | $\color{green}{44.4}$ | 281 | $\color{green}{216}$ | 0.13% |
| Train station, field recording | 39.2 | $\color{green}{25.5}$ | $\color{green}{0.65}$ | 73.7 | $\color{green}{42.2}$ | 160 | $\color{green}{104}$ | 0.13% |
| Pink noise | 47.8 | $\color{green}{39.0}$ | $\color{green}{0.82}$ | 85.1 | $\color{green}{68.6}$ | 1616 | $\color{green}{292}$ | 0.20% |
| White noise | 47.3 | $\color{green}{37.2}$ | $\color{green}{0.79}$ | 81.1 | $\color{green}{61.9}$ | 1000 | $\color{green}{284}$ | 0.19% |

### The bytes, libFLAC at level 5

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5325}$ | $\color{red}{1.002}$ | 46.2% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2485}$ | $\color{red}{1.004}$ | 32.4% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1499}$ | 1.000 | 39.0% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{red}{4556}$ | 1.000 | 79.1% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | $\color{green}{8657}$ | 1.000 | 75.1% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | $\color{red}{1421}$ | $\color{red}{1.001}$ | 40.3% |
| The Raven, read, produced | 559 | 3528 | 1362 | $\color{red}{1364}$ | $\color{red}{1.001}$ | 38.7% |
| Leviticus 13, read | 725 | 1764 | 986 | 986 | 1.000 | 55.9% |
| JFK at Rice, 1962 | 836 | 5760 | 1798 | $\color{green}{1797}$ | 1.000 | 31.2% |
| Train station, field recording | 141 | 3528 | 1272 | $\color{red}{1280}$ | $\color{red}{1.006}$ | 36.3% |
| Pink noise | 600 | 5760 | 4901 | $\color{green}{4900}$ | 1.000 | 85.1% |
| White noise | 600 | 5760 | 5387 | 5387 | 1.000 | 93.5% |

