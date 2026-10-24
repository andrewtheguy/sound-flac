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

In the recordings tables below, *the run before* is the run of the commit
before this one, made the afternoon of 2026-10-23 with the encoder of
v0.0.6, whose log that commit carries as `docs/bench/x86.log`. The days
named in this document and in [plan.md](plan.md) before this run, the
night of 2026-10-23 and 2026-10-24 among them, are the count the sessions
that made the earlier runs kept, which ran ahead of the machine's clock:
by the commits, which that clock stamps in Pacific time, the run of v0.0.5
was made at noon on 2026-10-23, the run before this one that afternoon,
and this run that evening, which is the clock's own day and is named by it
from here on; the whole of the encoder's work from the plan to this run
ran from the evening of 2026-10-22 to that evening.

They were run on 2026-10-23 by the machine's clock, the evening, on a
six-core Intel i5-8500T under WSL2, one thread, WSL restarted for the run
and the machine idle but for the terminal session it was started from,
with rustc 1.98.1 at the x86-64 baseline (SSE2, no `target-cpu`), on
`main` at the commit that carries them: the encoder of v0.0.6 with the
evening's change of [plan.md](plan.md), the stereo pair chosen on the
four channels' bits counted in one Rice partition where the estimates do
not settle it. A time on this machine moves by up to a tenth from one run
to the next, so a time ratio within a tenth of 1.00 is on the line; a
bytes ratio is exact. libFLAC's own times are the control: in this run
its level 5 is 3% to 9% under the run before's on every sound but
silence, where it is within 4%, its level 0 7% to 15% under, and its
level 8 within 6%, so this run caught levels 5 and 0 at a fast moment
where the run before had caught them at a slow one, and this encoder's
ratios to them read worse by that much over what it spent: its own block
is 0.99 to 1.05 of the run before's on the recordings, pink noise the
1.05, the wide mix 1.04, the orchestra 1.00 and the mono reading 0.99,
and 0.95 to 1.05 of it on the made sounds. The run is one whole run,
nothing in it measured again; an earlier run of the evening from the same
tree, before WSL was restarted, agrees with it within 3% on this
encoder's lines and 5% on libFLAC's, and to the byte, and its bytes are
the arm64 run's to a tenth of a byte on every line. A worst block
is one block of thousands and the machine's as much as the encoder's, even
as the least over passes, so it is read as an order of magnitude. The bench
also logs `worst_at`, the block that was slowest at its least over the
passes, and dumps every block's least time and bytes when asked
(`BLOCKS=dir`), which is how the late blocks below were read; every pass
starts from an encoder that has seen no block, so that what the encoder
carries from block to block, the fourth-block choice, falls on the same
blocks in every pass and the least over the passes is the block's own.

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

The Windows run of the section after it was made the same day, after the
two above, on this same i5-8500T under Windows 11 Pro, natively, with rustc
1.93 MSVC at the x86-64 baseline and libFLAC from its sys crate's Windows
archive, level 5 through v0.0.4 of this crate and levels 0 and 8 through
the bench's own link of the same crate at the same tag, as on the two
systems above, from the tree of this commit copied over the staging clone
of `main` there: the recordings copied from the store to
the clone for the run and deleted after it, the machine idle but for the
WSL session the run was started from. Unlike the x86 and arm64 runs, it
is made on demand, not with every change to the tuning, so its section
can be of an earlier encoder than the tables above; this paragraph says
which, and this time it is the encoder of v0.0.6, the run before's, whose
Windows run was made the afternoon of 2026-10-23 and is kept as it was:
the pair choice of this run's encoder was not run under Windows. The night before, with
the encoder of v0.0.5, levels 0 and 5 and this encoder were run again
under both systems in turn, Windows first and WSL after it, to read that
night's level 0 against that night's WSL run, the run before this one;
those logs are not kept, and what they showed is in the last point of
the next section.

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
  at its default level takes of a core or less, most of them well under
  half.** The recordings are 0.30 to 0.57 of its time a block: 16-bit
  stereo 0.41 to 0.46, mono 0.34, 24-bit 0.30 to 0.57, the two at 96 kHz
  and 24 bits 0.53 and 0.54. A block of a recording is 0.03% to 0.22% of
  one core. The made sounds are 0.25 to 0.53: music 0.32 to 0.51, speech
  0.29 to 0.35, the tone 0.41 to 0.53, noise 0.25 to 0.33, silence 0.05 to
  0.07. The run before, the afternoon of 2026-10-23 and the encoder of
  v0.0.6, had them at 0.28 to 0.52 and 0.23 to 0.52, the run of v0.0.5
  before it at 0.34 to 0.58 and 0.27 to 0.55, the evening's before that
  0.33 to 0.55 and 0.27 to 0.53, the afternoon's 0.31 to 0.59 and 0.25 to
  0.49, the morning's 0.44 to 0.77 and 0.40 to 0.74, for the same bytes
  within half a per cent but the tone's, which fell, and in this run the
  pianos', which fell by a tenth of a per cent. Against the run before
  this encoder's own block is 0.99 to 1.05 of its on the recordings, pink
  noise 1.05, the wide mix 1.04, the orchestra 1.00 and the mono reading
  0.99, and 0.95 to 1.05 on the made sounds, the 48 kHz speech 0.95, the
  48 kHz music 1.05; libFLAC's own, run beside it, is 3% to 9% under its
  run before's, so the ratios to it read worse by more than the encoder
  moved. On arm64, where libFLAC's held within 2%, this encoder's is
  0.99 to 1.04 of the run before's on the recordings, pink noise the 1.04
  and the Fantasy 1.03, and 0.96 to 1.04 on the made sounds, the 8-bit
  music the 1.04, where the fit is never clear and all four channels are
  always counted.
- **The bytes are libFLAC's at its default level, within 0.2%, and on a
  pure tone under them.** Every recording is 1.000 but the Fantasy, the
  prison song and the two private mixes at 1.001, a byte to four a block,
  which is what fitting only the carried pair between full choices and
  coding it the fit first cost them (the measurements are in
  [plan.md](plan.md)); before the pair was chosen on the fit the train
  station was 1.006 and the Fantasy 1.004, and before it was chosen on the
  four channels' counts the Fantasy was 1.002 and the Aria 1.001. Against
  the run before every recording comes to the same bytes or fewer, the
  Fantasy 0.9983, the Aria 0.9989, the train station 0.9996, the Raven and
  the prison song 0.9999, which is the pair chosen on the counts where the
  estimates could not tell left and right from mid and side, and every
  made sound to the same bytes but the pure tone at 96 kHz, 0.990 of the
  run before's. The tone is 0.99 of libFLAC's at 48 kHz, 1.00 at
  44.1 kHz where it was 1.02, 0.90 at 96 kHz where it was 0.95, 0.98 at 24
  bits where it was 1.05 and 0.99 at 8 bits: a tone is predicted away by
  order 2 and what its frame then costs is the rounding of the
  coefficients, which the order choice now weighs exactly, from the
  autocorrelation, where before it was blind to it and the orders tied.
  The made speech, whose two channels are one signal, is 0.05% over
  libFLAC and 0.16% at 8 bits: its side channel is a constant, and a block
  with a constant channel is coded the fit first like any other where it
  had every channel coded both ways. Against libFLAC's smallest level, 8,
  which tries every order to 12 and every stereo pairing in five to nine
  times this encoder's time, the made sounds are 1.00 to 1.01, the tone at
  96 kHz and 24 bits 1.01 and 0.99 where they were 1.07 and 1.06, and the
  recordings 1.00 to 1.01.
- **The late block is still the fourth block, where the pair is chosen
  afresh, by a fifth to a third of a carried block.** Each block's least
  time over the passes, each pass from an encoder that has seen no block,
  read by the block's place in the cycle of four, in a run of this encoder
  alone after the tables' run: the fourth block is 21.9
  µs against 17.1 for a carried block on the orchestra, where the run
  before had 21.9 against 16.9; 48.2 against 36.1 on the Aria, from 43.5
  against 34.5; 37.2 against 30.0 on the Fantasy; 18.7 against 14.4 on the
  Raven, from 18.0 against 13.6; 17.6 against 14.6 on the wide mix, from
  16.5 against 13.5; on the made tone 25.7 against 19.0, from 23.7 against
  18.4, and on the made music, where the estimates settle the pair and the
  two other channels are not counted, 17.0 against 15.6. Where the
  estimates do not settle it, the fourth block has the two other channels'
  residuals and folds over what it had, which is the Aria's and the
  Fantasy's rise, and against that the run before's choice coded 1.9 pairs
  on the orchestra's mean fourth block, which is why that block did not
  move there. The two blocks after the
  choice, where each channel of the pair has both codings made once to
  bring its correction up to date, are about 1 µs over a plain carried
  block at 16 bits and 3.5 on the Aria. This encoder's own 99th-percentile
  block is set by the loud blocks as much as by the fourth, 1.4 to 1.5
  times its mean on the stereo recordings, and its worst about twice. A
  single pass's 99th percentile is higher still for both encoders, which
  is scheduling and cache, not the sound. On the recordings this encoder's
  99th-percentile block is 0.22 to 0.53 of libFLAC's, 9 to 70 µs against
  38 to 129, and its worst block 86 to 228 µs, at most 1.1% of the 20 ms
  the block is, where libFLAC's worst is 126 to 2791. Where two channels
  are one signal, the 1962 speech and the made speech, the fourth block
  counts the three equivalent channels, each the same bits within the
  estimates' margin: 19.9 µs against 11.3 on that recording and 12.2
  against 9.1 on the made speech, the blocks between coding one channel
  and a constant.
- **The pair is chosen on the four channels' bits every fourth block, and
  carried between.** On the fourth block every channel is estimated from
  its autocorrelation, the mid's derived from left's, right's and side's
  so three passes serve four channels, and where one pair is estimated
  under every other by a quarter of a bit a sample it is taken. Otherwise
  every channel is counted: its fit's residual is taken and counted in one
  Rice partition, the fold and the finest sums the partition search needs
  anyway, a true count of the channel's bits within what the partitions
  chosen gain, and the fixed predictor of the order the autocorrelation
  chooses is counted the same way beside it where the fit is not clear;
  the pair whose two counts are least is taken, and its two channels are
  finished, the partitions searched and the fixed path run where the fit
  was not clear. That is libFLAC's choice, every pair by its bits, for a
  residual pass and a fold on the two channels not coded. The estimates
  are good at differences within a channel and poor between channels: on
  the piano recordings left and right against mid and side are within a
  twentieth of a bit a sample on 99% of the fourth blocks and the
  estimates' pair is wrong on half of them, and on 8-bit music it is
  wrong on every block; chosen on the estimates and coded again where
  the coded pair's real bits said another was cheaper, as the run before
  did, 1.9 pairs were coded on the mean fourth block of the orchestra for
  a pair that stood on 85% of them, which is where the fourth block's
  time went. The estimates settle every fourth block of the made 16-bit
  and 24-bit music, half the made speech's, a third of the noise's and a
  quarter to a third of the mixes', and none of the piano's; past a fifth
  of a bit no count changed a choice on any sound measured, and an eighth
  was not safe at 8 bits, where all of the 8% of the blocks the estimates
  separate by a tenth are wrong. On the three blocks between, the pair is coded again, each
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
  channels, on the Raven on three in five, on the mixes nine in ten, on
  the Aria at 24 bits nearly all, and on 8-bit music almost none, which
  is an open item of [plan.md](plan.md). The two other channels are not looked at
  past a constant check and a bound from one pass over their differences,
  and the pair moves where the bound says another is certainly cheaper,
  160 times in 12 000 blocks of the wide mix and once in the Raven's. A
  channel on its own, mono or more than two, is coded the fit first on
  every block. Where the fit is estimated not to win on a channel of fixed
  order 0, which is noise and a quiet desktop, it is skipped for seven
  blocks.
- **Where the time goes at 16-bit stereo**, on the orchestra, by the parts
  measured apart with timers in a copy of the tree: a carried block, three
  in four, is the two autocorrelations with their windowing 3.6 µs, the
  fits from them 0.7, the pair's two residuals 2.3 and their partitions
  1.9, the folds and the two bounds 1.2, the stereo split 0.7, the bits
  out 4.4 and the CRC 0.7, about 16 in all where the fit is clear on both
  channels, and a fixed coding of 1.7 for each channel whose fit is not,
  which on the orchestra is two carried channels in three, so about 18 on
  the mean carried block and 19 where neither is clear, the timers' own
  cost in these where the run's blocks, timed whole, put the carried
  block at 16.9; the range check's own pass, 0.9, is gone into the folds.
  The fourth block, about 22 µs, has three autocorrelations, four fits and,
  where the estimates do not settle the pair, which on the orchestra is
  every block but one in a hundred, the two other channels' residuals and
  folds, 1.35 µs each, and the fixed predictor's one-partition count on
  each channel whose fit is not clear, in place of the two
  autocorrelations, two fits and two bounds. At 24 bits the residual of a fit runs in two 16-bit passes
  at twice the 16-bit pass's time, 10 µs of the Aria's 39 µs block, and
  the writer, three codes to a word at the parameters a 24-bit residual
  takes, about 6 µs of pink noise's 24 and 11 of the 1924 record's 42,
  where four to a word, falling to pairs, took 14. A 17-bit side channel
  whose samples fit 16 bits takes the 16-bit pass. A block of noise, where
  order 0 wins, takes one difference pass in the order search where it
  took four.
- **Against libFLAC's fastest level, 0, which fits no predictor and codes
  stereo as it comes**, this encoder takes 0.87 to 1.12 of its time on the
  16-bit stereo recordings, the Fantasy at 96 kHz 1.12 and the orchestra
  1.03, 0.70 on the 1962 speech and 0.64 on white noise, 0.61 on the mono
  reading, and 1.14 to 1.29 on the four at 24 bits with sound in them, for
  0.49 to 0.99 of its bytes: the two stereo speeches 0.49 and 0.78, the
  music 0.78 to 0.96, noise 0.99. On the made sounds it is 0.82 to 1.23 on
  stereo music and 0.93 to 1.27 on the tone, 0.52 to 0.65 on speech, 0.47
  to 0.75 on noise, 0.47 to 0.87 on mono and 0.09 to 0.15 on silence. The
  run before had the 16-bit stereo recordings at 0.78 to 1.01, the 1962
  speech 0.62, white noise 0.58, the mono reading 0.53 and the 24-bit four
  at 1.01 to 1.13, the made stereo music and tone at 0.78 to 1.07 and 0.85
  to 1.15: libFLAC's level 0 came out 7% to 15% faster in this run than in
  that one, as its level 5 did by 3% to 9%, and this encoder's block
  within 5% of the run before's, so the column moved by about a tenth,
  most of it libFLAC's and the encoder's own part the four channels
  counted on the fourth block where the estimates do not settle the pair.
  On arm64, where both
  coders' controls came to the run before's within 2%, it reads 0.98 to
  1.22 on the stereo recordings but the 1962 speech and white noise, 0.69
  and 0.63, from 0.98 to 1.19 and the same two: the Fantasy 1.22 from
  1.19, pink noise 1.13 from 1.10, the prison song 1.05 from 1.03, the
  rest within 0.01. The two are not doing the
  same work: level 0's block of silence alone costs 12 µs where this
  encoder's costs 1.3, and what this encoder does that level 0 does not,
  the fit and the stereo choice, is most of its carried block. What was
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
  x86.** The same run on the two-core Ampere Altra, in the section after
  the tables: the recordings are 0.26 to 0.45 of libFLAC's time a block
  where x86 is 0.30 to 0.57, the made sounds 0.22 to 0.49 against 0.25 to
  0.53 and silence 0.06 to 0.08 against 0.05 to 0.07. This encoder's mean
  block is within 0.4 µs of the i5's or shorter on every recording, by up
  to 3 µs, while libFLAC's is longer on every stereo recording and 6%
  shorter on the mono reading; this encoder's 99th-percentile block is
  lower there on every recording, 8 to 56 µs against 9 to 69. Against
  level 0 it is 0.98 to 1.22 of its time on the stereo recordings but
  white noise and the 1962 speech, 0.63 and 0.69, where x86 is 0.64 to
  1.29, and 0.74 on the mono reading where x86 is 0.61: level 0's fixed
  path is cheap on the Altra. Against the run before on that machine,
  libFLAC's times came to within 2% on every made sound and recording,
  this encoder's to 0.99 to 1.04 of them on the recordings, pink noise
  1.04, the Fantasy and the 1924 record 1.02 to 1.03, the orchestra and
  the Raven 1.00, and 0.96 to 1.04 on the made sounds, the tones 0.96 to
  1.02 and the 8-bit music 1.04: what the four channels counted on the
  fourth block cost, read on the machine whose times hold still.
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
- **Under Windows, natively, this encoder's times were the WSL run's within
  7% and its bytes the same, and libFLAC's levels 5 and 0 within a
  tenth either way: WSL2 costs this encoder nothing a block.** The run of
  the afternoon of 2026-10-23 on the same i5 under Windows 11, in the last
  section, made the same day as that afternoon's WSL run from the same
  tree, the encoder of v0.0.6, and not made again for this run: this
  encoder's mean block was 0.93 to 1.04 of that WSL run's on the made
  sounds and 0.96 to 0.98 on the recordings, and its bytes the same to a
  tenth of a byte on every line. libFLAC's level 5 was 0.89 to 1.10 of
  that WSL run's, its level 0 0.93 to 1.12 and its level 8 0.85 to 1.13,
  in neither direction as a rule: the two systems time alike, and a level
  moves by a tenth from run to run under either, as the night of
  2026-10-23 had found with the encoder of v0.0.5, when levels 0 and 5
  and this encoder were run
  again under each system in turn, Windows then WSL: level 0 under Windows
  came to 0.90 to 1.08 of the WSL rerun's with a median of 1.00, level 5
  0.90 to 1.05 and this encoder 0.89 to 1.08, where that night's WSL run,
  the run before this one, had caught its level 0 at a fast moment, the
  rerun over it by up to 18% with a median of 11%, which this run's level
  0, 6% to 17% over it, bears out again. So against level 0 under Windows
  this encoder reads 0.77 to 0.98 on the 16-bit stereo recordings and 0.96
  to 1.08 on the 24-bit four with sound in them, where that WSL run read
  0.78 to 1.01 and 1.01 to 1.13, and on the made sounds 0.74 to 1.07 on
  stereo music and 0.77 to 1.18 on the tone; against level 5 the
  recordings were 0.26 to 0.49 under Windows where that WSL run had 0.28
  to 0.52, and the made sounds 0.23 to 0.53 against 0.23 to 0.52. Windows
  times more quietly: this encoder's 99th-percentile block was lower there
  on every recording, 8 to 60 µs against 9 to 70, its worst block 0.48 to
  1.17 of WSL's, and libFLAC's worst block 118 to 490 µs where WSL had 154
  to 448 and the day's earlier WSL run 149 to 745: a worst block past a
  few hundred µs is the machine's under either. libFLAC's 24-bit tone
  comes to other bytes under MSVC, 2465 against 2321 at level 5 and 2279
  against 2302 at level 8, where this encoder's are the same. The Windows
  section below sets that run beside this one's WSL columns, so its *run
  by run* tables compare two encoders: the WSL columns are this run's,
  with the pair chosen on the counts, and the Windows columns the
  afternoon's encoder, v0.0.6's; the figures in this point are of the
  afternoon's two runs, the same encoder under both systems.
## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.40}$ | $\color{green}{0.38}$ | $\color{green}{0.46}$ | $\color{green}{0.32}$ | $\color{green}{0.51}$ | $\color{green}{0.47}$ |
| speech | $\color{green}{0.30}$ | $\color{green}{0.29}$ | $\color{green}{0.32}$ | $\color{green}{0.35}$ | $\color{green}{0.30}$ | $\color{green}{0.32}$ |
| tone | $\color{green}{0.53}$ | $\color{green}{0.41}$ | $\color{green}{0.49}$ | $\color{green}{0.44}$ | $\color{green}{0.53}$ | $\color{green}{0.49}$ |
| noise | $\color{green}{0.28}$ | $\color{green}{0.27}$ | $\color{green}{0.33}$ | $\color{green}{0.25}$ | $\color{green}{0.27}$ | $\color{green}{0.30}$ |
| silence | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.07}$ | $\color{green}{0.06}$ | $\color{green}{0.05}$ | $\color{green}{0.05}$ |

### Bytes a block, this encoder over libFLAC

Under 1.00 is a smaller frame than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | $\color{green}{0.99}$ | 1.00 | $\color{green}{0.90}$ | 1.00 | $\color{green}{0.99}$ | $\color{green}{0.98}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, against libFLAC at its two ends

### Time a block, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, which fits no linear predictor and codes stereo as it comes.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.93}$ | $\color{green}{0.82}$ | $\color{red}{1.16}$ | $\color{green}{0.63}$ | $\color{red}{1.23}$ | $\color{red}{1.09}$ |
| speech | $\color{green}{0.57}$ | $\color{green}{0.52}$ | $\color{green}{0.64}$ | $\color{green}{0.65}$ | $\color{green}{0.63}$ | $\color{green}{0.61}$ |
| tone | $\color{red}{1.26}$ | $\color{green}{0.93}$ | $\color{red}{1.27}$ | $\color{green}{0.87}$ | $\color{red}{1.26}$ | $\color{red}{1.18}$ |
| noise | $\color{green}{0.62}$ | $\color{green}{0.55}$ | $\color{green}{0.75}$ | $\color{green}{0.47}$ | $\color{green}{0.62}$ | $\color{green}{0.64}$ |
| silence | $\color{green}{0.10}$ | $\color{green}{0.10}$ | $\color{green}{0.15}$ | $\color{green}{0.11}$ | $\color{green}{0.10}$ | $\color{green}{0.09}$ |

### Bytes a block, this encoder over libFLAC at level 8

Under 1.00 is a smaller frame than libFLAC's smallest level, which tries every order to 12 and every stereo pairing.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.01}$ | $\color{red}{1.01}$ | 1.00 | $\color{red}{1.01}$ | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | $\color{red}{1.01}$ | 1.00 | 1.00 | 1.00 | $\color{green}{0.99}$ | $\color{green}{0.99}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, each shape apart

libFLAC is at level 5. The time is one block's, the least of five passes;
a block is 20 ms of sound, so a share of one core is that time over 20 ms.

### 48 kHz stereo 16-bit, 960 frames a block, 3840 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 41.5 | $\color{green}{16.5}$ | $\color{green}{0.40}$ | 0.08% | 1838 | 1838 | 1.00 | 47.9% |
| speech | 34.9 | $\color{green}{10.3}$ | $\color{green}{0.30}$ | 0.05% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 40.3 | $\color{green}{21.4}$ | $\color{green}{0.53}$ | 0.11% | 1008 | $\color{green}{997}$ | $\color{green}{0.99}$ | 26.0% |
| noise | 40.8 | $\color{green}{11.6}$ | $\color{green}{0.28}$ | 0.06% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 25.6 | $\color{green}{1.3}$ | $\color{green}{0.05}$ | 0.01% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 36.9 | $\color{green}{14.2}$ | $\color{green}{0.38}$ | 0.07% | 1702 | 1702 | 1.00 | 48.2% |
| speech | 30.8 | $\color{green}{9.0}$ | $\color{green}{0.29}$ | 0.04% | 1063 | $\color{red}{1064}$ | 1.00 | 30.1% |
| tone | 35.7 | $\color{green}{14.7}$ | $\color{green}{0.41}$ | 0.07% | 920 | 920 | 1.00 | 26.1% |
| noise | 35.2 | $\color{green}{9.4}$ | $\color{green}{0.27}$ | 0.05% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 23.1 | $\color{green}{1.2}$ | $\color{green}{0.05}$ | 0.01% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 70.9 | $\color{green}{32.9}$ | $\color{green}{0.46}$ | 0.16% | 3454 | 3454 | 1.00 | 45.0% |
| speech | 60.8 | $\color{green}{19.3}$ | $\color{green}{0.32}$ | 0.10% | 2226 | $\color{red}{2228}$ | 1.00 | 29.0% |
| tone | 68.2 | $\color{green}{33.5}$ | $\color{green}{0.49}$ | 0.17% | 1651 | $\color{green}{1481}$ | $\color{green}{0.90}$ | 19.3% |
| noise | 66.9 | $\color{green}{21.8}$ | $\color{green}{0.33}$ | 0.11% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 42.8 | $\color{green}{2.9}$ | $\color{green}{0.07}$ | 0.01% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 22.6 | $\color{green}{7.3}$ | $\color{green}{0.32}$ | 0.04% | 950 | 950 | 1.00 | 49.5% |
| speech | 22.5 | $\color{green}{7.8}$ | $\color{green}{0.35}$ | 0.04% | 1148 | 1148 | 1.00 | 59.8% |
| tone | 21.4 | $\color{green}{9.4}$ | $\color{green}{0.44}$ | 0.05% | 512 | 512 | 1.00 | 26.7% |
| noise | 22.1 | $\color{green}{5.5}$ | $\color{green}{0.25}$ | 0.03% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 16.9 | $\color{green}{1.0}$ | $\color{green}{0.06}$ | 0.01% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 40.3 | $\color{green}{20.8}$ | $\color{green}{0.51}$ | 0.10% | 456 | 456 | 1.00 | 23.7% |
| speech | 35.1 | $\color{green}{10.7}$ | $\color{green}{0.30}$ | 0.05% | 250 | 250 | 1.00 | 13.0% |
| tone | 39.0 | $\color{green}{20.6}$ | $\color{green}{0.53}$ | 0.10% | 476 | $\color{green}{470}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 38.4 | $\color{green}{10.4}$ | $\color{green}{0.27}$ | 0.05% | 1002 | $\color{red}{1006}$ | 1.00 | 52.4% |
| silence | 24.6 | $\color{green}{1.3}$ | $\color{green}{0.05}$ | 0.01% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 44.9 | $\color{green}{21.1}$ | $\color{green}{0.47}$ | 0.11% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 38.0 | $\color{green}{12.2}$ | $\color{green}{0.32}$ | 0.06% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 44.0 | $\color{green}{21.6}$ | $\color{green}{0.49}$ | 0.11% | 2321 | $\color{green}{2283}$ | $\color{green}{0.98}$ | 39.6% |
| noise | 43.0 | $\color{green}{12.7}$ | $\color{green}{0.30}$ | 0.06% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 25.7 | $\color{green}{1.2}$ | $\color{green}{0.05}$ | 0.01% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{green}{0.53}$ | 1.00 | $\color{red}{1.29}$ | $\color{green}{0.95}$ | $\color{green}{0.19}$ | 1.00 |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.46}$ | 1.00 | $\color{red}{1.12}$ | $\color{green}{0.87}$ | $\color{green}{0.17}$ | $\color{red}{1.01}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.46}$ | 1.00 | $\color{red}{1.03}$ | $\color{green}{0.78}$ | $\color{green}{0.16}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.50}$ | 1.00 | $\color{red}{1.14}$ | $\color{green}{0.96}$ | $\color{green}{0.17}$ | 1.00 |
| Mix at −13 LUFS, wide stereo | 44.1 kHz stereo 16-bit | $\color{green}{0.44}$ | 1.00 | $\color{green}{0.90}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Mix at −9 LUFS, loud master | 44.1 kHz stereo 16-bit | $\color{green}{0.43}$ | 1.00 | $\color{green}{0.88}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | $\color{green}{0.54}$ | 1.00 | $\color{red}{1.25}$ | $\color{green}{0.96}$ | $\color{green}{0.20}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.43}$ | 1.00 | $\color{green}{0.90}$ | $\color{green}{0.88}$ | $\color{green}{0.19}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.43}$ | 1.00 | $\color{green}{0.90}$ | $\color{green}{0.78}$ | $\color{green}{0.19}$ | 1.00 |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.34}$ | 1.00 | $\color{green}{0.61}$ | $\color{green}{0.97}$ | $\color{green}{0.24}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.36}$ | 1.00 | $\color{green}{0.70}$ | $\color{green}{0.49}$ | $\color{green}{0.14}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.41}$ | 1.00 | $\color{green}{0.87}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.57}$ | 1.00 | $\color{red}{1.22}$ | $\color{green}{0.99}$ | $\color{green}{0.20}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.30}$ | 1.00 | $\color{green}{0.64}$ | $\color{green}{0.99}$ | $\color{green}{0.11}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms. *The run before* is this
encoder's mean block in the run of the commit before this one, and *over it*
this run's over that.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | The run before, mean µs | Over it | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 76.5 | $\color{green}{40.6}$ | $\color{green}{0.53}$ | 39.5 | $\color{red}{1.03}$ | 128 | $\color{green}{68.6}$ | 499 | $\color{green}{176}$ | 0.20% |
| Chopin, Fantasy Op. 49, piano | 70.1 | $\color{green}{32.2}$ | $\color{green}{0.46}$ | 31.8 | $\color{red}{1.01}$ | 117 | $\color{green}{54.3}$ | 2791 | $\color{green}{218}$ | 0.16% |
| Shostakovich 5, orchestra, live | 41.2 | $\color{green}{18.8}$ | $\color{green}{0.46}$ | 18.9 | 1.00 | 73.6 | $\color{green}{36.3}$ | 227 | $\color{green}{150}$ | 0.09% |
| Indie rock, 2022, mastered | 46.8 | $\color{green}{23.5}$ | $\color{green}{0.50}$ | 22.8 | $\color{red}{1.03}$ | 78.7 | $\color{green}{44.5}$ | 218 | $\color{green}{158}$ | 0.12% |
| Mix at −13 LUFS, wide stereo | 36.3 | $\color{green}{15.9}$ | $\color{green}{0.44}$ | 15.3 | $\color{red}{1.04}$ | 64.5 | $\color{green}{32.4}$ | 164 | $\color{green}{93.0}$ | 0.08% |
| Mix at −9 LUFS, loud master | 36.1 | $\color{green}{15.5}$ | $\color{green}{0.43}$ | 15.2 | $\color{red}{1.02}$ | 62.7 | $\color{green}{31.3}$ | 126 | $\color{green}{96.8}$ | 0.08% |
| Limehouse Blues, 1924 78 rpm | 81.0 | $\color{green}{43.5}$ | $\color{green}{0.54}$ | 42.7 | $\color{red}{1.02}$ | 129 | $\color{green}{71.2}$ | 489 | $\color{green}{190}$ | 0.22% |
| Prison work song, 1947 field recording | 35.8 | $\color{green}{15.2}$ | $\color{green}{0.43}$ | 14.9 | $\color{red}{1.02}$ | 62.0 | $\color{green}{31.2}$ | 162 | $\color{green}{88.3}$ | 0.08% |
| The Raven, read, produced | 36.3 | $\color{green}{15.8}$ | $\color{green}{0.43}$ | 15.5 | $\color{red}{1.02}$ | 63.4 | $\color{green}{31.7}$ | 220 | $\color{red}{228}$ | 0.08% |
| Leviticus 13, read | 20.7 | $\color{green}{6.9}$ | $\color{green}{0.34}$ | 7.0 | $\color{green}{0.99}$ | 38.2 | $\color{green}{8.7}$ | 178 | $\color{green}{89.9}$ | 0.03% |
| JFK at Rice, 1962 | 38.2 | $\color{green}{13.9}$ | $\color{green}{0.36}$ | 13.6 | $\color{red}{1.02}$ | 68.2 | $\color{green}{29.7}$ | 235 | $\color{green}{148}$ | 0.07% |
| Train station, field recording | 35.8 | $\color{green}{14.7}$ | $\color{green}{0.41}$ | 14.6 | $\color{red}{1.01}$ | 57.8 | $\color{green}{29.5}$ | 184 | $\color{green}{85.8}$ | 0.07% |
| Pink noise | 44.3 | $\color{green}{25.1}$ | $\color{green}{0.57}$ | 23.9 | $\color{red}{1.05}$ | 76.9 | $\color{green}{46.3}$ | 333 | $\color{green}{189}$ | 0.13% |
| White noise | 43.8 | $\color{green}{13.2}$ | $\color{green}{0.30}$ | 12.9 | $\color{red}{1.02}$ | 76.0 | $\color{green}{28.5}$ | 260 | $\color{green}{104}$ | 0.07% |

### The bytes, libFLAC at level 5

*The run before* is this encoder's bytes a block in the run of the commit
before this one, and *over it* this run's over that, to a ten-thousandth.

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | The run before, bytes | Over it | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5313}$ | 1.000 | 5318 | $\color{green}{0.9989}$ | 46.1% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2478}$ | $\color{red}{1.001}$ | 2482 | $\color{green}{0.9983}$ | 32.3% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1499}$ | 1.000 | 1499 | 1.0000 | 39.0% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{red}{4556}$ | 1.000 | 4556 | 1.0000 | 79.1% |
| Mix at −13 LUFS, wide stereo | 250 | 3528 | 2325 | $\color{red}{2327}$ | $\color{red}{1.001}$ | 2327 | 1.0000 | 66.0% |
| Mix at −9 LUFS, loud master | 225 | 3528 | 2543 | $\color{red}{2547}$ | $\color{red}{1.001}$ | 2547 | 1.0000 | 72.2% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | $\color{green}{8657}$ | 1.000 | 8657 | 1.0000 | 75.1% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | $\color{red}{1421}$ | $\color{red}{1.001}$ | 1421 | $\color{green}{0.9999}$ | 40.3% |
| The Raven, read, produced | 559 | 3528 | 1362 | $\color{red}{1363}$ | 1.000 | 1363 | $\color{green}{0.9999}$ | 38.6% |
| Leviticus 13, read | 725 | 1764 | 986 | $\color{red}{987}$ | 1.000 | 987 | 1.0000 | 55.9% |
| JFK at Rice, 1962 | 836 | 5760 | 1798 | $\color{green}{1797}$ | 1.000 | 1797 | 1.0000 | 31.2% |
| Train station, field recording | 141 | 3528 | 1272 | 1272 | 1.000 | 1273 | $\color{green}{0.9996}$ | 36.1% |
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
| music | $\color{green}{0.35}$ | $\color{green}{0.34}$ | $\color{green}{0.37}$ | $\color{green}{0.35}$ | $\color{green}{0.47}$ | $\color{green}{0.39}$ |
| speech | $\color{green}{0.26}$ | $\color{green}{0.25}$ | $\color{green}{0.26}$ | $\color{green}{0.36}$ | $\color{green}{0.28}$ | $\color{green}{0.26}$ |
| tone | $\color{green}{0.46}$ | $\color{green}{0.37}$ | $\color{green}{0.43}$ | $\color{green}{0.45}$ | $\color{green}{0.49}$ | $\color{green}{0.39}$ |
| noise | $\color{green}{0.24}$ | $\color{green}{0.22}$ | $\color{green}{0.26}$ | $\color{green}{0.25}$ | $\color{green}{0.22}$ | $\color{green}{0.24}$ |
| silence | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.08}$ | $\color{green}{0.07}$ | $\color{green}{0.06}$ |

### Time a block on arm64, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on arm64.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.08}$ | $\color{green}{0.97}$ | $\color{red}{1.23}$ | $\color{green}{0.79}$ | $\color{red}{1.45}$ | $\color{red}{1.07}$ |
| speech | $\color{green}{0.64}$ | $\color{green}{0.59}$ | $\color{green}{0.68}$ | $\color{green}{0.81}$ | $\color{green}{0.74}$ | $\color{green}{0.59}$ |
| tone | $\color{red}{1.43}$ | $\color{red}{1.07}$ | $\color{red}{1.39}$ | $\color{red}{1.06}$ | $\color{red}{1.52}$ | $\color{red}{1.09}$ |
| noise | $\color{green}{0.65}$ | $\color{green}{0.56}$ | $\color{green}{0.71}$ | $\color{green}{0.53}$ | $\color{green}{0.66}$ | $\color{green}{0.62}$ |
| silence | $\color{green}{0.15}$ | $\color{green}{0.15}$ | $\color{green}{0.17}$ | $\color{green}{0.17}$ | $\color{green}{0.15}$ | $\color{green}{0.13}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 |
|---|---:|---:|---:|---:|---:|---:|
| music | 41.5 | 47.0 | 16.5 | 16.6 | $\color{green}{0.40}$ | $\color{green}{0.35}$ |
| speech | 34.9 | 39.0 | 10.3 | 10.0 | $\color{green}{0.30}$ | $\color{green}{0.26}$ |
| tone | 40.3 | 45.0 | 21.4 | 20.8 | $\color{green}{0.53}$ | $\color{green}{0.46}$ |
| noise | 40.8 | 43.2 | 11.6 | 10.2 | $\color{green}{0.28}$ | $\color{green}{0.24}$ |
| silence | 25.6 | 23.6 | 1.3 | 1.6 | $\color{green}{0.05}$ | $\color{green}{0.07}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 | 99th, x86 | 99th, arm64 | Time vs 0, x86 | Time vs 0, arm64 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 76.5 | 92.8 | 40.6 | 37.5 | $\color{green}{0.53}$ | $\color{green}{0.40}$ | 68.6 | 53.2 | $\color{red}{1.29}$ | $\color{red}{1.15}$ |
| Chopin, Fantasy Op. 49, piano | 70.1 | 85.6 | 32.2 | 32.6 | $\color{green}{0.46}$ | $\color{green}{0.38}$ | 54.3 | 48.7 | $\color{red}{1.12}$ | $\color{red}{1.22}$ |
| Shostakovich 5, orchestra, live | 41.2 | 46.0 | 18.8 | 18.9 | $\color{green}{0.46}$ | $\color{green}{0.41}$ | 36.3 | 27.2 | $\color{red}{1.03}$ | $\color{red}{1.19}$ |
| Indie rock, 2022, mastered | 46.8 | 53.0 | 23.5 | 21.9 | $\color{green}{0.50}$ | $\color{green}{0.41}$ | 44.5 | 34.2 | $\color{red}{1.14}$ | $\color{red}{1.09}$ |
| Mix at −13 LUFS, wide stereo | 36.3 | 42.0 | 15.9 | 15.5 | $\color{green}{0.44}$ | $\color{green}{0.37}$ | 32.4 | 23.8 | $\color{green}{0.90}$ | $\color{red}{1.01}$ |
| Mix at −9 LUFS, loud master | 36.1 | 41.8 | 15.5 | 15.4 | $\color{green}{0.43}$ | $\color{green}{0.37}$ | 31.3 | 21.4 | $\color{green}{0.88}$ | $\color{green}{0.98}$ |
| Limehouse Blues, 1924 78 rpm | 81.0 | 96.4 | 43.5 | 40.2 | $\color{green}{0.54}$ | $\color{green}{0.42}$ | 71.2 | 55.5 | $\color{red}{1.25}$ | $\color{red}{1.11}$ |
| Prison work song, 1947 field recording | 35.8 | 40.9 | 15.2 | 15.2 | $\color{green}{0.43}$ | $\color{green}{0.37}$ | 31.2 | 20.8 | $\color{green}{0.90}$ | $\color{red}{1.05}$ |
| The Raven, read, produced | 36.3 | 41.2 | 15.8 | 15.6 | $\color{green}{0.43}$ | $\color{green}{0.38}$ | 31.7 | 22.6 | $\color{green}{0.90}$ | $\color{red}{1.05}$ |
| Leviticus 13, read | 20.7 | 19.4 | 6.9 | 6.8 | $\color{green}{0.34}$ | $\color{green}{0.35}$ | 8.7 | 8.2 | $\color{green}{0.61}$ | $\color{green}{0.74}$ |
| JFK at Rice, 1962 | 38.2 | 43.1 | 13.9 | 13.2 | $\color{green}{0.36}$ | $\color{green}{0.31}$ | 29.7 | 21.9 | $\color{green}{0.70}$ | $\color{green}{0.69}$ |
| Train station, field recording | 35.8 | 40.6 | 14.7 | 14.5 | $\color{green}{0.41}$ | $\color{green}{0.36}$ | 29.5 | 19.7 | $\color{green}{0.87}$ | $\color{red}{1.01}$ |
| Pink noise | 44.3 | 49.6 | 25.1 | 22.5 | $\color{green}{0.57}$ | $\color{green}{0.45}$ | 46.3 | 29.2 | $\color{red}{1.22}$ | $\color{red}{1.13}$ |
| White noise | 43.8 | 49.0 | 13.2 | 12.6 | $\color{green}{0.30}$ | $\color{green}{0.26}$ | 28.5 | 25.3 | $\color{green}{0.64}$ | $\color{green}{0.63}$ |

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
| music | $\color{green}{0.33}$ | $\color{green}{0.39}$ | $\color{green}{0.44}$ | $\color{green}{0.31}$ | $\color{green}{0.48}$ | $\color{green}{0.44}$ |
| speech | $\color{green}{0.28}$ | $\color{green}{0.28}$ | $\color{green}{0.34}$ | $\color{green}{0.32}$ | $\color{green}{0.31}$ | $\color{green}{0.30}$ |
| tone | $\color{green}{0.49}$ | $\color{green}{0.43}$ | $\color{green}{0.50}$ | $\color{green}{0.42}$ | $\color{green}{0.53}$ | $\color{green}{0.48}$ |
| noise | $\color{green}{0.27}$ | $\color{green}{0.26}$ | $\color{green}{0.31}$ | $\color{green}{0.23}$ | $\color{green}{0.25}$ | $\color{green}{0.27}$ |
| silence | $\color{green}{0.06}$ | $\color{green}{0.06}$ | $\color{green}{0.07}$ | $\color{green}{0.06}$ | $\color{green}{0.06}$ | $\color{green}{0.05}$ |

### Time a block on Windows, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on Windows.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.74}$ | $\color{green}{0.74}$ | $\color{red}{1.01}$ | $\color{green}{0.53}$ | $\color{red}{1.07}$ | $\color{green}{0.97}$ |
| speech | $\color{green}{0.50}$ | $\color{green}{0.46}$ | $\color{green}{0.59}$ | $\color{green}{0.56}$ | $\color{green}{0.61}$ | $\color{green}{0.51}$ |
| tone | $\color{red}{1.18}$ | $\color{green}{0.86}$ | $\color{red}{1.12}$ | $\color{green}{0.77}$ | $\color{red}{1.15}$ | $\color{red}{1.09}$ |
| noise | $\color{green}{0.56}$ | $\color{green}{0.47}$ | $\color{green}{0.64}$ | $\color{green}{0.38}$ | $\color{green}{0.55}$ | $\color{green}{0.55}$ |
| silence | $\color{green}{0.11}$ | $\color{green}{0.10}$ | $\color{green}{0.16}$ | $\color{green}{0.10}$ | $\color{green}{0.11}$ | $\color{green}{0.10}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, WSL µs | libFLAC, Windows µs | This encoder, WSL µs | This encoder, Windows µs | Ratio, WSL | Ratio, Windows |
|---|---:|---:|---:|---:|---:|---:|
| music | 41.5 | 48.3 | 16.5 | 16.1 | $\color{green}{0.40}$ | $\color{green}{0.33}$ |
| speech | 34.9 | 35.5 | 10.3 | 9.9 | $\color{green}{0.30}$ | $\color{green}{0.28}$ |
| tone | 40.3 | 41.9 | 21.4 | 20.6 | $\color{green}{0.53}$ | $\color{green}{0.49}$ |
| noise | 40.8 | 41.4 | 11.6 | 11.0 | $\color{green}{0.28}$ | $\color{green}{0.27}$ |
| silence | 25.6 | 22.9 | 1.3 | 1.3 | $\color{green}{0.05}$ | $\color{green}{0.06}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, WSL µs | libFLAC, Windows µs | This encoder, WSL µs | This encoder, Windows µs | Ratio, WSL | Ratio, Windows | 99th, WSL | 99th, Windows | Time vs 0, WSL | Time vs 0, Windows |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 76.5 | 78.8 | 40.6 | 38.2 | $\color{green}{0.53}$ | $\color{green}{0.48}$ | 68.6 | 59.3 | $\color{red}{1.29}$ | $\color{red}{1.08}$ |
| Chopin, Fantasy Op. 49, piano | 70.1 | 69.2 | 32.2 | 30.6 | $\color{green}{0.46}$ | $\color{green}{0.44}$ | 54.3 | 48.6 | $\color{red}{1.12}$ | $\color{green}{0.98}$ |
| Shostakovich 5, orchestra, live | 41.2 | 42.2 | 18.8 | 18.2 | $\color{green}{0.46}$ | $\color{green}{0.43}$ | 36.3 | 29.9 | $\color{red}{1.03}$ | $\color{green}{0.93}$ |
| Indie rock, 2022, mastered | 46.8 | 48.6 | 23.5 | 22.1 | $\color{green}{0.50}$ | $\color{green}{0.46}$ | 44.5 | 36.8 | $\color{red}{1.14}$ | $\color{green}{0.96}$ |
| Mix at −13 LUFS, wide stereo | 36.3 | 35.9 | 15.9 | 15.0 | $\color{green}{0.44}$ | $\color{green}{0.42}$ | 32.4 | 22.6 | $\color{green}{0.90}$ | $\color{green}{0.77}$ |
| Mix at −9 LUFS, loud master | 36.1 | 35.4 | 15.5 | 14.8 | $\color{green}{0.43}$ | $\color{green}{0.42}$ | 31.3 | 22.5 | $\color{green}{0.88}$ | $\color{green}{0.77}$ |
| Limehouse Blues, 1924 78 rpm | 81.0 | 83.6 | 43.5 | 41.1 | $\color{green}{0.54}$ | $\color{green}{0.49}$ | 71.2 | 59.8 | $\color{red}{1.25}$ | $\color{red}{1.04}$ |
| Prison work song, 1947 field recording | 35.8 | 35.0 | 15.2 | 14.4 | $\color{green}{0.43}$ | $\color{green}{0.41}$ | 31.2 | 22.5 | $\color{green}{0.90}$ | $\color{green}{0.78}$ |
| The Raven, read, produced | 36.3 | 35.2 | 15.8 | 15.2 | $\color{green}{0.43}$ | $\color{green}{0.43}$ | 31.7 | 24.9 | $\color{green}{0.90}$ | $\color{green}{0.79}$ |
| Leviticus 13, read | 20.7 | 21.1 | 6.9 | 6.7 | $\color{green}{0.34}$ | $\color{green}{0.32}$ | 8.7 | 8.3 | $\color{green}{0.61}$ | $\color{green}{0.53}$ |
| JFK at Rice, 1962 | 38.2 | 39.9 | 13.9 | 13.2 | $\color{green}{0.36}$ | $\color{green}{0.33}$ | 29.7 | 27.4 | $\color{green}{0.70}$ | $\color{green}{0.59}$ |
| Train station, field recording | 35.8 | 34.7 | 14.7 | 14.1 | $\color{green}{0.41}$ | $\color{green}{0.41}$ | 29.5 | 19.9 | $\color{green}{0.87}$ | $\color{green}{0.77}$ |
| Pink noise | 44.3 | 47.2 | 25.1 | 23.3 | $\color{green}{0.57}$ | $\color{green}{0.49}$ | 46.3 | 38.4 | $\color{red}{1.22}$ | $\color{red}{1.01}$ |
| White noise | 43.8 | 47.0 | 13.2 | 12.3 | $\color{green}{0.30}$ | $\color{green}{0.26}$ | 28.5 | 23.7 | $\color{green}{0.64}$ | $\color{green}{0.53}$ |

The bytes a block of this encoder differ between the two runs on tone at 96 kHz stereo 16-bit (1481 against 1496), Bach, Goldberg Aria, piano (5313 against 5318), Chopin, Fantasy Op. 49, piano (2478 against 2482), Mix at −9 LUFS, loud master (2547 against 2547), Limehouse Blues, 1924 78 rpm (8657 against 8657), Prison work song, 1947 field recording (1421 against 1421), The Raven, read, produced (1363 against 1363), Train station, field recording (1272 against 1273).

The bytes a block of libFLAC at level 5 differ between the two runs on tone at 48 kHz stereo 24-bit (2321 against 2465).

The bytes a block of libFLAC at level 0 are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 8 differ between the two runs on tone at 48 kHz stereo 24-bit (2302 against 2279).

