# What the encoder was measured at

The numbers the encoder's decisions were taken on, against libFLAC, which the
crate linked until v0.0.4. How a block becomes a frame is in the module docs
of `src/lib.rs`, `src/subframe.rs` and `src/lpc.rs`, each carrying the
numbers of its own part.

The tables are made by `tools/bench`, a cargo project beside the crate that
is not a member of it, so the crate itself keeps to `thiserror`: it builds
v0.0.4 of this repository from its tag for libFLAC at its default level 5 as
that release ran it, a stream of its own for every frame, and libFLAC's sys
crate for its levels 0 and 8, and codes fifty 20 ms blocks of five sounds
with each and with this crate. A cell's time is one block's, the least of
five passes over the fifty, each pass timed as a whole after one to warm up;
its bytes are the frames' over the fifty, a block's share. `tools/tables.py`
makes the tables below from the log:

```sh
cargo run --release --manifest-path tools/bench/Cargo.toml > bench.log
python3 -I tools/tables.py bench.log
```

They were run on 2026-10-22 on a six-core Intel i5-8500T under WSL2, one
thread, the machine idle, with rustc 1.98.1 at the x86-64 baseline (SSE2, no
`target-cpu`), on commit e3173f7 of branch `own-encoder`, the encoder with
its linear predictor of up to order 8. A time on this machine moves by up to a
tenth from one run to the next, so a time ratio within a tenth of 1.00 is on
the line; a bytes ratio is exact.

Throughout, a *ratio* is this encoder over libFLAC, so under 1.00 is less
time or fewer bytes. A block is 20 ms of sound, so *of one core* is its time
over 20 ms, the share of a core the encoder takes coding the stream as it
comes; *of the PCM* is the frame's bytes over the block's. Green is this
encoder's side of the line and red the other; a number on the line is left
plain.

## The sounds

Made, not recorded, so the run is the same on any machine and the sounds
are each one thing: fifty blocks, a second of each.

- `music`: a three-note chord with five harmonics a note and a slow vibrato,
  a bass note under it, a little room noise, and a right channel that is the
  left mixed with another note. What a desktop plays.
- `speech`: one voice-like tone of twelve harmonics, its pitch and loudness
  moving, with breath noise, the same on both channels: a call.
- `tone`: a pure 1 kHz sine at −6 dBFS, a notification's beep, at 0.7 of it
  on the right. A resonance is the ill-conditioned fit, where a predictor's
  coefficients are large and its rounding tells.
- `noise`: white noise at −20 dBFS on each channel, nothing to predict: the
  most a desktop sends.
- `silence`: zeros.

## What the tables say

- **Every shape and sound takes less of a core than libFLAC at its default
  level**, 0.40 to 0.84 of its time: 16-bit stereo 0.40 to 0.74, mono 0.44
  to 0.46, 8-bit 0.47 to 0.66, 24-bit 0.56 to 0.84; silence 0.09 to 0.21.
  A block of 16-bit stereo at 48 kHz is 0.09% to 0.15% of one core, mono
  0.05% to 0.06%, 96 kHz 0.18% to 0.29%; silence 0.01% to 0.05%.
- **The bytes are libFLAC's at its default level.** Music, speech, noise
  and silence come to the same bytes at every shape within a byte a block,
  but music at 96 kHz, six bytes over of 3454; the tone 0.94 to 1.00 of
  them but at 96 kHz. Against libFLAC's smallest level, 8, which tries
  every order to 12 and every stereo pairing in 1.6 to 3.6 times the
  default's time, music is 1.00 to 1.01, speech, noise and silence 1.00,
  and the tone 0.95 to 1.00 but at 96 kHz.
- **What is over the line: the tone at 96 kHz, 1.07 of libFLAC's bytes at
  level 5 and 1.19 at level 8.** The fit's autocorrelation is summed in
  single precision, and on a pure tone over 1920 samples its rounding
  misfits the predictor. Summed in double the same block is 1576 bytes,
  0.95 of libFLAC's, for 20 µs more a block at 96 kHz and 8 more at 48 kHz,
  where single precision already fits the tone better than libFLAC does. A
  desktop's beep carries noise and an envelope a pure sine has not, so it is
  left as it is and noted here.
- **Against libFLAC's fastest level, 0, which fits no predictor and codes
  stereo as it comes**, this encoder takes 1.07 to 1.88 of its time on
  stereo music, tone and noise, and 0.78 to 1.13 on speech, for fewer
  bytes: level 0 is 1.10 of the default's bytes on music and 2.0 on speech.
  Mono is 0.86 to 0.90 of it at the same bytes or fewer, silence 0.16 to
  0.46.
- **Where the time goes at 16-bit stereo**, measured when the fit was
  written: without it a block of music took 24 µs, speech 18, noise 22; the
  fit is 1.5 µs a channel for the two channels written, 1.2 of it the
  autocorrelation, its residual 1.1 µs a channel through 16-bit lanes and
  its partitions about 1. Noise pays the autocorrelation before the fit is
  declined, so noise costs more than speech, whose fit is taken. At 24 bits
  the residual runs in 32-bit lanes, 3 µs a channel, which is the 0.84 on
  music there.

## Against libFLAC at level 5, as v0.0.4 ran it

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

## Against libFLAC at its two ends

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

## Each shape apart

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

