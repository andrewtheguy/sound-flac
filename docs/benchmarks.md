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
`tools/tables.py` makes the tables below from the log, a JSON object a
line, and the logs the tables are of are kept in `docs/bench/`, so that the
next run's recordings tables can carry this run beside them as *the run
before*:

```sh
tools/samples.sh copy      # the recordings into tmp/samples, from the store
cargo run --release --manifest-path tools/bench/Cargo.toml > docs/bench/x86.jsonl
python3 -I tools/tables.py docs/bench/x86.jsonl
python3 -I tools/tables.py docs/bench/x86.jsonl arm64=docs/bench/arm64.jsonl   # with an arm64 run's log, its section too
python3 -I tools/tables.py docs/bench/x86.jsonl arm64=docs/bench/arm64.jsonl windows=docs/bench/windows.jsonl   # and the Windows run's, made on demand
git show HEAD:docs/bench/x86.jsonl > before.jsonl       # the run before, from the commit before; a commit from before the JSON lines holds x86.log, whose key=value lines are made JSON objects by hand first
python3 -I tools/tables.py docs/bench/x86.jsonl arm64=docs/bench/arm64.jsonl windows=docs/bench/windows.jsonl --before before.jsonl
```

In the recordings tables below, *the run before* is the run of the commit
before this one, made the evening of 2026-10-23 with this same encoder,
whose log that commit carries as `docs/bench/x86.log`, a log of
`key=value` fields a line as the logs were until they became JSON lines,
`.jsonl`, which `tools/tables.py` reads alone; this run was made so that
the kept logs are JSON lines, the encoder unchanged. The days
named in this document and in [plan.md](plan.md) before the evening's run,
the night of 2026-10-23 and 2026-10-24 among them, are the count the
sessions that made the earlier runs kept, which ran ahead of the machine's
clock: by the commits, which that clock stamps in Pacific time, the run of
v0.0.5 was made at noon on 2026-10-23, the run of v0.0.6 that afternoon,
the run before this one that evening, and this run later the same evening,
which is the clock's own day and is named by it from here on; the whole of
the encoder's work from the plan to the run before ran from the evening of
2026-10-22 to that evening.

They were run on 2026-10-23 by the machine's clock, late in the evening,
on a six-core Intel i5-8500T under WSL2, one thread, WSL restarted for
the run and the machine idle but for the terminal session it was started
from, with rustc 1.98.1 at the x86-64 baseline (SSE2, no `target-cpu`),
at the commit that carries them: the encoder of v0.0.6 with the evening's
change of [plan.md](plan.md), the stereo pair chosen on the four
channels' bits counted in one Rice partition where the estimates do not
settle it, the same encoder as the run before's, this run being the
remake that made the kept logs JSON lines. A time on this machine moves
by up to a tenth from one run to the next, so a time ratio within a tenth
of 1.00 is on the line; a bytes ratio is exact, and every coder's bytes
in this run are the run before's on every line. libFLAC's own times are
the control: in this run its level 5 is 5% to 10% over the run before's
on the recordings and within 3% under to 12% over on the made sounds,
its level 0 8% to 17% over on the recordings and within 4% under to 17%
over on the made sounds, and its level 8 within 8%, so this run caught
levels 5 and 0 at a slow moment where the run before had caught them at
a fast one, and this encoder's ratios to them read better by that much
for nothing it did: its own block is 0.96 to 1.03 of the run before's on
the recordings, the wide mix the 0.96, the orchestra 0.98, the 1924
record 1.03, and 0.90 to 1.03 of it on the made sounds, the mono silence
the 0.90. The run is one whole run, nothing in it measured again, and
its bytes are the arm64 run's to a tenth of a byte on every line. A worst block
is one block of thousands and the machine's as much as the encoder's, even
as the least over passes, so it is read as an order of magnitude. The bench
also logs `worst_at`, the block that was slowest at its least over the
passes, and dumps every block's least time and bytes when asked
(`BLOCKS=dir`), which is how the late blocks below were read; every pass
starts from an encoder that has seen no block, so that what the encoder
carries from block to block, the fourth-block choice, falls on the same
blocks in every pass and the least over the passes is the block's own.

The arm64 run of the section after the recordings was made the same
evening from the same commit on a two-core Ampere Altra (Neoverse N1) in
a Debian 13 LXC container, one thread, the machine idle, with rustc
1.98.1 at the aarch64 baseline, whose NEON the compiler uses on its own:
the tree is copied there, the recordings are kept there from the
morning's run, the bench run, and its log brought back as the second log
of `tools/tables.py`. The morning's first run there was two to three
times slow on the made sounds while the file system was still compressing
the recordings just copied, and was run again once the machine was idle;
the recordings came to the same bytes and the same times within 0.2 µs in
both runs, and the made sounds run twice more agreed to 0.1 µs, so a time
on that machine is repeatable where this one's moves by a tenth. In this
run libFLAC's three levels are the run before's within 2% on every line
and this encoder's recordings within 3%, the mono reading the 3%; its
made sounds are within 1% but the mono shape, where all five are 0.2 µs
over the run before's, silence 1.36 µs against 1.13, which the shape run
twice more came to again while libFLAC's mono times held: the encoder
being the same code, that is the bench binary, laid out otherwise with
its JSON writer, and it is left as measured. The level 0 cells of that
machine, this encoder over libFLAC's fastest level at 1.08 on 48 kHz
music and 1.43 on its tone, 1.45 and 1.51 on 8-bit, 1.23 and 1.39 at 96
kHz, were checked afterwards with the bench timing the four coders
together, every block by each coder in turn (`INTERLEAVE=1`), so that
both sides of a ratio see the machine in one state: two such runs, with
two ordinary runs between them, put every made-sound ratio of the arm64
tables within 0.03 of the run's, this encoder's and level 0's times
within 2%, and the mono shape's 0.2 µs came off again with the bench
rebuilt. The cells are the machines', not the run's: this encoder's block
is the same length on the two, 16.6 µs against 16.8 on 48 kHz music, and
level 0's is 15.3 µs on the Altra against 19.9 on the i5. On the i5 the
interleaving puts about 1 µs on every coder's block, the four sharing the
cache, and leaves the ratios within 0.03 but silence, whose 1.3 µs block
grows by half.

The Windows run of the section after it was made the same evening, after
the x86 run and while the arm64 run was ending on its own machine, on
this same i5-8500T under Windows 11 Pro, natively, with rustc 1.93 MSVC
at the x86-64 baseline and libFLAC from its sys crate's Windows archive,
level 5 through v0.0.4 of this crate and levels 0 and 8 through the
bench's own link of the same crate at the same tag, as on the two systems
above, from the tree of this commit copied over the staging clone of
`main` there: the recordings copied from the store to the clone for the
run and deleted after it, the machine idle but for the WSL session the
run was started from. Its made sounds, which the bench runs first, came
out 1.1 to 3.7 times the WSL run's for all four coders, the system still
busy over the 1.5 GB of recordings just copied, where the recordings that
followed came to within 5% of WSL's for this encoder; so once the run was
done the made sounds were run again, the 44.1 kHz shape twice and then
all of them, the three agreeing within a tenth, and the log carries the
whole second run's 120 lines in place of the first's, with the run's own
56 lines of recordings. Unlike the x86 and arm64 runs, it is made on
demand, not with every change to the tuning, so its section can be of an
earlier encoder than the tables above; this paragraph says which, and
this time it is this run's encoder, the x86 and arm64 runs'. Earlier
that day, with the encoder of v0.0.5, levels 0 and 5 and this encoder
were run again under both systems in turn, Windows first and WSL after
it, to read that run's level 0 against its WSL run; those logs are not
kept, and what they showed is in the last point of the next section.

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
  half.** The recordings are 0.28 to 0.54 of its time a block: 16-bit
  stereo 0.39 to 0.43, mono 0.30, 24-bit 0.28 to 0.54, the two at 96 kHz
  and 24 bits 0.51 and 0.52. A block of a recording is 0.03% to 0.22% of
  one core. The made sounds are 0.23 to 0.50: music 0.30 to 0.49, speech
  0.27 to 0.31, the tone 0.40 to 0.50, noise 0.23 to 0.31, silence 0.05 to
  0.07. The run before, the evening of 2026-10-23 and this same encoder,
  had them at 0.30 to 0.57 and 0.25 to 0.53, the run of v0.0.6 before it,
  that afternoon, at 0.28 to 0.52 and 0.23 to 0.52, the run of v0.0.5
  before it at 0.34 to 0.58 and 0.27 to 0.55, the evening's before that
  0.33 to 0.55 and 0.27 to 0.53, the afternoon's 0.31 to 0.59 and 0.25 to
  0.49, the morning's 0.44 to 0.77 and 0.40 to 0.74, for the same bytes
  within half a per cent but the tone's, which fell, and in the run
  before the pianos', which fell by a tenth of a per cent. Against the
  run before this encoder's own block is 0.96 to 1.03 of its on the
  recordings, the wide mix 0.96, the orchestra 0.98, the 1924 record
  1.03, and 0.90 to 1.03 on the made sounds, the mono silence 0.90, the
  44.1 kHz noise 1.03; libFLAC's own, run beside it, is 5% to 10% over
  its run before's on the recordings, so the ratios to it read better by
  more than the encoder moved, the encoder being the same. On arm64,
  where libFLAC's held within 2%, this encoder's is 0.99 to 1.03 of the
  run before's on the recordings, the mono reading the 1.03, and within
  1% on the made sounds but the mono shape, 0.2 µs over on all five of
  them, the bench binary's layout and not the encoder, which the run
  paragraph above says.
- **The bytes are libFLAC's at its default level, within 0.2%, and on a
  pure tone under them.** Every recording is 1.000 but the Fantasy, the
  prison song and the two private mixes at 1.001, a byte to four a block,
  which is what fitting only the carried pair between full choices and
  coding it the fit first cost them (the measurements are in
  [plan.md](plan.md)); before the pair was chosen on the fit the train
  station was 1.006 and the Fantasy 1.004, and before it was chosen on the
  four channels' counts the Fantasy was 1.002 and the Aria 1.001. Against
  the run before every recording and every made sound comes to the same
  bytes, the encoder being the same; the run before, against the
  afternoon's, had brought every recording to the same bytes or fewer, the
  Fantasy 0.9983, the Aria 0.9989, the train station 0.9996, the Raven and
  the prison song 0.9999, which is the pair chosen on the counts where the
  estimates could not tell left and right from mid and side, and every
  made sound to the same bytes but the pure tone at 96 kHz, 0.990 of the
  afternoon's. The tone is 0.99 of libFLAC's at 48 kHz, 1.00 at
  44.1 kHz where it was 1.02, 0.90 at 96 kHz where it was 0.95, 0.98 at 24
  bits where it was 1.05 and 0.99 at 8 bits: a tone is predicted away by
  order 2 and what its frame then costs is the rounding of the
  coefficients, which the order choice now weighs exactly, from the
  autocorrelation, where before it was blind to it and the orders tied.
  The made speech, whose two channels are one signal, is 0.05% over
  libFLAC and 0.16% at 8 bits: its side channel is a constant, and a block
  with a constant channel is coded the fit first like any other where it
  had every channel coded both ways. Against libFLAC's smallest level, 8,
  which tries every order to 12 and every stereo pairing in five to ten
  times this encoder's time, the made sounds are 1.00 to 1.01, the tone at
  96 kHz and 24 bits 1.01 and 0.99 where they were 1.07 and 1.06, and the
  recordings 1.00 to 1.01.
- **The late block is still the fourth block, where the pair is chosen
  afresh, by a fifth to a third of a carried block.** Each block's least
  time over the passes, each pass from an encoder that has seen no block,
  read by the block's place in the cycle of four, in a run of this encoder
  alone after the run before's tables, the same encoder: the fourth block is 21.9
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
  99th-percentile block is 0.21 to 0.58 of libFLAC's, 9 to 74 µs against
  42 to 138, and its worst block 86 to 227 µs, at most 1.1% of the 20 ms
  the block is, where libFLAC's worst is 154 to 2647. Where two channels
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
  stereo as it comes**, this encoder takes 0.79 to 1.00 of its time on the
  16-bit stereo recordings, the Fantasy at 96 kHz 1.00 and the orchestra
  0.92, 0.64 on the 1962 speech and 0.58 on white noise, 0.52 on the mono
  reading, and 1.05 to 1.17 on the four at 24 bits with sound in them, for
  0.49 to 0.99 of its bytes: the two stereo speeches 0.49 and 0.78, the
  music 0.78 to 0.96, noise 0.99. On the made sounds it is 0.75 to 1.11 on
  stereo music and 0.84 to 1.16 on the tone, 0.48 to 0.58 on speech, 0.51
  to 0.68 on noise, 0.41 to 0.75 on mono and 0.10 to 0.16 on silence. The
  run before had the 16-bit stereo recordings at 0.87 to 1.12, the 1962
  speech 0.70, white noise 0.64, the mono reading 0.61 and the 24-bit four
  at 1.14 to 1.29, the made stereo music and tone at 0.82 to 1.23 and 0.93
  to 1.27: libFLAC's level 0 came out 8% to 17% slower in this run than in
  that one, as its level 5 did by 5% to 10%, and this encoder's block
  within 4% of the run before's, so the column moved by about a tenth, all
  of it libFLAC's, the encoder being the same; the run of v0.0.6 before
  them both had read 0.78 to 1.01 on the 16-bit stereo recordings, a fast
  moment of level 0's, as the run before's 0.87 to 1.12 was of this
  encoder's four channels counted on the fourth block. On arm64, where
  both coders' controls came to the run before's within 2%, it reads 0.98
  to 1.22 on the stereo recordings but the 1962 speech and white noise,
  0.69 and 0.63, as the run before did, every recording within 0.01 of
  it. The two are not doing the
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
  the tables: the recordings are 0.25 to 0.45 of libFLAC's time a block
  where x86 is 0.28 to 0.54, the made sounds 0.22 to 0.49 against 0.23 to
  0.50 and silence 0.06 to 0.10 against 0.05 to 0.07. This encoder's mean
  block is within 0.3 µs of the i5's or shorter on every recording, by up
  to 4.5 µs, while libFLAC's is longer on every stereo recording and 15%
  shorter on the mono reading; this encoder's 99th-percentile block is
  lower there on every recording, 8 to 56 µs against 9 to 74. Against
  level 0 it is 0.98 to 1.22 of its time on the stereo recordings but
  white noise and the 1962 speech, 0.63 and 0.69, where x86 is 0.58 to
  1.17, and 0.76 on the mono reading where x86 is 0.52: level 0's fixed
  path is cheap on the Altra. Against the run before on that machine,
  libFLAC's times came to within 2% on every made sound and recording,
  this encoder's to 0.99 to 1.03 of them on the recordings, the mono
  reading the 1.03 and every other within 1%, and within 1% on the made
  sounds but the mono shape's five, 0.2 µs over, the bench binary's
  layout: the same encoder, read on the machine whose times hold still.
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
  5% on the recordings and 6% on the made sounds but one silence, and its
  bytes the same; libFLAC's levels 5 and 0 within a tenth either way but
  five cells, and its level 8 slower under Windows, by a tenth to a
  quarter on half of its 44 cells: WSL2 costs this encoder nothing a
  block.** The run of the same
  evening on the same i5 under Windows 11, in the last section, from the
  tree of this commit, the same encoder as this run's WSL columns: this
  encoder's mean block is 0.95 to 1.05 of the WSL run's on the recordings,
  the wide mix the 1.05 and the 1924 record 0.95, and 0.94 to 1.05 on the
  made sounds but the mono silence, 1.13 of a block under a microsecond,
  and its bytes the same to a tenth of a byte on every line. libFLAC's
  level 5 is 0.92 to 1.12 of the WSL run's on the recordings, the 1924
  record the 1.12, and 0.88 to 1.01 on the made sounds, its level 0 0.98
  to 1.18 and 0.95 to 1.04, the two in neither direction as a rule, with
  a median of 0.95 and 1.00; its level 8 is 0.92 to 1.25 and 0.89 to 1.18
  with a median of 1.07, over 1.00 on 30 of its 44 cells and past 1.10 on
  21, so in this run level 8 is the slower under Windows, as a rule, where
  this encoder and the two other levels are not. The two systems time this
  encoder alike, and a libFLAC level moves by a tenth from run to run
  under either, as the day's earlier
  runs had found with the encoder of v0.0.5, when levels 0 and 5 and this
  encoder were run again under each system in turn, Windows then WSL:
  level 0 under Windows came to 0.90 to 1.08 of the WSL rerun's with a
  median of 1.00, level 5 0.90 to 1.05 and this encoder 0.89 to 1.08. So
  against level 0 under Windows this encoder reads 0.78 to 1.00 on the
  16-bit stereo recordings and 0.92 to 1.17 on the 24-bit four with sound
  in them, where WSL reads 0.79 to 1.00 and 1.05 to 1.17, and on the made
  sounds 0.73 to 1.13 on stereo music and 0.81 to 1.18 on the tone;
  against level 5 the recordings are 0.28 to 0.54 under Windows as under
  WSL, and the made sounds 0.23 to 0.51 against 0.23 to 0.50. The late
  blocks are not the quieter under Windows in this run: this encoder's
  99th-percentile block is 20 to 84 µs there against 9 to 74, lower on
  seven recordings, the 16-bit stereo ones from archive.org, the indie
  rock, the Fantasy and the 1924 record, and higher on the other seven,
  the mono reading's 23 µs against 9; its worst block is 0.53 to 0.88 of
  WSL's on every recording but the loud master, 1.50, 58 to 194 µs in all,
  and libFLAC's worst block 123 to 514 µs where WSL has 154 to 2647: a
  worst block past a few hundred µs is the machine's under either.
  libFLAC's 24-bit tone comes to other bytes under MSVC, 2465 against 2321
  at level 5 and 2279 against 2302 at level 8, where this encoder's are
  the same. Against the Windows run before, the afternoon's with the
  encoder of v0.0.6, this encoder's block is 1.02 to 1.13 on the
  recordings and 0.97 to 1.10 on the made sounds, with libFLAC's three
  levels 0.88 to 1.17 beside it, and its bytes are the evening's change's,
  the Fantasy 0.9983 of that run's, the Aria 0.9989 and the 96 kHz tone
  0.990, as under WSL.

## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.38}$ | $\color{green}{0.37}$ | $\color{green}{0.44}$ | $\color{green}{0.30}$ | $\color{green}{0.49}$ | $\color{green}{0.46}$ |
| speech | $\color{green}{0.28}$ | $\color{green}{0.27}$ | $\color{green}{0.31}$ | $\color{green}{0.31}$ | $\color{green}{0.30}$ | $\color{green}{0.31}$ |
| tone | $\color{green}{0.50}$ | $\color{green}{0.40}$ | $\color{green}{0.48}$ | $\color{green}{0.40}$ | $\color{green}{0.50}$ | $\color{green}{0.47}$ |
| noise | $\color{green}{0.27}$ | $\color{green}{0.26}$ | $\color{green}{0.31}$ | $\color{green}{0.23}$ | $\color{green}{0.25}$ | $\color{green}{0.28}$ |
| silence | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.07}$ | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.05}$ |

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
| music | $\color{green}{0.84}$ | $\color{green}{0.75}$ | $\color{red}{1.05}$ | $\color{green}{0.54}$ | $\color{red}{1.11}$ | 1.00 |
| speech | $\color{green}{0.52}$ | $\color{green}{0.48}$ | $\color{green}{0.56}$ | $\color{green}{0.57}$ | $\color{green}{0.58}$ | $\color{green}{0.57}$ |
| tone | $\color{red}{1.14}$ | $\color{green}{0.84}$ | $\color{red}{1.16}$ | $\color{green}{0.75}$ | $\color{red}{1.12}$ | $\color{red}{1.06}$ |
| noise | $\color{green}{0.57}$ | $\color{green}{0.51}$ | $\color{green}{0.68}$ | $\color{green}{0.41}$ | $\color{green}{0.55}$ | $\color{green}{0.59}$ |
| silence | $\color{green}{0.10}$ | $\color{green}{0.10}$ | $\color{green}{0.16}$ | $\color{green}{0.10}$ | $\color{green}{0.10}$ | $\color{green}{0.10}$ |

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
| music | 44.1 | $\color{green}{16.8}$ | $\color{green}{0.38}$ | 0.08% | 1838 | 1838 | 1.00 | 47.9% |
| speech | 36.9 | $\color{green}{10.3}$ | $\color{green}{0.28}$ | 0.05% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 42.5 | $\color{green}{21.1}$ | $\color{green}{0.50}$ | 0.11% | 1008 | $\color{green}{997}$ | $\color{green}{0.99}$ | 26.0% |
| noise | 42.1 | $\color{green}{11.4}$ | $\color{green}{0.27}$ | 0.06% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 24.9 | $\color{green}{1.3}$ | $\color{green}{0.05}$ | 0.01% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 38.7 | $\color{green}{14.4}$ | $\color{green}{0.37}$ | 0.07% | 1702 | 1702 | 1.00 | 48.2% |
| speech | 32.8 | $\color{green}{8.9}$ | $\color{green}{0.27}$ | 0.04% | 1063 | $\color{red}{1064}$ | 1.00 | 30.1% |
| tone | 37.0 | $\color{green}{14.9}$ | $\color{green}{0.40}$ | 0.07% | 920 | 920 | 1.00 | 26.1% |
| noise | 37.3 | $\color{green}{9.6}$ | $\color{green}{0.26}$ | 0.05% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 23.6 | $\color{green}{1.2}$ | $\color{green}{0.05}$ | 0.01% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 74.3 | $\color{green}{32.6}$ | $\color{green}{0.44}$ | 0.16% | 3454 | 3454 | 1.00 | 45.0% |
| speech | 61.7 | $\color{green}{19.0}$ | $\color{green}{0.31}$ | 0.09% | 2226 | $\color{red}{2228}$ | 1.00 | 29.0% |
| tone | 71.9 | $\color{green}{34.4}$ | $\color{green}{0.48}$ | 0.17% | 1651 | $\color{green}{1481}$ | $\color{green}{0.90}$ | 19.3% |
| noise | 71.6 | $\color{green}{22.2}$ | $\color{green}{0.31}$ | 0.11% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 43.2 | $\color{green}{2.9}$ | $\color{green}{0.07}$ | 0.01% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 24.5 | $\color{green}{7.4}$ | $\color{green}{0.30}$ | 0.04% | 950 | 950 | 1.00 | 49.5% |
| speech | 25.2 | $\color{green}{7.9}$ | $\color{green}{0.31}$ | 0.04% | 1148 | 1148 | 1.00 | 59.8% |
| tone | 23.7 | $\color{green}{9.4}$ | $\color{green}{0.40}$ | 0.05% | 512 | 512 | 1.00 | 26.7% |
| noise | 24.2 | $\color{green}{5.5}$ | $\color{green}{0.23}$ | 0.03% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 17.8 | $\color{green}{0.9}$ | $\color{green}{0.05}$ | 0.00% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 41.5 | $\color{green}{20.3}$ | $\color{green}{0.49}$ | 0.10% | 456 | 456 | 1.00 | 23.7% |
| speech | 35.7 | $\color{green}{10.6}$ | $\color{green}{0.30}$ | 0.05% | 250 | 250 | 1.00 | 13.0% |
| tone | 40.7 | $\color{green}{20.5}$ | $\color{green}{0.50}$ | 0.10% | 476 | $\color{green}{470}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 41.5 | $\color{green}{10.3}$ | $\color{green}{0.25}$ | 0.05% | 1002 | $\color{red}{1006}$ | 1.00 | 52.4% |
| silence | 25.5 | $\color{green}{1.3}$ | $\color{green}{0.05}$ | 0.01% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 47.0 | $\color{green}{21.5}$ | $\color{green}{0.46}$ | 0.11% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 40.2 | $\color{green}{12.3}$ | $\color{green}{0.31}$ | 0.06% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 45.9 | $\color{green}{21.6}$ | $\color{green}{0.47}$ | 0.11% | 2321 | $\color{green}{2283}$ | $\color{green}{0.98}$ | 39.6% |
| noise | 45.9 | $\color{green}{12.9}$ | $\color{green}{0.28}$ | 0.06% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 26.0 | $\color{green}{1.3}$ | $\color{green}{0.05}$ | 0.01% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{green}{0.51}$ | 1.00 | $\color{red}{1.17}$ | $\color{green}{0.95}$ | $\color{green}{0.18}$ | 1.00 |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.43}$ | 1.00 | 1.00 | $\color{green}{0.87}$ | $\color{green}{0.17}$ | $\color{red}{1.01}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.42}$ | 1.00 | $\color{green}{0.92}$ | $\color{green}{0.78}$ | $\color{green}{0.16}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.49}$ | 1.00 | $\color{red}{1.05}$ | $\color{green}{0.96}$ | $\color{green}{0.16}$ | 1.00 |
| Mix at −13 LUFS, wide stereo | 44.1 kHz stereo 16-bit | $\color{green}{0.40}$ | 1.00 | $\color{green}{0.79}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Mix at −9 LUFS, loud master | 44.1 kHz stereo 16-bit | $\color{green}{0.40}$ | 1.00 | $\color{green}{0.80}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | $\color{green}{0.52}$ | 1.00 | $\color{red}{1.14}$ | $\color{green}{0.96}$ | $\color{green}{0.21}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.40}$ | 1.00 | $\color{green}{0.82}$ | $\color{green}{0.88}$ | $\color{green}{0.18}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.40}$ | 1.00 | $\color{green}{0.82}$ | $\color{green}{0.78}$ | $\color{green}{0.18}$ | 1.00 |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.30}$ | 1.00 | $\color{green}{0.52}$ | $\color{green}{0.97}$ | $\color{green}{0.22}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.35}$ | 1.00 | $\color{green}{0.64}$ | $\color{green}{0.49}$ | $\color{green}{0.13}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.39}$ | 1.00 | $\color{green}{0.80}$ | $\color{green}{0.94}$ | $\color{green}{0.18}$ | 1.00 |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.54}$ | 1.00 | $\color{red}{1.12}$ | $\color{green}{0.99}$ | $\color{green}{0.20}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.28}$ | 1.00 | $\color{green}{0.58}$ | $\color{green}{0.99}$ | $\color{green}{0.11}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms. *The run before* is this
encoder's mean block in the run of the commit before this one, and *over it*
this run's over that.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | The run before, mean µs | Over it | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 81.7 | $\color{green}{41.4}$ | $\color{green}{0.51}$ | 40.6 | $\color{red}{1.02}$ | 132 | $\color{green}{72.8}$ | 313 | $\color{green}{227}$ | 0.21% |
| Chopin, Fantasy Op. 49, piano | 74.4 | $\color{green}{32.4}$ | $\color{green}{0.43}$ | 32.2 | 1.00 | 125 | $\color{green}{54.9}$ | 2647 | $\color{green}{204}$ | 0.16% |
| Shostakovich 5, orchestra, live | 44.2 | $\color{green}{18.5}$ | $\color{green}{0.42}$ | 18.8 | $\color{green}{0.98}$ | 78.2 | $\color{green}{36.4}$ | 199 | $\color{green}{137}$ | 0.09% |
| Indie rock, 2022, mastered | 49.0 | $\color{green}{23.8}$ | $\color{green}{0.49}$ | 23.5 | $\color{red}{1.01}$ | 77.1 | $\color{green}{44.8}$ | 202 | $\color{green}{133}$ | 0.12% |
| Mix at −13 LUFS, wide stereo | 38.2 | $\color{green}{15.3}$ | $\color{green}{0.40}$ | 15.9 | $\color{green}{0.96}$ | 63.9 | $\color{green}{30.8}$ | 160 | $\color{green}{118}$ | 0.08% |
| Mix at −9 LUFS, loud master | 38.2 | $\color{green}{15.5}$ | $\color{green}{0.40}$ | 15.5 | 1.00 | 66.4 | $\color{green}{30.5}$ | 154 | $\color{green}{87.0}$ | 0.08% |
| Limehouse Blues, 1924 78 rpm | 86.0 | $\color{green}{44.8}$ | $\color{green}{0.52}$ | 43.5 | $\color{red}{1.03}$ | 138 | $\color{green}{74.1}$ | 291 | $\color{green}{184}$ | 0.22% |
| Prison work song, 1947 field recording | 38.0 | $\color{green}{15.2}$ | $\color{green}{0.40}$ | 15.2 | 1.00 | 65.0 | $\color{green}{30.4}$ | 178 | $\color{green}{120}$ | 0.08% |
| The Raven, read, produced | 38.5 | $\color{green}{15.6}$ | $\color{green}{0.40}$ | 15.8 | $\color{green}{0.99}$ | 70.1 | $\color{green}{31.7}$ | 270 | $\color{green}{127}$ | 0.08% |
| Leviticus 13, read | 22.8 | $\color{green}{7.0}$ | $\color{green}{0.30}$ | 6.9 | 1.00 | 42.2 | $\color{green}{8.7}$ | 186 | $\color{green}{86.2}$ | 0.03% |
| JFK at Rice, 1962 | 40.7 | $\color{green}{14.1}$ | $\color{green}{0.35}$ | 13.9 | $\color{red}{1.01}$ | 74.1 | $\color{green}{29.9}$ | 246 | $\color{green}{171}$ | 0.07% |
| Train station, field recording | 38.0 | $\color{green}{14.8}$ | $\color{green}{0.39}$ | 14.7 | 1.00 | 59.3 | $\color{green}{29.6}$ | 159 | $\color{green}{91.4}$ | 0.07% |
| Pink noise | 47.6 | $\color{green}{25.5}$ | $\color{green}{0.54}$ | 25.1 | $\color{red}{1.02}$ | 84.1 | $\color{green}{47.8}$ | 347 | $\color{green}{164}$ | 0.13% |
| White noise | 46.6 | $\color{green}{13.1}$ | $\color{green}{0.28}$ | 13.2 | $\color{green}{0.99}$ | 81.2 | $\color{green}{28.3}$ | 296 | $\color{green}{142}$ | 0.07% |

### The bytes, libFLAC at level 5

*The run before* is this encoder's bytes a block in the run of the commit
before this one, and *over it* this run's over that, to a ten-thousandth.

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | The run before, bytes | Over it | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5313}$ | 1.000 | 5313 | 1.0000 | 46.1% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2478}$ | $\color{red}{1.001}$ | 2478 | 1.0000 | 32.3% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1499}$ | 1.000 | 1499 | 1.0000 | 39.0% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{red}{4556}$ | 1.000 | 4556 | 1.0000 | 79.1% |
| Mix at −13 LUFS, wide stereo | 250 | 3528 | 2325 | $\color{red}{2327}$ | $\color{red}{1.001}$ | 2327 | 1.0000 | 66.0% |
| Mix at −9 LUFS, loud master | 225 | 3528 | 2543 | $\color{red}{2547}$ | $\color{red}{1.001}$ | 2547 | 1.0000 | 72.2% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | $\color{green}{8657}$ | 1.000 | 8657 | 1.0000 | 75.1% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | $\color{red}{1421}$ | $\color{red}{1.001}$ | 1421 | 1.0000 | 40.3% |
| The Raven, read, produced | 559 | 3528 | 1362 | $\color{red}{1363}$ | 1.000 | 1363 | 1.0000 | 38.6% |
| Leviticus 13, read | 725 | 1764 | 986 | $\color{red}{987}$ | 1.000 | 987 | 1.0000 | 55.9% |
| JFK at Rice, 1962 | 836 | 5760 | 1798 | $\color{green}{1797}$ | 1.000 | 1797 | 1.0000 | 31.2% |
| Train station, field recording | 141 | 3528 | 1272 | 1272 | 1.000 | 1272 | 1.0000 | 36.1% |
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
| music | $\color{green}{0.35}$ | $\color{green}{0.33}$ | $\color{green}{0.37}$ | $\color{green}{0.36}$ | $\color{green}{0.47}$ | $\color{green}{0.38}$ |
| speech | $\color{green}{0.26}$ | $\color{green}{0.25}$ | $\color{green}{0.27}$ | $\color{green}{0.37}$ | $\color{green}{0.28}$ | $\color{green}{0.26}$ |
| tone | $\color{green}{0.46}$ | $\color{green}{0.37}$ | $\color{green}{0.43}$ | $\color{green}{0.46}$ | $\color{green}{0.49}$ | $\color{green}{0.39}$ |
| noise | $\color{green}{0.24}$ | $\color{green}{0.22}$ | $\color{green}{0.26}$ | $\color{green}{0.26}$ | $\color{green}{0.22}$ | $\color{green}{0.25}$ |
| silence | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.10}$ | $\color{green}{0.07}$ | $\color{green}{0.06}$ |

### Time a block on arm64, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on arm64.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.08}$ | $\color{green}{0.97}$ | $\color{red}{1.23}$ | $\color{green}{0.82}$ | $\color{red}{1.45}$ | $\color{red}{1.07}$ |
| speech | $\color{green}{0.63}$ | $\color{green}{0.59}$ | $\color{green}{0.68}$ | $\color{green}{0.83}$ | $\color{green}{0.74}$ | $\color{green}{0.59}$ |
| tone | $\color{red}{1.43}$ | $\color{red}{1.07}$ | $\color{red}{1.39}$ | $\color{red}{1.09}$ | $\color{red}{1.51}$ | $\color{red}{1.09}$ |
| noise | $\color{green}{0.64}$ | $\color{green}{0.57}$ | $\color{green}{0.71}$ | $\color{green}{0.55}$ | $\color{green}{0.66}$ | $\color{green}{0.62}$ |
| silence | $\color{green}{0.16}$ | $\color{green}{0.16}$ | $\color{green}{0.17}$ | $\color{green}{0.20}$ | $\color{green}{0.16}$ | $\color{green}{0.14}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 |
|---|---:|---:|---:|---:|---:|---:|
| music | 44.1 | 47.1 | 16.8 | 16.6 | $\color{green}{0.38}$ | $\color{green}{0.35}$ |
| speech | 36.9 | 39.0 | 10.3 | 10.0 | $\color{green}{0.28}$ | $\color{green}{0.26}$ |
| tone | 42.5 | 45.0 | 21.1 | 20.8 | $\color{green}{0.50}$ | $\color{green}{0.46}$ |
| noise | 42.1 | 43.3 | 11.4 | 10.3 | $\color{green}{0.27}$ | $\color{green}{0.24}$ |
| silence | 24.9 | 23.7 | 1.3 | 1.7 | $\color{green}{0.05}$ | $\color{green}{0.07}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 | 99th, x86 | 99th, arm64 | Time vs 0, x86 | Time vs 0, arm64 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 81.7 | 92.9 | 41.4 | 37.5 | $\color{green}{0.51}$ | $\color{green}{0.40}$ | 72.8 | 51.5 | $\color{red}{1.17}$ | $\color{red}{1.14}$ |
| Chopin, Fantasy Op. 49, piano | 74.4 | 85.5 | 32.4 | 32.7 | $\color{green}{0.43}$ | $\color{green}{0.38}$ | 54.9 | 46.3 | 1.00 | $\color{red}{1.22}$ |
| Shostakovich 5, orchestra, live | 44.2 | 46.2 | 18.5 | 18.9 | $\color{green}{0.42}$ | $\color{green}{0.41}$ | 36.4 | 26.4 | $\color{green}{0.92}$ | $\color{red}{1.19}$ |
| Indie rock, 2022, mastered | 49.0 | 52.9 | 23.8 | 21.8 | $\color{green}{0.49}$ | $\color{green}{0.41}$ | 44.8 | 33.6 | $\color{red}{1.05}$ | $\color{red}{1.09}$ |
| Mix at −13 LUFS, wide stereo | 38.2 | 42.1 | 15.3 | 15.5 | $\color{green}{0.40}$ | $\color{green}{0.37}$ | 30.8 | 23.7 | $\color{green}{0.79}$ | $\color{green}{0.99}$ |
| Mix at −9 LUFS, loud master | 38.2 | 41.8 | 15.5 | 15.4 | $\color{green}{0.40}$ | $\color{green}{0.37}$ | 30.5 | 21.6 | $\color{green}{0.80}$ | $\color{green}{0.98}$ |
| Limehouse Blues, 1924 78 rpm | 86.0 | 96.7 | 44.8 | 40.3 | $\color{green}{0.52}$ | $\color{green}{0.42}$ | 74.1 | 55.9 | $\color{red}{1.14}$ | $\color{red}{1.11}$ |
| Prison work song, 1947 field recording | 38.0 | 40.9 | 15.2 | 15.2 | $\color{green}{0.40}$ | $\color{green}{0.37}$ | 30.4 | 20.9 | $\color{green}{0.82}$ | $\color{red}{1.03}$ |
| The Raven, read, produced | 38.5 | 41.4 | 15.6 | 15.5 | $\color{green}{0.40}$ | $\color{green}{0.38}$ | 31.7 | 22.5 | $\color{green}{0.82}$ | $\color{red}{1.03}$ |
| Leviticus 13, read | 22.8 | 19.5 | 7.0 | 7.0 | $\color{green}{0.30}$ | $\color{green}{0.36}$ | 8.7 | 8.4 | $\color{green}{0.52}$ | $\color{green}{0.76}$ |
| JFK at Rice, 1962 | 40.7 | 43.0 | 14.1 | 13.2 | $\color{green}{0.35}$ | $\color{green}{0.31}$ | 29.9 | 21.8 | $\color{green}{0.64}$ | $\color{green}{0.69}$ |
| Train station, field recording | 38.0 | 40.6 | 14.8 | 14.5 | $\color{green}{0.39}$ | $\color{green}{0.36}$ | 29.6 | 19.6 | $\color{green}{0.80}$ | 1.00 |
| Pink noise | 47.6 | 49.6 | 25.5 | 22.6 | $\color{green}{0.54}$ | $\color{green}{0.45}$ | 47.8 | 30.2 | $\color{red}{1.12}$ | $\color{red}{1.14}$ |
| White noise | 46.6 | 49.0 | 13.1 | 12.5 | $\color{green}{0.28}$ | $\color{green}{0.25}$ | 28.3 | 25.3 | $\color{green}{0.58}$ | $\color{green}{0.63}$ |

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
| music | $\color{green}{0.38}$ | $\color{green}{0.40}$ | $\color{green}{0.45}$ | $\color{green}{0.31}$ | $\color{green}{0.51}$ | $\color{green}{0.44}$ |
| speech | $\color{green}{0.28}$ | $\color{green}{0.29}$ | $\color{green}{0.32}$ | $\color{green}{0.32}$ | $\color{green}{0.30}$ | $\color{green}{0.31}$ |
| tone | $\color{green}{0.51}$ | $\color{green}{0.40}$ | $\color{green}{0.51}$ | $\color{green}{0.42}$ | $\color{green}{0.51}$ | $\color{green}{0.47}$ |
| noise | $\color{green}{0.26}$ | $\color{green}{0.27}$ | $\color{green}{0.31}$ | $\color{green}{0.23}$ | $\color{green}{0.25}$ | $\color{green}{0.27}$ |
| silence | $\color{green}{0.06}$ | $\color{green}{0.06}$ | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.06}$ | $\color{green}{0.05}$ |

### Time a block on Windows, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on Windows.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.80}$ | $\color{green}{0.73}$ | $\color{red}{1.04}$ | $\color{green}{0.53}$ | $\color{red}{1.13}$ | $\color{green}{0.93}$ |
| speech | $\color{green}{0.49}$ | $\color{green}{0.47}$ | $\color{green}{0.56}$ | $\color{green}{0.55}$ | $\color{green}{0.57}$ | $\color{green}{0.54}$ |
| tone | $\color{red}{1.15}$ | $\color{green}{0.81}$ | $\color{red}{1.18}$ | $\color{green}{0.76}$ | $\color{red}{1.13}$ | $\color{red}{1.05}$ |
| noise | $\color{green}{0.55}$ | $\color{green}{0.48}$ | $\color{green}{0.65}$ | $\color{green}{0.40}$ | $\color{green}{0.54}$ | $\color{green}{0.56}$ |
| silence | $\color{green}{0.11}$ | $\color{green}{0.11}$ | $\color{green}{0.16}$ | $\color{green}{0.11}$ | $\color{green}{0.11}$ | $\color{green}{0.10}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, WSL µs | libFLAC, Windows µs | This encoder, WSL µs | This encoder, Windows µs | Ratio, WSL | Ratio, Windows |
|---|---:|---:|---:|---:|---:|---:|
| music | 44.1 | 42.5 | 16.8 | 16.0 | $\color{green}{0.38}$ | $\color{green}{0.38}$ |
| speech | 36.9 | 35.8 | 10.3 | 10.0 | $\color{green}{0.28}$ | $\color{green}{0.28}$ |
| tone | 42.5 | 41.5 | 21.1 | 21.0 | $\color{green}{0.50}$ | $\color{green}{0.51}$ |
| noise | 42.1 | 42.6 | 11.4 | 10.9 | $\color{green}{0.27}$ | $\color{green}{0.26}$ |
| silence | 24.9 | 23.0 | 1.3 | 1.4 | $\color{green}{0.05}$ | $\color{green}{0.06}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, WSL µs | libFLAC, Windows µs | This encoder, WSL µs | This encoder, Windows µs | Ratio, WSL | Ratio, Windows | 99th, WSL | 99th, Windows | Time vs 0, WSL | Time vs 0, Windows |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 81.7 | 79.6 | 41.4 | 42.5 | $\color{green}{0.51}$ | $\color{green}{0.53}$ | 72.8 | 84.3 | $\color{red}{1.17}$ | $\color{red}{1.17}$ |
| Chopin, Fantasy Op. 49, piano | 74.4 | 70.4 | 32.4 | 31.6 | $\color{green}{0.43}$ | $\color{green}{0.45}$ | 54.9 | 49.0 | 1.00 | 1.00 |
| Shostakovich 5, orchestra, live | 44.2 | 42.1 | 18.5 | 18.6 | $\color{green}{0.42}$ | $\color{green}{0.44}$ | 36.4 | 31.4 | $\color{green}{0.92}$ | $\color{green}{0.95}$ |
| Indie rock, 2022, mastered | 49.0 | 48.9 | 23.8 | 23.4 | $\color{green}{0.49}$ | $\color{green}{0.48}$ | 44.8 | 37.3 | $\color{red}{1.05}$ | 1.00 |
| Mix at −13 LUFS, wide stereo | 38.2 | 36.8 | 15.3 | 16.1 | $\color{green}{0.40}$ | $\color{green}{0.44}$ | 30.8 | 35.4 | $\color{green}{0.79}$ | $\color{green}{0.79}$ |
| Mix at −9 LUFS, loud master | 38.2 | 35.7 | 15.5 | 15.7 | $\color{green}{0.40}$ | $\color{green}{0.44}$ | 30.5 | 33.1 | $\color{green}{0.80}$ | $\color{green}{0.80}$ |
| Limehouse Blues, 1924 78 rpm | 86.0 | 96.3 | 44.8 | 42.5 | $\color{green}{0.52}$ | $\color{green}{0.44}$ | 74.1 | 65.4 | $\color{red}{1.14}$ | $\color{green}{0.92}$ |
| Prison work song, 1947 field recording | 38.0 | 35.0 | 15.2 | 15.2 | $\color{green}{0.40}$ | $\color{green}{0.43}$ | 30.4 | 21.0 | $\color{green}{0.82}$ | $\color{green}{0.82}$ |
| The Raven, read, produced | 38.5 | 35.2 | 15.6 | 15.5 | $\color{green}{0.40}$ | $\color{green}{0.44}$ | 31.7 | 22.8 | $\color{green}{0.82}$ | $\color{green}{0.81}$ |
| Leviticus 13, read | 22.8 | 21.5 | 7.0 | 7.2 | $\color{green}{0.30}$ | $\color{green}{0.33}$ | 8.7 | 23.1 | $\color{green}{0.52}$ | $\color{green}{0.52}$ |
| JFK at Rice, 1962 | 40.7 | 40.5 | 14.1 | 14.6 | $\color{green}{0.35}$ | $\color{green}{0.36}$ | 29.9 | 33.1 | $\color{green}{0.64}$ | $\color{green}{0.64}$ |
| Train station, field recording | 38.0 | 35.3 | 14.8 | 14.4 | $\color{green}{0.39}$ | $\color{green}{0.41}$ | 29.6 | 19.8 | $\color{green}{0.80}$ | $\color{green}{0.78}$ |
| Pink noise | 47.6 | 48.5 | 25.5 | 26.2 | $\color{green}{0.54}$ | $\color{green}{0.54}$ | 47.8 | 53.1 | $\color{red}{1.12}$ | $\color{red}{1.13}$ |
| White noise | 46.6 | 47.2 | 13.1 | 13.3 | $\color{green}{0.28}$ | $\color{green}{0.28}$ | 28.3 | 31.1 | $\color{green}{0.58}$ | $\color{green}{0.56}$ |

The bytes a block of this encoder are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 5 differ between the two runs on tone at 48 kHz stereo 24-bit (2321 against 2465).

The bytes a block of libFLAC at level 0 are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 8 differ between the two runs on tone at 48 kHz stereo 24-bit (2302 against 2279).

