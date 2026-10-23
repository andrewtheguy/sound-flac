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
two of the user's own, whole, in blocks of 20 ms. A made sound's time is one block's, the least of five passes' mean over
the fifty, each block timed on its own, after one pass to warm up. A
recording's is the least of three passes' mean block, each block timed on
its own as well, and with it
the least of the passes' 99th-percentile and worst blocks, the latency a
stream sees; its bytes are the frames' over the recording, a block's share.
`tools/tables.py` makes the tables below from the log, and the logs the
tables are of are kept in `docs/bench/`, so that the next run's recordings
tables can carry this run beside them as *the run before*:

```sh
tools/samples.sh copy      # the recordings into tmp/samples, from the store
cargo run --release --manifest-path tools/bench/Cargo.toml > docs/bench/x86.log
python3 -I tools/tables.py docs/bench/x86.log
python3 -I tools/tables.py docs/bench/x86.log arm64=docs/bench/arm64.log   # with an arm64 run's log, its section too
python3 -I tools/tables.py docs/bench/x86.log arm64=docs/bench/arm64.log windows=docs/bench/windows.log   # and the Windows run's, made on demand
git show HEAD:docs/bench/x86.log > before.log           # the run before, from the commit before
python3 -I tools/tables.py docs/bench/x86.log arm64=docs/bench/arm64.log windows=docs/bench/windows.log --before before.log
```

In the recordings tables below, *the run before* is the evening's run of
the same day, the tables the commit before this one carried, of the same
encoder: this run differs from it only in the bench, which now codes every
block into an empty buffer in a timed pass, as its warm-up pass always did,
where before a pass's frames piled up in one buffer.

They were run on 2026-10-23 on a six-core Intel i5-8500T under WSL2, one
thread, the machine idle but for the terminal session the run was started
from, with rustc 1.98.1 at the x86-64 baseline (SSE2, no
`target-cpu`), on branch `own-encoder` at the commit that carries them: the
encoder with its stereo pair chosen on the fit as well as the fixed
predictors every fourth block and carried between, the fit skipped on noise,
and, from the afternoon's work of [plan.md](plan.md), a carried block's two
channels coded the fit first with the fixed path left out where the fit
clearly wins, the autocorrelation in one pass at the floor of the baseline's
floating-point ports, the frame's CRC in two halves, and, from the end of
that day, the fourth block's four channels weighed on a one-partition count
of their fixed predictor in place of four fixed codings, each channel of
the pair having both codings made once a cycle on the blocks after it. A
time on this
machine moves by up to a tenth from one run to the next, so a time ratio
within a tenth of 1.00 is on the line; a bytes ratio is exact. libFLAC's
own times are the control: in this run they are 1% over to 13% under the
run before's at level 5 and up to 17% under at level 0, and this encoder's
moved with them, 1% to 10% under on all but a few sounds within 3% over,
so its ratio to level 5 stands within a few hundredths while its ratio to
level 0 rose by up to a tenth on x86 with no change to the encoder, which
is the commit before's. What differs is the bench, every block now coded
into an empty buffer, and the machine; on arm64, where a time repeats to
1%, both coders came to the run before's times within 3%, so the bench's
change is not what moved x86. A worst block
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

The Windows run of the section after it was made the night of the same day
on this same i5-8500T under Windows 11 Pro, natively, with rustc 1.93 MSVC
at the x86-64 baseline and libFLAC's levels 0 and 8 from its sys crate's
Windows archive, from a clone of `main` at v0.0.5, whose encoder is the one
of these tables to the byte: the recordings copied from the store to the
clone for the run and deleted after it, the machine idle but for the WSL
session the run was started from. Unlike the x86 and arm64 runs, it is made
on demand, not with every change to the tuning, so its section can be of an
earlier encoder than the tables above; this paragraph says which. Levels 0
and 5 and this encoder were then run again under both systems the same
night, Windows first and WSL after it, to read that run's level 0 against
the tables' WSL run; those logs are not kept, and what they showed is the
last point of the next section.

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
- `noise`: uniform white noise peaking at −26 dBFS on each channel, nothing
  to predict.
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

- **Every recording and every made sound takes about half of what libFLAC
  at its default level takes of a core or less, most of them a little
  over a third.** The recordings are 0.34 to 0.58 of its time a block:
  16-bit stereo 0.41 to 0.46, mono 0.34, 24-bit 0.36 to 0.58, the two at
  96 kHz and 24 bits 0.51 and 0.58. A block of a recording is 0.04% to
  0.24% of one core. The made sounds are 0.27 to 0.55: music 0.33 to 0.47,
  speech 0.30 to 0.36, the tone 0.42 to 0.55, noise 0.27 to 0.36, silence
  0.07 to 0.10. The evening's run before this one, the same encoder, had
  them at 0.33 to 0.55 and 0.27 to 0.53, the afternoon's 0.31 to 0.59 and
  0.25 to 0.49, the morning's 0.44 to 0.77 and 0.40 to 0.74, the
  night before 0.43 to 0.87 and 0.41 to 0.91, for the same bytes within
  half a per cent.
- **The bytes are libFLAC's at its default level, within 0.2%.** Every
  recording is 1.000 but the two pianos at 1.001 and 1.002, the prison song,
  the Raven and the two private mixes at 1.001, a byte to six a block, which
  is what fitting only the carried pair between full choices and coding it
  the fit first cost them (the measurements are in [plan.md](plan.md));
  before the pair was chosen on the fit the train station was 1.006 and the
  Fantasy 1.004. Against the run before, the same encoder, every
  recording and every made sound comes to the same bytes to a tenth of a
  byte. Every made sound comes to the same bytes within two
  bytes a block but the pure tone, 1.02 at 44.1 kHz, 0.95 at 96 kHz and
  1.05 at 24 bits: a tone is predicted away by order 2 and what its frame
  then costs is the rounding of the coefficients, which goes with every
  change to the fit's arithmetic and which no estimate tried could foresee.
  The made speech, whose two channels are one signal, is 0.05% over the
  afternoon's run and 0.16% at 8 bits: its side channel is a constant, and a block
  with a constant channel is now coded the fit first like any other where
  it had every channel coded both ways. Against libFLAC's smallest level,
  8, which tries every order to 12 and every stereo pairing in four to
  seven times this encoder's time, the made sounds are 1.00 to 1.03 but the
  tone at 96 kHz, 1.07, and at 24 bits, 1.06, and the recordings 1.00 to
  1.01.
- **The late block is still the fourth block, where the pair is chosen
  afresh, by a quarter to a third of a carried block rather than half as
  much again.** Each block's least time over the passes, each pass from an
  encoder that has seen no block, read by the block's place in the cycle of
  four: the fourth block is 22.4 µs against 17.4 for a carried block on the
  orchestra, where the afternoon's run had 27.1 against 18.1; 44.1 against
  34.5 on the Aria, from 57.9 against 35.0; 18.2 against 13.9 on the Raven,
  from 22.2 against 14.5; 16.7 against 13.8 on the wide mix, from 21.2
  against 14.2, each within 1 µs of the evening's run before this one. The two blocks after the choice, where each channel of the pair has
  both codings made once to bring its correction up to date, are about 1
  µs over a plain carried block at 16 bits and 4 on the Aria. This
  encoder's own 99th-percentile block is now set by the loud blocks as
  much as by the fourth, 1.4 to 1.5 times its mean, and its worst about
  twice. A single pass's 99th percentile is higher still for both encoders,
  which is scheduling and cache, not the sound. On the recordings this
  encoder's 99th-percentile block is 0.32 to 0.56 of libFLAC's, 13 to 77 µs
  against 40 to 142, and its worst block 78 to 224 µs, at most 1.1% of the
  20 ms the block is, where libFLAC's worst is 166 to 3550. Where two
  channels are one signal, the 1962 speech and the made speech, the fourth
  block codes the three equivalent channels in turn, each coming out over
  its estimate: 19 µs against 12 on that recording, whose mean fell to 0.86
  of the afternoon's run all the same, the blocks between coding one channel
  and a constant.
- **The pair is chosen on the fit as well as the fixed predictors every
  fourth block, and carried between.** On the fourth block every channel
  is weighed from its autocorrelation, the mid's derived from left's,
  right's and side's so three passes serve four channels: the fixed
  predictor of the order the autocorrelation chooses has its residual
  taken and counted in one Rice partition, a true count of the channel's
  bits within what the partitions chosen gain, and the bits its fit is
  estimated to save come off that. The pair is chosen on those, the two
  chosen are coded the fit first, and the pair is chosen again with their
  real bits until it stands. The estimate alone is good at differences and
  poor at levels: chosen on estimates alone the 8-bit speech was 1.63 and
  the tones 1.05 to 1.10, and the count is what stands in for the fixed
  coding's real bits, which the run before had for all four channels at
  1.9 µs each. On the three blocks between, the pair is coded again, each
  channel the fit first: where the fit is estimated to beat the best fixed
  predictor by an eighth of a bit a sample after a correction the channel
  learned the last time both were coded, the fit alone is coded and the
  order search and the fixed partitions are not run, and otherwise both are
  and compared; each channel of the pair has both coded once a cycle
  whatever the estimate, on the two blocks after the choice, which keeps
  the correction of this second: left to the blocks where the fit was not
  clear, it went stale and cost the orchestra 0.4%. The correction is what
  the estimates miss, the window and the coefficients' rounding: on the
  orchestra at 16 bits the fit is coded alone on a third of the carried
  channels, on the Raven on three in five, on the mixes nine in ten and on
  the Aria at 24 bits nearly all. The two other channels are not looked at
  past a constant check and a bound from one pass over their differences,
  and the pair moves where the bound says another is certainly cheaper,
  160 times in 12 000 blocks of the wide mix and once in the Raven's. A
  channel on its own, mono or more than two, is coded the fit first on
  every block. Where the fit is estimated not to win on a channel of fixed
  order 0, which is noise and a quiet desktop, it is skipped for seven
  blocks.
- **Where the time goes at 16-bit stereo**, on the orchestra, by the parts
  measured apart: a carried block, 17.6 µs, is the two autocorrelations 2.8,
  the fits from them 0.6, the pair's two residuals 2.2 and their partitions
  1.6, the two fixed codings 3.8 where the fit is not clear, which on the
  orchestra is two carried channels in three, the folds and the two bounds
  1.2, the range check and the stereo split 1.6, the bits out 3.4 and the
  CRC 0.5; the fourth block, 22.5 µs, has three autocorrelations 4.2, four
  fits 1.2 and four one-partition counts, 0.2 to 0.5 each by the order,
  in place of the two autocorrelations, two fits and two bounds, where the
  run before had four fixed codings, 7.6, and came to 27. The night before,
  30 µs a block all alike, the three autocorrelations were 8.3, in two
  passes of two lanes that checked every pair of samples; the one pass
  without a check is at the floor the baseline's two floating-point ports
  set, 1.4 µs a channel. At 24 bits the residual of a fit runs in two
  16-bit passes, and a 17-bit side channel whose samples fit 16 bits takes
  the 16-bit pass.
- **Against libFLAC's fastest level, 0, which fits no predictor and codes
  stereo as it comes**, this encoder takes 0.85 to 1.11 of its time on the
  16-bit stereo recordings, the Fantasy at 96 kHz and the orchestra the two
  over 1.00, 0.70 on the 1962 speech and 0.79 on white noise, 0.61 on the
  mono reading, and 1.16 to 1.33 on the four at 24 bits with sound in
  them, for 0.49 to 0.99 of its bytes: the two stereo speeches 0.49 and
  0.78, the music 0.78 to 0.96, noise 0.99. On the made sounds it is 0.85
  to 1.17 on stereo music and 1.10 to 1.40 on the tone, 0.55 to 0.70 on
  speech, 0.64 to 0.83 on noise, 0.52 to 0.83 on mono and 0.14 to 0.21 on
  silence. The evening's run before this one, the same encoder, had the
  16-bit stereo recordings at 0.80 to 1.01, the 1962 speech at 0.65, the
  mono reading at 0.58 and the 24-bit four at 1.07 to 1.19: this column
  moved by libFLAC's level 0 alone, 5% to 17% faster on x86 in this run
  where its level 5 was 1% to 13% faster, and on arm64, where neither
  moved past 3%, the column is the run before's. The afternoon's run had
  the 16-bit stereo recordings at 0.81 to 1.04, the mono reading at 0.53
  and the 24-bit four at 1.11 to 1.28, the morning's 1.21 to 1.79 and 1.25
  to 1.89 on stereo music and tone, and the night before 1.06 to 2.09 and
  1.35 to 2.37. The two are not doing the
  same work: level 0's block of silence alone costs 12.6 µs where this
  encoder's costs 2, and what this encoder does that level 0 does not, the
  fit and the stereo choice, is most of its carried block. What was
  measured to bring the column down, and what is left, is in
  [plan.md](plan.md).
- **A loud modern master is most of its PCM at 24 bits.** The indie rock
  track, mixed and mastered loud in 2022 and the one popular record in the
  set, is 79% of its PCM at 48 kHz 24-bit for both encoders, where the 1962
  speech at the same shape is 31% and the orchestra at 16 bits 39%: under a
  dense mix the low bits of a 24-bit sample are the mix's own noise floor,
  which no predictor takes out and Rice writes as it is. The same track
  dithered to 16 bits, measured aside on 2026-10-22 and not in the tables,
  was 69% of its PCM, 2645 bytes a block against libFLAC's 2638 (1.003).
- **On arm64 the encoder stands where it does on x86, a little further on
  its side of libFLAC's default level, and against level 0 a little behind
  x86.** The same run on the two-core Ampere Altra, in the last section:
  the recordings are 0.31 to 0.48 of libFLAC's time a block where x86 is
  0.34 to 0.58, the made sounds 0.26 to 0.48 against 0.27 to 0.55 and
  silence 0.07 to 0.10 on both. This encoder's mean block is within 0.1 µs
  of the i5's or shorter on every recording, by up to 2 µs, while
  libFLAC's is longer on every stereo recording and 8% shorter on the mono
  reading; this encoder's 99th-percentile block is lower there on every
  recording, 8 to 61 µs against 13 to 77. Against level 0 it is 0.98 to
  1.23 of its time on the stereo recordings but white noise and the 1962
  speech, 0.79 and 0.71, where x86 is 0.70 to 1.33, and 0.75 on the mono
  reading where x86 is 0.61: level 0's fixed path is cheap on the Altra.
  Against the run before on that machine, the same encoder, both coders
  came to the same times within 3% on every made sound and recording, this
  encoder's recordings within 1% but the 1924 record and white noise at 3%
  and 2% over; the evening's change before it, against the afternoon's
  run, had every stereo recording's mean fall by 1% to 3%, the 1962
  speech's by 15% and the made speech's by 16% to 26%, and the tone rise 3%
  to 6%, the fourth block's two fits more now that it has no fixed coding
  to hide behind.
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
- **Under Windows, natively, this encoder's times are the WSL run's within
  3% and its bytes the same, and so are libFLAC's, run again the same
  night: the tables' WSL run caught its level 0 at a fast moment.** The
  same run on the same i5 under Windows 11, in the last section, made on
  demand: this encoder's mean block is 0.97 to 1.05 of the WSL run's on
  every made sound and recording, so WSL2 costs it nothing a block, and its
  bytes are the same to a tenth of a byte. In that run libFLAC's level 0
  came out 5% to 16% over the WSL run's on every sound but the stereo
  silences, and its level 5 29% over on pink noise. Run again under each
  system in turn, levels 0 and 5 and this encoder, the same night, level 0
  under Windows is 0.90 to 1.08 of the WSL rerun's with a median of 1.00,
  level 5 0.90 to 1.05 and this encoder 0.89 to 1.08: nothing is slower
  under Windows. What differed was the tables' WSL run, whose level 0 the
  WSL rerun is over by up to 18% on every sound with anything in it, a
  median of 11%, where it is over its level 5 by 4% at the median and 10%
  at most and over this encoder by nothing at the median; level 0's time
  under WSL moves by a tenth from run to run on this machine, as the run
  before had already shown, and the tables' run was a fast one, so the
  Windows section's level-0 column, 0.79 to 1.04 on the 16-bit stereo
  recordings and 1.06 to 1.18 on the 24-bit four with sound in them, is
  where WSL's lands on a run with level 0 at its usual time, and the
  tables' 0.85 to 1.11 and 1.16 to 1.33 are of a fast one. The pink noise
  was the Windows run's one disturbed stretch, level 5 at 58 µs and level
  0 at 28 where the rerun had 47 and 23, as WSL does, and this encoder's
  26.3 stood. Against level 5 the recordings are 0.33 to 0.56 under Windows
  where WSL is 0.34 to 0.58, and the made sounds 0.27 to 0.53.
  Windows times more quietly: this encoder's 99th-percentile and worst
  blocks are lower there on every recording but the Aria, 8 to 79 µs and
  60 to 183 against 13 to 77 and 78 to 224, and libFLAC's worst block is
  118 to 391 µs where WSL had 166 to 3550, so the worst blocks of the WSL
  run past a few hundred µs are the machine's. libFLAC's 24-bit tone comes
  to other bytes under MSVC, 2465 against 2321 at level 5 and 2279 against
  2302 at level 8, where this encoder's are the same.

## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.40}$ | $\color{green}{0.40}$ | $\color{green}{0.46}$ | $\color{green}{0.33}$ | $\color{green}{0.47}$ | $\color{green}{0.46}$ |
| speech | $\color{green}{0.30}$ | $\color{green}{0.30}$ | $\color{green}{0.34}$ | $\color{green}{0.35}$ | $\color{green}{0.33}$ | $\color{green}{0.36}$ |
| tone | $\color{green}{0.53}$ | $\color{green}{0.50}$ | $\color{green}{0.55}$ | $\color{green}{0.42}$ | $\color{green}{0.49}$ | $\color{green}{0.52}$ |
| noise | $\color{green}{0.30}$ | $\color{green}{0.30}$ | $\color{green}{0.35}$ | $\color{green}{0.27}$ | $\color{green}{0.30}$ | $\color{green}{0.36}$ |
| silence | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.10}$ | $\color{green}{0.08}$ | $\color{green}{0.07}$ | $\color{green}{0.07}$ |

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
| music | $\color{green}{0.94}$ | $\color{green}{0.85}$ | $\color{red}{1.17}$ | $\color{green}{0.61}$ | $\color{red}{1.16}$ | $\color{red}{1.08}$ |
| speech | $\color{green}{0.58}$ | $\color{green}{0.55}$ | $\color{green}{0.69}$ | $\color{green}{0.68}$ | $\color{green}{0.70}$ | $\color{green}{0.68}$ |
| tone | $\color{red}{1.28}$ | $\color{red}{1.10}$ | $\color{red}{1.40}$ | $\color{green}{0.83}$ | $\color{red}{1.20}$ | $\color{red}{1.20}$ |
| noise | $\color{green}{0.67}$ | $\color{green}{0.64}$ | $\color{green}{0.83}$ | $\color{green}{0.52}$ | $\color{green}{0.71}$ | $\color{green}{0.78}$ |
| silence | $\color{green}{0.14}$ | $\color{green}{0.14}$ | $\color{green}{0.21}$ | $\color{green}{0.14}$ | $\color{green}{0.14}$ | $\color{green}{0.14}$ |

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
| music | 42.3 | $\color{green}{17.1}$ | $\color{green}{0.40}$ | 0.09% | 1838 | 1838 | 1.00 | 47.9% |
| speech | 36.2 | $\color{green}{10.8}$ | $\color{green}{0.30}$ | 0.05% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 40.7 | $\color{green}{21.7}$ | $\color{green}{0.53}$ | 0.11% | 1008 | 1008 | 1.00 | 26.2% |
| noise | 41.5 | $\color{green}{12.6}$ | $\color{green}{0.30}$ | 0.06% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 25.1 | $\color{green}{1.9}$ | $\color{green}{0.07}$ | 0.01% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 36.7 | $\color{green}{14.6}$ | $\color{green}{0.40}$ | 0.07% | 1702 | 1702 | 1.00 | 48.2% |
| speech | 31.4 | $\color{green}{9.3}$ | $\color{green}{0.30}$ | 0.05% | 1063 | $\color{red}{1064}$ | 1.00 | 30.1% |
| tone | 35.5 | $\color{green}{17.7}$ | $\color{green}{0.50}$ | 0.09% | 920 | $\color{red}{940}$ | $\color{red}{1.02}$ | 26.6% |
| noise | 35.6 | $\color{green}{10.8}$ | $\color{green}{0.30}$ | 0.05% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 23.6 | $\color{green}{1.7}$ | $\color{green}{0.07}$ | 0.01% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 70.7 | $\color{green}{32.7}$ | $\color{green}{0.46}$ | 0.16% | 3454 | 3454 | 1.00 | 45.0% |
| speech | 59.6 | $\color{green}{20.3}$ | $\color{green}{0.34}$ | 0.10% | 2226 | $\color{red}{2228}$ | 1.00 | 29.0% |
| tone | 67.8 | $\color{green}{37.0}$ | $\color{green}{0.55}$ | 0.18% | 1651 | $\color{green}{1576}$ | $\color{green}{0.95}$ | 20.5% |
| noise | 70.1 | $\color{green}{24.5}$ | $\color{green}{0.35}$ | 0.12% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 43.0 | $\color{green}{4.1}$ | $\color{green}{0.10}$ | 0.02% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 22.3 | $\color{green}{7.3}$ | $\color{green}{0.33}$ | 0.04% | 950 | 950 | 1.00 | 49.5% |
| speech | 22.8 | $\color{green}{8.1}$ | $\color{green}{0.35}$ | 0.04% | 1148 | 1148 | 1.00 | 59.8% |
| tone | 21.6 | $\color{green}{8.9}$ | $\color{green}{0.42}$ | 0.04% | 512 | 512 | 1.00 | 26.7% |
| noise | 22.3 | $\color{green}{6.1}$ | $\color{green}{0.27}$ | 0.03% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 17.1 | $\color{green}{1.3}$ | $\color{green}{0.08}$ | 0.01% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 40.6 | $\color{green}{19.2}$ | $\color{green}{0.47}$ | 0.10% | 456 | 456 | 1.00 | 23.7% |
| speech | 35.0 | $\color{green}{11.6}$ | $\color{green}{0.33}$ | 0.06% | 250 | 250 | 1.00 | 13.0% |
| tone | 39.0 | $\color{green}{19.1}$ | $\color{green}{0.49}$ | 0.10% | 476 | $\color{green}{471}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 39.5 | $\color{green}{11.8}$ | $\color{green}{0.30}$ | 0.06% | 1002 | $\color{red}{1006}$ | 1.00 | 52.4% |
| silence | 24.9 | $\color{green}{1.8}$ | $\color{green}{0.07}$ | 0.01% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 45.7 | $\color{green}{20.9}$ | $\color{green}{0.46}$ | 0.10% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 38.3 | $\color{green}{13.9}$ | $\color{green}{0.36}$ | 0.07% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 43.7 | $\color{green}{22.6}$ | $\color{green}{0.52}$ | 0.11% | 2321 | $\color{red}{2440}$ | $\color{red}{1.05}$ | 42.4% |
| noise | 43.9 | $\color{green}{15.8}$ | $\color{green}{0.36}$ | 0.08% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 26.0 | $\color{green}{1.8}$ | $\color{green}{0.07}$ | 0.01% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{green}{0.51}$ | 1.00 | $\color{red}{1.24}$ | $\color{green}{0.95}$ | $\color{green}{0.18}$ | 1.00 |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.45}$ | 1.00 | $\color{red}{1.11}$ | $\color{green}{0.87}$ | $\color{green}{0.17}$ | $\color{red}{1.01}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.46}$ | 1.00 | $\color{red}{1.07}$ | $\color{green}{0.78}$ | $\color{green}{0.16}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.51}$ | 1.00 | $\color{red}{1.16}$ | $\color{green}{0.96}$ | $\color{green}{0.17}$ | 1.00 |
| Mix at −13 LUFS, wide stereo | 44.1 kHz stereo 16-bit | $\color{green}{0.41}$ | 1.00 | $\color{green}{0.87}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Mix at −9 LUFS, loud master | 44.1 kHz stereo 16-bit | $\color{green}{0.41}$ | 1.00 | $\color{green}{0.85}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | $\color{green}{0.58}$ | 1.00 | $\color{red}{1.33}$ | $\color{green}{0.96}$ | $\color{green}{0.22}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.42}$ | 1.00 | $\color{green}{0.90}$ | $\color{green}{0.88}$ | $\color{green}{0.18}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.43}$ | 1.00 | $\color{green}{0.91}$ | $\color{green}{0.78}$ | $\color{green}{0.19}$ | $\color{red}{1.01}$ |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.34}$ | 1.00 | $\color{green}{0.61}$ | $\color{green}{0.97}$ | $\color{green}{0.25}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.36}$ | 1.00 | $\color{green}{0.70}$ | $\color{green}{0.49}$ | $\color{green}{0.14}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.41}$ | 1.00 | $\color{green}{0.86}$ | $\color{green}{0.95}$ | $\color{green}{0.18}$ | 1.00 |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.58}$ | 1.00 | $\color{red}{1.25}$ | $\color{green}{0.99}$ | $\color{green}{0.21}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.37}$ | 1.00 | $\color{green}{0.79}$ | $\color{green}{0.99}$ | $\color{green}{0.14}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms. *The run before* is this
encoder's mean block in the run of the commit before this one, and *over it*
this run's over that.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | The run before, mean µs | Over it | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 77.9 | $\color{green}{39.7}$ | $\color{green}{0.51}$ | 41.3 | $\color{green}{0.96}$ | 132 | $\color{green}{68.4}$ | 279 | $\color{green}{174}$ | 0.20% |
| Chopin, Fantasy Op. 49, piano | 71.2 | $\color{green}{32.3}$ | $\color{green}{0.45}$ | 33.0 | $\color{green}{0.98}$ | 127 | $\color{green}{53.4}$ | 3550 | $\color{green}{223}$ | 0.16% |
| Shostakovich 5, orchestra, live | 42.2 | $\color{green}{19.3}$ | $\color{green}{0.46}$ | 20.0 | $\color{green}{0.97}$ | 73.6 | $\color{green}{36.2}$ | 206 | $\color{green}{118}$ | 0.10% |
| Indie rock, 2022, mastered | 47.4 | $\color{green}{24.4}$ | $\color{green}{0.51}$ | 24.5 | $\color{green}{0.99}$ | 79.4 | $\color{green}{42.3}$ | 216 | $\color{green}{145}$ | 0.12% |
| Mix at −13 LUFS, wide stereo | 37.5 | $\color{green}{15.5}$ | $\color{green}{0.41}$ | 15.9 | $\color{green}{0.97}$ | 70.6 | $\color{green}{31.1}$ | 170 | $\color{green}{117}$ | 0.08% |
| Mix at −9 LUFS, loud master | 37.1 | $\color{green}{15.3}$ | $\color{green}{0.41}$ | 15.9 | $\color{green}{0.96}$ | 67.0 | $\color{green}{31.6}$ | 191 | $\color{green}{101}$ | 0.08% |
| Limehouse Blues, 1924 78 rpm | 81.6 | $\color{green}{47.1}$ | $\color{green}{0.58}$ | 46.7 | $\color{red}{1.01}$ | 142 | $\color{green}{77.2}$ | 344 | $\color{green}{156}$ | 0.24% |
| Prison work song, 1947 field recording | 36.9 | $\color{green}{15.5}$ | $\color{green}{0.42}$ | 16.1 | $\color{green}{0.96}$ | 62.1 | $\color{green}{31.4}$ | 292 | $\color{green}{87.4}$ | 0.08% |
| The Raven, read, produced | 36.8 | $\color{green}{16.0}$ | $\color{green}{0.43}$ | 16.1 | $\color{green}{0.99}$ | 68.4 | $\color{green}{32.3}$ | 207 | $\color{green}{155}$ | 0.08% |
| Leviticus 13, read | 21.1 | $\color{green}{7.1}$ | $\color{green}{0.34}$ | 7.5 | $\color{green}{0.95}$ | 39.5 | $\color{green}{12.7}$ | 258 | $\color{green}{108}$ | 0.04% |
| JFK at Rice, 1962 | 39.0 | $\color{green}{14.1}$ | $\color{green}{0.36}$ | 14.4 | $\color{green}{0.98}$ | 70.6 | $\color{green}{29.7}$ | 270 | $\color{green}{116}$ | 0.07% |
| Train station, field recording | 36.4 | $\color{green}{14.7}$ | $\color{green}{0.41}$ | 15.1 | $\color{green}{0.98}$ | 60.7 | $\color{green}{31.2}$ | 166 | $\color{green}{78.0}$ | 0.07% |
| Pink noise | 45.0 | $\color{green}{26.0}$ | $\color{green}{0.58}$ | 25.8 | $\color{red}{1.01}$ | 80.1 | $\color{green}{44.9}$ | 444 | $\color{green}{224}$ | 0.13% |
| White noise | 44.5 | $\color{green}{16.4}$ | $\color{green}{0.37}$ | 16.1 | $\color{red}{1.02}$ | 79.0 | $\color{green}{31.8}$ | 317 | $\color{green}{147}$ | 0.08% |

### The bytes, libFLAC at level 5

*The run before* is this encoder's bytes a block in the run of the commit
before this one, and *over it* this run's over that, to a ten-thousandth.

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | The run before, bytes | Over it | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5318}$ | $\color{red}{1.001}$ | 5318 | 1.0000 | 46.2% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2482}$ | $\color{red}{1.002}$ | 2482 | 1.0000 | 32.3% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1499}$ | 1.000 | 1499 | 1.0000 | 39.0% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{red}{4556}$ | 1.000 | 4556 | 1.0000 | 79.1% |
| Mix at −13 LUFS, wide stereo | 250 | 3528 | 2325 | $\color{red}{2327}$ | $\color{red}{1.001}$ | 2327 | 1.0000 | 66.0% |
| Mix at −9 LUFS, loud master | 225 | 3528 | 2543 | $\color{red}{2547}$ | $\color{red}{1.001}$ | 2547 | 1.0000 | 72.2% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | $\color{green}{8657}$ | 1.000 | 8657 | 1.0000 | 75.1% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | $\color{red}{1421}$ | $\color{red}{1.001}$ | 1421 | 1.0000 | 40.3% |
| The Raven, read, produced | 559 | 3528 | 1362 | $\color{red}{1364}$ | $\color{red}{1.001}$ | 1364 | 1.0000 | 38.7% |
| Leviticus 13, read | 725 | 1764 | 986 | $\color{red}{987}$ | 1.000 | 987 | 1.0000 | 55.9% |
| JFK at Rice, 1962 | 836 | 5760 | 1798 | $\color{green}{1797}$ | 1.000 | 1797 | 1.0000 | 31.2% |
| Train station, field recording | 141 | 3528 | 1272 | $\color{red}{1273}$ | 1.000 | 1273 | 1.0000 | 36.1% |
| Pink noise | 600 | 5760 | 4901 | $\color{green}{4900}$ | 1.000 | 4900 | 1.0000 | 85.1% |
| White noise | 600 | 5760 | 5387 | 5387 | 1.000 | 5387 | 1.0000 | 93.5% |

## On arm64

The same run on an arm64 machine, beside the x86 run above: this encoder's
time a block over libFLAC's on each. A ratio under 1.00 is less of a core
than libFLAC on that machine, and a block's µs are that machine's.

### Time a block on arm64, this encoder over libFLAC at level 5

Under 1.00 is less of a core than libFLAC at its default level, on arm64.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.36}$ | $\color{green}{0.35}$ | $\color{green}{0.37}$ | $\color{green}{0.35}$ | $\color{green}{0.45}$ | $\color{green}{0.40}$ |
| speech | $\color{green}{0.26}$ | $\color{green}{0.26}$ | $\color{green}{0.28}$ | $\color{green}{0.37}$ | $\color{green}{0.30}$ | $\color{green}{0.29}$ |
| tone | $\color{green}{0.48}$ | $\color{green}{0.44}$ | $\color{green}{0.45}$ | $\color{green}{0.44}$ | $\color{green}{0.46}$ | $\color{green}{0.41}$ |
| noise | $\color{green}{0.27}$ | $\color{green}{0.26}$ | $\color{green}{0.29}$ | $\color{green}{0.28}$ | $\color{green}{0.26}$ | $\color{green}{0.31}$ |
| silence | $\color{green}{0.08}$ | $\color{green}{0.08}$ | $\color{green}{0.09}$ | $\color{green}{0.09}$ | $\color{green}{0.08}$ | $\color{green}{0.07}$ |

### Time a block on arm64, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on arm64.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.10}$ | 1.00 | $\color{red}{1.24}$ | $\color{green}{0.79}$ | $\color{red}{1.38}$ | $\color{red}{1.14}$ |
| speech | $\color{green}{0.65}$ | $\color{green}{0.61}$ | $\color{green}{0.73}$ | $\color{green}{0.82}$ | $\color{green}{0.80}$ | $\color{green}{0.66}$ |
| tone | $\color{red}{1.48}$ | $\color{red}{1.29}$ | $\color{red}{1.46}$ | $\color{red}{1.05}$ | $\color{red}{1.42}$ | $\color{red}{1.16}$ |
| noise | $\color{green}{0.74}$ | $\color{green}{0.66}$ | $\color{green}{0.82}$ | $\color{green}{0.61}$ | $\color{green}{0.76}$ | $\color{green}{0.78}$ |
| silence | $\color{green}{0.19}$ | $\color{green}{0.18}$ | $\color{green}{0.21}$ | $\color{green}{0.19}$ | $\color{green}{0.18}$ | $\color{green}{0.16}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 |
|---|---:|---:|---:|---:|---:|---:|
| music | 42.3 | 47.0 | 17.1 | 16.8 | $\color{green}{0.40}$ | $\color{green}{0.36}$ |
| speech | 36.2 | 39.8 | 10.8 | 10.4 | $\color{green}{0.30}$ | $\color{green}{0.26}$ |
| tone | 40.7 | 44.9 | 21.7 | 21.6 | $\color{green}{0.53}$ | $\color{green}{0.48}$ |
| noise | 41.5 | 43.3 | 12.6 | 11.7 | $\color{green}{0.30}$ | $\color{green}{0.27}$ |
| silence | 25.1 | 23.8 | 1.9 | 2.0 | $\color{green}{0.07}$ | $\color{green}{0.08}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 | 99th, x86 | 99th, arm64 | Time vs 0, x86 | Time vs 0, arm64 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 77.9 | 93.4 | 39.7 | 37.7 | $\color{green}{0.51}$ | $\color{green}{0.40}$ | 68.4 | 54.9 | $\color{red}{1.24}$ | $\color{red}{1.16}$ |
| Chopin, Fantasy Op. 49, piano | 71.2 | 85.9 | 32.3 | 32.4 | $\color{green}{0.45}$ | $\color{green}{0.38}$ | 53.4 | 49.4 | $\color{red}{1.11}$ | $\color{red}{1.21}$ |
| Shostakovich 5, orchestra, live | 42.2 | 46.1 | 19.3 | 19.4 | $\color{green}{0.46}$ | $\color{green}{0.42}$ | 36.2 | 30.9 | $\color{red}{1.07}$ | $\color{red}{1.21}$ |
| Indie rock, 2022, mastered | 47.4 | 53.1 | 24.4 | 22.9 | $\color{green}{0.51}$ | $\color{green}{0.43}$ | 42.3 | 35.3 | $\color{red}{1.16}$ | $\color{red}{1.14}$ |
| Mix at −13 LUFS, wide stereo | 37.5 | 42.0 | 15.5 | 15.6 | $\color{green}{0.41}$ | $\color{green}{0.37}$ | 31.1 | 24.6 | $\color{green}{0.87}$ | $\color{red}{1.01}$ |
| Mix at −9 LUFS, loud master | 37.1 | 41.8 | 15.3 | 15.4 | $\color{green}{0.41}$ | $\color{green}{0.37}$ | 31.6 | 22.2 | $\color{green}{0.85}$ | $\color{green}{0.98}$ |
| Limehouse Blues, 1924 78 rpm | 81.6 | 96.5 | 47.1 | 44.8 | $\color{green}{0.58}$ | $\color{green}{0.46}$ | 77.2 | 61.2 | $\color{red}{1.33}$ | $\color{red}{1.23}$ |
| Prison work song, 1947 field recording | 36.9 | 41.0 | 15.5 | 15.3 | $\color{green}{0.42}$ | $\color{green}{0.37}$ | 31.4 | 20.6 | $\color{green}{0.90}$ | $\color{red}{1.05}$ |
| The Raven, read, produced | 36.8 | 41.3 | 16.0 | 15.6 | $\color{green}{0.43}$ | $\color{green}{0.38}$ | 32.3 | 23.8 | $\color{green}{0.91}$ | $\color{red}{1.05}$ |
| Leviticus 13, read | 21.1 | 19.4 | 7.1 | 7.0 | $\color{green}{0.34}$ | $\color{green}{0.36}$ | 12.7 | 8.4 | $\color{green}{0.61}$ | $\color{green}{0.75}$ |
| JFK at Rice, 1962 | 39.0 | 43.1 | 14.1 | 13.5 | $\color{green}{0.36}$ | $\color{green}{0.31}$ | 29.7 | 26.2 | $\color{green}{0.70}$ | $\color{green}{0.71}$ |
| Train station, field recording | 36.4 | 40.6 | 14.7 | 14.5 | $\color{green}{0.41}$ | $\color{green}{0.36}$ | 31.2 | 20.3 | $\color{green}{0.86}$ | 1.00 |
| Pink noise | 45.0 | 49.9 | 26.0 | 23.8 | $\color{green}{0.58}$ | $\color{green}{0.48}$ | 44.9 | 32.3 | $\color{red}{1.25}$ | $\color{red}{1.20}$ |
| White noise | 44.5 | 49.1 | 16.4 | 15.6 | $\color{green}{0.37}$ | $\color{green}{0.32}$ | 31.8 | 25.6 | $\color{green}{0.79}$ | $\color{green}{0.79}$ |

The bytes a block of this encoder are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 5 differ between the two runs on tone at 48 kHz stereo 24-bit (2321 against 2315).

The bytes a block of libFLAC at level 0 are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 8 differ between the two runs on tone at 48 kHz stereo 24-bit (2302 against 2502).

## On Windows

The same run on the same machine under Windows, natively, beside the run
above, which was under WSL2 on it: this encoder's time a block over
libFLAC's on each. A ratio under 1.00 is less of a core than libFLAC under
that system, and a block's µs are that system's. This run is made on
demand, not with every change to the tuning as the two above are, so it can
be of an earlier encoder than theirs; the prose above says which.

### Time a block on Windows, this encoder over libFLAC at level 5

Under 1.00 is less of a core than libFLAC at its default level, on Windows.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.39}$ | $\color{green}{0.42}$ | $\color{green}{0.47}$ | $\color{green}{0.32}$ | $\color{green}{0.50}$ | $\color{green}{0.46}$ |
| speech | $\color{green}{0.30}$ | $\color{green}{0.31}$ | $\color{green}{0.35}$ | $\color{green}{0.34}$ | $\color{green}{0.33}$ | $\color{green}{0.34}$ |
| tone | $\color{green}{0.53}$ | $\color{green}{0.53}$ | $\color{green}{0.53}$ | $\color{green}{0.41}$ | $\color{green}{0.50}$ | $\color{green}{0.50}$ |
| noise | $\color{green}{0.31}$ | $\color{green}{0.32}$ | $\color{green}{0.37}$ | $\color{green}{0.27}$ | $\color{green}{0.30}$ | $\color{green}{0.35}$ |
| silence | $\color{green}{0.08}$ | $\color{green}{0.08}$ | $\color{green}{0.11}$ | $\color{green}{0.08}$ | $\color{green}{0.08}$ | $\color{green}{0.08}$ |

### Time a block on Windows, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on Windows.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.86}$ | $\color{green}{0.80}$ | $\color{red}{1.09}$ | $\color{green}{0.55}$ | $\color{red}{1.09}$ | $\color{green}{0.99}$ |
| speech | $\color{green}{0.54}$ | $\color{green}{0.49}$ | $\color{green}{0.63}$ | $\color{green}{0.58}$ | $\color{green}{0.63}$ | $\color{green}{0.60}$ |
| tone | $\color{red}{1.18}$ | 1.00 | $\color{red}{1.23}$ | $\color{green}{0.76}$ | $\color{red}{1.09}$ | $\color{red}{1.11}$ |
| noise | $\color{green}{0.66}$ | $\color{green}{0.57}$ | $\color{green}{0.75}$ | $\color{green}{0.46}$ | $\color{green}{0.67}$ | $\color{green}{0.73}$ |
| silence | $\color{green}{0.16}$ | $\color{green}{0.15}$ | $\color{green}{0.24}$ | $\color{green}{0.13}$ | $\color{green}{0.16}$ | $\color{green}{0.15}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, WSL µs | libFLAC, Windows µs | This encoder, WSL µs | This encoder, Windows µs | Ratio, WSL | Ratio, Windows |
|---|---:|---:|---:|---:|---:|---:|
| music | 42.3 | 43.8 | 17.1 | 17.2 | $\color{green}{0.40}$ | $\color{green}{0.39}$ |
| speech | 36.2 | 36.2 | 10.8 | 10.9 | $\color{green}{0.30}$ | $\color{green}{0.30}$ |
| tone | 40.7 | 41.4 | 21.7 | 22.1 | $\color{green}{0.53}$ | $\color{green}{0.53}$ |
| noise | 41.5 | 42.1 | 12.6 | 13.0 | $\color{green}{0.30}$ | $\color{green}{0.31}$ |
| silence | 25.1 | 23.0 | 1.9 | 1.9 | $\color{green}{0.07}$ | $\color{green}{0.08}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, WSL µs | libFLAC, Windows µs | This encoder, WSL µs | This encoder, Windows µs | Ratio, WSL | Ratio, Windows | 99th, WSL | 99th, Windows | Time vs 0, WSL | Time vs 0, Windows |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 77.9 | 78.5 | 39.7 | 41.6 | $\color{green}{0.51}$ | $\color{green}{0.53}$ | 68.4 | 79.1 | $\color{red}{1.24}$ | $\color{red}{1.18}$ |
| Chopin, Fantasy Op. 49, piano | 71.2 | 69.7 | 32.3 | 32.3 | $\color{green}{0.45}$ | $\color{green}{0.46}$ | 53.4 | 51.5 | $\color{red}{1.11}$ | $\color{red}{1.04}$ |
| Shostakovich 5, orchestra, live | 42.2 | 42.2 | 19.3 | 19.7 | $\color{green}{0.46}$ | $\color{green}{0.47}$ | 36.2 | 33.9 | $\color{red}{1.07}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 47.4 | 48.7 | 24.4 | 24.4 | $\color{green}{0.51}$ | $\color{green}{0.50}$ | 42.3 | 38.6 | $\color{red}{1.16}$ | $\color{red}{1.06}$ |
| Mix at −13 LUFS, wide stereo | 37.5 | 35.3 | 15.5 | 15.5 | $\color{green}{0.41}$ | $\color{green}{0.44}$ | 31.1 | 24.7 | $\color{green}{0.87}$ | $\color{green}{0.81}$ |
| Mix at −9 LUFS, loud master | 37.1 | 35.2 | 15.3 | 15.4 | $\color{green}{0.41}$ | $\color{green}{0.44}$ | 31.6 | 20.9 | $\color{green}{0.85}$ | $\color{green}{0.79}$ |
| Limehouse Blues, 1924 78 rpm | 81.6 | 83.3 | 47.1 | 46.7 | $\color{green}{0.58}$ | $\color{green}{0.56}$ | 77.2 | 67.1 | $\color{red}{1.33}$ | $\color{red}{1.18}$ |
| Prison work song, 1947 field recording | 36.9 | 35.1 | 15.5 | 15.3 | $\color{green}{0.42}$ | $\color{green}{0.44}$ | 31.4 | 22.3 | $\color{green}{0.90}$ | $\color{green}{0.83}$ |
| The Raven, read, produced | 36.8 | 35.1 | 16.0 | 15.7 | $\color{green}{0.43}$ | $\color{green}{0.45}$ | 32.3 | 24.8 | $\color{green}{0.91}$ | $\color{green}{0.82}$ |
| Leviticus 13, read | 21.1 | 21.3 | 7.1 | 7.1 | $\color{green}{0.34}$ | $\color{green}{0.33}$ | 12.7 | 8.5 | $\color{green}{0.61}$ | $\color{green}{0.55}$ |
| JFK at Rice, 1962 | 39.0 | 39.8 | 14.1 | 14.1 | $\color{green}{0.36}$ | $\color{green}{0.35}$ | 29.7 | 28.9 | $\color{green}{0.70}$ | $\color{green}{0.63}$ |
| Train station, field recording | 36.4 | 34.5 | 14.7 | 14.8 | $\color{green}{0.41}$ | $\color{green}{0.43}$ | 31.2 | 20.2 | $\color{green}{0.86}$ | $\color{green}{0.81}$ |
| Pink noise | 45.0 | 58.1 | 26.0 | 26.3 | $\color{green}{0.58}$ | $\color{green}{0.45}$ | 44.9 | 41.7 | $\color{red}{1.25}$ | $\color{green}{0.94}$ |
| White noise | 44.5 | 46.4 | 16.4 | 16.6 | $\color{green}{0.37}$ | $\color{green}{0.36}$ | 31.8 | 27.8 | $\color{green}{0.79}$ | $\color{green}{0.72}$ |

The bytes a block of this encoder are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 5 differ between the two runs on tone at 48 kHz stereo 24-bit (2321 against 2465).

The bytes a block of libFLAC at level 0 are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 8 differ between the two runs on tone at 48 kHz stereo 24-bit (2302 against 2279).

