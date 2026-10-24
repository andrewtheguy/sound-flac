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
and fifteen recordings, twelve from archive.org, public domain or CC0, and
three of the user's own, whole, in blocks of 20 ms. A made sound's time is one block's, the least of five passes' mean over
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
git show HEAD:docs/bench/x86.jsonl > before.jsonl       # the run before, from the commit before; a commit from before the JSON lines holds x86.log, whose key=value lines are made JSON objects by hand first
python3 -I tools/tables.py docs/bench/x86.jsonl arm64=docs/bench/arm64.jsonl --before before.jsonl
```

In the recordings tables below, *the run before* is the run of v0.0.8,
made late the evening of 2026-10-23 on the desktop of the runs before it,
a six-core Intel i5-8500T under WSL2, whose log that release carries as
`docs/bench/x86.jsonl`; this run is of the same encoder, v0.0.8's, with
nothing changed, on another machine. The x86 run has moved: every run
before this one was made on that desktop, which was never quiet, the
session that started the run and the system under it taking a tenth of a
time from one run to the next, and libFLAC's own times with it, so that a
ratio's last hundredths were the machine's; from this run on it is made
on a host that does nothing else, over ssh, and the *run before* columns
of this one run set the two machines side by side, the time machine to
machine and the bytes, which a machine does not move, like for like.
The run also adds a fifteenth recording, the third of the user's own,
one voice talking over soft music, which the run before has no line for,
so its *run before* cells are blank. The days named in this document and
in [plan.md](plan.md) before the evening of 2026-10-23 are the count the
sessions that made the earlier runs kept, which ran ahead of the
machine's clock: by the commits, which that clock stamps in Pacific time,
the run of v0.0.5 was made at noon on 2026-10-23, the run of v0.0.6 that
afternoon, the run of v0.0.7 later that evening and the run of v0.0.8
later still, and this run the morning after, 2026-10-24, which is the
clock's own day and is named by it from here on.

They were run on 2026-10-24 by the machine's clock, in the morning, on a
KVM virtual machine with four virtual processors of QEMU's EPYC-Rome
model at 2.8 GHz, two cores with two threads each, the host's own chip
not shown to it, under Debian 13, one
thread, nothing else running on it, with rustc 1.98.1 at the x86-64
baseline (SSE2, no `target-cpu`; the machine has AVX2 and the fused
multiply-add and the build asks for neither), from the tree of this
commit copied over, the recordings copied from the store to it the same
hour: the encoder of v0.0.8, unchanged. The run was made twice, the
first with rustc 1.99.1 right after the recordings were copied and the
second, the one kept, with 1.98.1, the compiler of the arm64 run and of
the runs before, a minute after its build; the two agree within 6% on
every line of every coder, this encoder 0.93 to 1.05 of the first on the
made sounds and 0.94 to 1.05 on the recordings, libFLAC's level 5 0.95
to 1.06, its level 0 0.95 to 1.05 and its level 8 0.96 to 1.05, for the
same bytes, so a time on this machine moves by a twentieth from one run
to the next where the desktop's moved by a tenth, and a time ratio
within a twentieth of 1.00 is on the line; a bytes ratio is exact. Its
bytes are the run before's on every line of every coder, four coders on
45 lines, the encoder being the same and a frame not depending on the
machine; its times are another machine's. This encoder's block is 0.64
to 0.68 of the desktop's on every recording and 0.60 to 0.67 on every
made sound but the silences, 0.61 to 0.79 of a block of a microsecond;
libFLAC's level 5 is 0.74 to 0.84 of its on the stereo recordings and
0.90 on the mono reading, 0.71 to 0.90 on the made sounds, its level 0
0.80 to 0.96 on the stereo recordings and 1.14 on the mono reading,
0.82 to 1.22 on the made sounds, the mono shape the over, and its level
8 0.82 to 0.90 on the recordings and 0.75 to 1.04 on the made sounds.
This machine runs this encoder half as fast again as the desktop did
and libFLAC's default level a quarter faster, its fastest level hardly
faster and on one channel slower, so every time ratio in the tables
reads better for this encoder than the run before's by a fifth or so,
and against level 0 by a quarter, for nothing the encoder did: the
*over it* column of the block times reads 0.64 to 0.68 down the
recordings, the move of machine, and the *over it* column of the bytes
1.0000 down them, the one column that compares like with like. A worst
block is one block of thousands and the machine's as much as the
encoder's, even as the least over passes, so it is read as an order of
magnitude; on this machine it is 114 to 146 µs on every recording for
this encoder, the mono reading's 25 times its mean block, which is the
virtual machine's tick and not the sound, where libFLAC's is 131 to
230. The bench also logs `worst_at`, the block that was slowest at its
least over the passes, and dumps every block's least time and bytes
when asked (`BLOCKS=dir`), which is how the late blocks below were
read; every pass starts from an encoder that has seen no block, so that
what the encoder carries from block to block, the fourth-block choice,
falls on the same blocks in every pass and the least over the passes is
the block's own.

The arm64 run of the section after the recordings was made the same
morning from the same tree on a two-core Ampere Altra (Neoverse N1) in
a Debian 13 LXC container, one thread, the machine idle, with rustc
1.98.1 at the aarch64 baseline, whose NEON the compiler uses on its own
and whose fused multiply-add the autocorrelation asks for: the tree is
copied there, the recordings copied again from the store, the bench run
once the load had fallen under 0.3 for the file system's compression of
what was copied, seven minutes, and its log brought back as the second
log of `tools/tables.py`. A run started while that compression is still
going reads two to three times slow on the made sounds, as the first
run there found on 2026-10-23, so the wait is part of the run. Against
its run before, the same encoder on the same machine the evening
before, every line of every coder is within 2%: this encoder 1.00 to
1.01 of it on every made sound and every recording, libFLAC's level 5
0.99 to 1.02, its level 0 0.98 to 1.02 and its level 8 1.00 to 1.02,
for the same bytes on every line, so that machine repeats a time to a
hundredth and its tables are the run before's but for the new
recording's line. In this run this encoder's block is 1.24 to 1.46 of
the EPYC's on every line, the recordings 1.24 to 1.43 and the mono
reading 1.37, libFLAC's level 5 is 1.30 to 1.61 of its on the stereo
recordings and 1.01 on the mono reading, 1.02 to 1.61 on the made
sounds but a mono silence, its level 0 0.84 to 1.20 on the recordings
and 0.68 on the mono reading, 0.61 to 1.13 on the made sounds, and its
level 8 1.14 to 1.88 but the silences, 0.75 to 1.37. On the desktop of the runs before this encoder's
block was shorter on the Altra than there on every recording, by 0.83
to 0.94 of it, and level 0's shorter still; on this machine it is the
EPYC that is the shorter on everything but level 0's fixed path on one
channel, which the Altra runs in 0.68 of its time. The run before's
level 0 cells on the Altra were checked, after v0.0.7, with the bench
timing the four coders together, every block by each coder in turn
(`INTERLEAVE=1`), so that both sides of a ratio see the machine in one
state, and the control put every made-sound ratio of the arm64 tables
within 0.03 of the ordinary run's, the coders' times within 2%; the
cells were the machine's, level 0's fixed path being cheap on the
Altra, 0.78 to 0.99 of its i5 time, and this encoder's block the same
length on the two. On the EPYC the same cells read 0.62 on 48 kHz music
and 0.72 on its tone, 0.50 and 0.71 at 8 bits, 0.89 and 1.01 at 96 kHz,
against 0.98 and 1.11, 0.84 and 1.15, 1.12 and 1.27 on the Altra, this
encoder's block being 0.70 to 0.75 of its Altra block on those six
lines and level 0's 0.88 to 1.16 of it.

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
the repository holds a recording. Three more are the user's own, not
licensed for publication: two dense mixes and one voice talking over soft
music, all at 44.1 kHz stereo 16-bit, WAVs at the root of the store named
by their track numbers, not in `tools/samples.txt` and not fetched. The
tables carry their numbers, named for what they sound like, and the
repository nothing else of them.

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
| Talk over soft music | the user's own | 44.1 kHz stereo 16-bit | 181 | one voice speaking with a little music under it, −17.5 LUFS over 5.9 LU, the side channel 10.8 dB under the mix: 45% of its PCM |
| Leviticus 13, read | `LeviticusKJV` | 44.1 kHz mono 16-bit | 725 | one voice reading, a room's quiet between verses |
| JFK at Rice, 1962 | `NasaAudioHighlightReels` | 48 kHz stereo 24-bit | 836 | a speech through a public-address system, a crowd |
| Train station, field recording | `aporee_53029_60598` | 44.1 kHz stereo 16-bit | 141 | a passenger train arriving, the station's ambience |
| Pink noise | `TenMinutesOfWhiteNoisePinkNoiseAndBrownianNoise` | 48 kHz stereo 24-bit | 600 | noise falling 3 dB an octave, predictable a little |
| White noise | `TenMinutesOfWhiteNoisePinkNoiseAndBrownianNoise` | 48 kHz stereo 24-bit | 600 | nothing to predict: the most a stream can cost |

## What the tables say

- **Every recording and every made sound takes less than half of what
  libFLAC at its default level takes of a core, most of them about a
  third or less.** The recordings are 0.24 to 0.47 of its time a block:
  16-bit stereo 0.32 to 0.39, mono 0.24, 24-bit 0.24 to 0.47, the two at
  96 kHz and 24 bits 0.45 and 0.47. A block of a recording is 0.02% to
  0.15% of one core. The made sounds are 0.05 to 0.44: music 0.25 to
  0.39, speech 0.17 to 0.27, the tone 0.19 to 0.44, noise 0.18 to 0.27,
  silence 0.05 to 0.06. The run before, the same encoder on the i5 late
  the evening of 2026-10-23, had them at 0.29 to 0.55 and 0.05 to 0.50,
  the run of v0.0.7 before it that evening at 0.28 to 0.54 and 0.23 to
  0.50, the run before it, the same encoder as v0.0.7's, at 0.30 to 0.57
  and 0.25 to 0.53, the run of v0.0.6 before it, that afternoon, at 0.28
  to 0.52 and 0.23 to 0.52, the run of v0.0.5 before it at 0.34 to 0.58
  and 0.27 to 0.55, the evening's before that 0.33 to 0.55 and 0.27 to
  0.53, the afternoon's 0.31 to 0.59 and 0.25 to 0.49, the morning's 0.44
  to 0.77 and 0.40 to 0.74, for the same bytes within half a per cent
  but the tone's, which fell, and two runs before the pianos', which fell
  by a tenth of a per cent. This run's move from the run before's is the
  machine's, not the encoder's, which is the same: this encoder's block
  is 0.64 to 0.68 of its i5 block on the recordings and 0.60 to 0.67 on
  the made sounds but the silences, and libFLAC's default level 0.74 to
  0.84 on the stereo recordings, 0.90 on the mono reading and 0.71 to
  0.90 on the made sounds, so the ratios read a fifth better here for
  the same work. On arm64, where the run is the run before's within 2%
  on every line, they are 0.25 to 0.42 and 0.06 to 0.39.
- **The bytes are libFLAC's at its default level, within 0.2%, and on a
  pure tone under them.** Every recording is 1.000 but the Fantasy, the
  prison song, the orchestra, the two mixes and the talk at 1.001, a
  byte to five a block,
  which is what fitting only the carried pair between full choices and
  coding it the fit first cost them (the measurements are in
  [plan.md](plan.md)); before the pair was chosen on the fit the train
  station was 1.006 and the Fantasy 1.004, and before it was chosen on the
  four channels' counts the Fantasy was 1.002 and the Aria 1.001. Against
  the run before every line comes to the same bytes, this run being the
  same encoder on another machine; that run's encoder, against the one
  before it, had brought the Raven, the train station and the orchestra
  a tenth of a byte a block over, 0.01%, and the made 8-bit speech 0.3
  of a byte, 0.12%: blocks where the fit, skipped after two losses by an
  eighth of a bit, would have won. The encoder before that, against the
  afternoon's, had brought every recording to the same bytes or fewer,
  the Fantasy 0.9983, the Aria 0.9989, the train station 0.9996, the
  Raven and the prison song 0.9999, which is the pair chosen on the
  counts where the estimates could not tell left and right from mid and
  side, and every made sound to the same bytes but the pure tone at 96
  kHz, 0.990 of the afternoon's. The tone is 0.99 of libFLAC's at 48
  kHz, 1.00 at 44.1 kHz where it was 1.02, 0.90 at 96 kHz where it was
  0.95, 0.98 at 24 bits where it was 1.05 and 0.99 at 8 bits: a tone is
  predicted away by order 2 and what its frame then costs is the
  rounding of the coefficients, which the order choice now weighs
  exactly, from the autocorrelation, where before it was blind to it and
  the orders tied. The made speech, whose two channels are one signal, is
  0.05% over libFLAC and 0.12% at 8 bits, the skip's: its side channel
  is a constant, and a block with a constant channel is coded the fit
  first like any other where it had every channel coded both ways.
  Against libFLAC's smallest level, 8, which tries every order to 12 and
  every stereo pairing in six to fourteen times this encoder's time on
  this machine, the made sounds are 0.99 to 1.01, the tone at 96 kHz and
  24 bits 1.00 and 0.99 where they were 1.07 and 1.06, and the
  recordings 1.000 to 1.009, the orchestra the 1.009.
- **The late block is still the fourth block, where the pair is chosen
  afresh, by a quarter to a third of a carried block.** Each block's
  least time over the passes, each pass from an encoder that has seen no
  block, read by the block's place in the cycle of four, in a run on
  this machine after this run's tables with every block dumped
  (`BLOCKS=dir`): the fourth block is 14.3 µs against 10.9 for a carried
  block on the orchestra, where the run before had 21.4 against 16.5 on
  the i5; 31.3 against 23.4 on the Aria, from 48.6 against 36.6; 24.4
  against 19.3 on the Fantasy, from 36.7 against 29.6; 12.2 against 9.2
  on the Raven, from 18.4 against 14.2; 11.5 against 9.4 on the wide
  mix, from 17.3 against 14.3; on the made tone 15.1 against 9.7, from
  22.1 against 14.7, the fit skipped on the channel where it loses, and
  on the made music, where the estimates settle the pair and the two
  other channels are not counted, 11.3 against 10.2, from 16.8 against
  15.4; every pair of them the i5's at 0.64 to 0.68, the fourth block
  and the carried one moved alike by the machine. Where the
  estimates do not settle it, the fourth block has the two other channels'
  residuals and folds over what it had, which is the Aria's and the
  Fantasy's rise, and against that the run before's choice coded 1.9 pairs
  on the orchestra's mean fourth block, which is why that block did not
  move there. The two blocks after the
  choice, where each channel of the pair has both codings made once to
  bring its correction up to date, are about half a microsecond over a
  plain carried block at 16 bits, 11.4 and 10.6 against 10.6 on the
  orchestra, and 2.4 on the Aria. This encoder's own 99th-percentile
  block is set by the loud blocks as much as by the fourth, 1.5 to 2.3
  times its mean on the stereo recordings and 1.4 on the mono reading,
  and its worst block on this machine is the virtual machine's tick: 114
  to 146 µs on every recording as the least over the passes of each
  pass's worst, where no one block is slow in every pass, the slowest
  block of the orchestra at its least over the passes being 29 µs, the
  Aria's 43 and the Fantasy's 34, so the tick falls on another block
  each pass and a worst block past a hundred µs is the machine's. A
  single pass's 99th percentile is higher still for both encoders, which
  is scheduling and cache, not the sound. On the recordings this encoder's
  99th-percentile block is 0.20 to 0.58 of libFLAC's, 6 to 49 µs against
  31 to 84, and its worst block 114 to 146 µs, at most 0.7% of the 20 ms
  the block is, where libFLAC's worst is 131 to 230. Where two channels
  are one signal, the 1962 speech and the made speech, the fourth block
  counts the three equivalent channels, each the same bits within the
  estimates' margin: 12.8 µs against 7.5 on that recording and 7.8
  against 5.8 on the made speech, the blocks between coding one channel
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
  the Aria at 24 bits nearly all, and on 8-bit music none, where the
  fixed coding wins every block and the fit never does, losing by a
  quarter of a bit a sample while estimated to win by a tenth. The two other channels are not looked at
  past a constant check and a bound from one pass over their differences,
  and the pair moves where the bound says another is certainly cheaper,
  160 times in 12 000 blocks of the wide mix and once in the Raven's. A
  channel on its own, mono or more than two, is coded the fit first on
  every block. Where the fit is estimated not to win on a channel of fixed
  order 0, which is noise and a quiet desktop, it is skipped for seven
  blocks, and so it is where it has lost to the fixed coding, both coded,
  by an eighth of a bit a sample two blocks running, which is 8-bit music
  and one channel of the made tone, and took 8-bit music from 20 µs a
  block to 13 and the tone from 21 to 18 on the i5 for the same bytes,
  which on this machine are 8.0 and 11.2. The margin
  is the point: on the recordings the fit loses on a tenth to a quarter of
  the blocks where both are coded, but mostly within a sixteenth of a bit,
  and the next block's fit wins, so a loss under the eighth ends the run
  of losses; skipping on any loss cost the prison song 0.7% of its bytes,
  and on two losses past an eighth it and every recording are within
  0.01%.
- **Where the time goes at 16-bit stereo**, on the orchestra, by the parts
  measured apart with timers in a copy of the tree on the i5 of the runs
  before, whose whole carried block was 16.9 µs where this machine's is
  10.9 and its fourth block 14.3 where the i5's was 22, every part about
  two thirds of what is written here: a carried block, three
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
  stereo as it comes**, this encoder takes less time on every recording
  on this machine: 0.58 to 0.87 of its time on the 16-bit stereo
  recordings, the Fantasy at 96 kHz the 0.87, the orchestra 0.68 and the
  six at 44.1 kHz 0.58 to 0.63, 0.48 on the 1962 speech, 0.44 on white
  noise, 0.34 on the mono reading, and 0.78 to 0.98 on the four at 24
  bits with sound in them, the Aria the 0.98, for 0.49 to 0.99 of its
  bytes: the three speeches 0.49, 0.78 and 0.87, the music 0.78 to 0.96,
  noise 0.99. On the made sounds it is 0.50 to 0.89 on stereo music,
  8-bit music the 0.50 and 96 kHz the 0.89, 0.62 to 1.01 on the tone, 48
  kHz 0.72 and 96 kHz the 1.01, the one cell over 1.00 on this machine,
  0.29 to 0.49 on speech, 0.27 to 0.55 on noise, 0.07 to 0.39 on mono
  and 0.07 to 0.11 on silence. The run before, on the i5, had the 16-bit
  stereo recordings at 0.85 to 1.09, the 1962 speech 0.69, white noise
  0.61, the mono reading 0.59 and the 24-bit four at 1.10 to 1.24, the
  made stereo music and tone at 0.76 to 1.11 and 0.90 to 1.29: the
  column moved by a quarter, all of it the machine's, this encoder's
  block being 0.64 to 0.68 of its i5 block and level 0's 0.80 to 0.96 of
  its on the stereo recordings, 1.14 on the mono reading, and 0.82 to
  1.22 on the made sounds, the mono shape the over, so the EPYC runs
  this encoder half as fast again as the i5 and level 0's fixed path
  about as fast. On arm64, where the run is the run before's within 2%,
  it reads 0.90 to 1.12 on the 16-bit stereo recordings, the Fantasy
  the 1.12 and the orchestra 1.07, the two mixes 0.90 and 0.93, the talk
  0.96, the 1962 speech and white noise 0.64 and 0.63, the mono reading
  0.69, and the 24-bit four 1.02 to 1.07; on the made sounds 0.84 to
  1.12 on stereo music, 48 kHz 0.98 and 8-bit 0.84, and 0.98 to 1.27 on
  the tone, 48 kHz 1.11 and 8-bit 1.15. What is over 1.00 on the Altra is
  96 kHz, the long block, the 24-bit four, the tone past 44.1 kHz, where
  the side channel's fit wins by an eighth of a bit and both codings are
  still made, and the orchestra and the Fantasy; on the EPYC only the 96
  kHz tone, the fixed path being cheaper on the Altra against this
  encoder's work than on either x86 machine.
- **On arm64 the encoder stands where it does on x86 against libFLAC's
  default level, and further from level 0.** The same run on the
  two-core Ampere Altra, in the section after the tables: the recordings
  are 0.25 to 0.42 of libFLAC's time a block where the EPYC is 0.24 to
  0.47, the made sounds 0.06 to 0.39 against 0.05 to 0.44 and silence
  0.06 to 0.08 against 0.05 to 0.06. This encoder's mean block is longer
  there than on the EPYC on every line, 1.24 to 1.46 of it, the
  recordings 1.24 to 1.43, while libFLAC's default level is 1.30 to 1.61
  of its EPYC time on the stereo recordings and 1.01 on the mono
  reading, and its level 0 0.84 to 1.20 on the recordings and 0.68 on
  the mono reading; this encoder's 99th-percentile block is 8 to 52 µs
  there against 6 to 49, and its worst 65 to 163 against 114 to 146, the
  EPYC's being its tick. Against level 0 it is 0.90 to 1.12 of its time
  on the 16-bit stereo recordings but the 1962 speech and white noise,
  0.64 and 0.63, where the EPYC is 0.44 to 0.98, and 0.69 on the mono
  reading where the EPYC is 0.34: level 0's fixed path is cheap on the
  Altra, 0.78 to 0.99 of its i5 time and 0.84 to 1.20 of its EPYC time,
  and this encoder's block, 0.83 to 0.94 of its i5 block on the Altra
  since the fused multiply-add, is 0.68 to 0.81 of its Altra block on
  the EPYC.
  Against its run before, the same encoder on the same machine the
  evening before, every line of every coder is within 2%, this encoder
  1.00 to 1.01, for the same bytes, so that machine's tables are the run
  before's but for the talk's line. The one thing in the encoder written
  with a machine in mind is the autocorrelation's multiply and add fused
  into one instruction where the machine has it, which the Altra has and
  the x86-64 baseline has not, though this EPYC does, and it is a
  question to the compiler, `cfg!`, not an instruction of its own. LLVM
  vectorizes the residual of a fit in four 16-bit lanes with NEON's
  widening multiply where SSE2 has its own, the autocorrelation in two
  f64 lanes, fused there, and the fixed predictors' residual sums in
  eight 16-bit lanes; the Rice coding and the bit writer are scalar on
  both. This encoder's bytes a block are the same on both machines to a
  tenth of a byte on every made sound and every recording, so a frame
  does not depend on the machine, the one rounding of the fused sum
  against the two differing in a last bit no fit is decided by;
  libFLAC's differ on the 24-bit tone, 2321 against 2315 at level 5 and
  2302 against 2502 at level 8, its arm64 build taking another
  arithmetic path through the ill-conditioned fit.

## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.33}$ | $\color{green}{0.31}$ | $\color{green}{0.39}$ | $\color{green}{0.25}$ | $\color{green}{0.25}$ | $\color{green}{0.37}$ |
| speech | $\color{green}{0.22}$ | $\color{green}{0.21}$ | $\color{green}{0.27}$ | $\color{green}{0.26}$ | $\color{green}{0.17}$ | $\color{green}{0.25}$ |
| tone | $\color{green}{0.35}$ | $\color{green}{0.33}$ | $\color{green}{0.44}$ | $\color{green}{0.19}$ | $\color{green}{0.34}$ | $\color{green}{0.40}$ |
| noise | $\color{green}{0.22}$ | $\color{green}{0.20}$ | $\color{green}{0.27}$ | $\color{green}{0.18}$ | $\color{green}{0.21}$ | $\color{green}{0.23}$ |
| silence | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.06}$ | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.05}$ |

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
| music | $\color{green}{0.62}$ | $\color{green}{0.56}$ | $\color{green}{0.89}$ | $\color{green}{0.37}$ | $\color{green}{0.50}$ | $\color{green}{0.73}$ |
| speech | $\color{green}{0.38}$ | $\color{green}{0.34}$ | $\color{green}{0.49}$ | $\color{green}{0.39}$ | $\color{green}{0.29}$ | $\color{green}{0.42}$ |
| tone | $\color{green}{0.72}$ | $\color{green}{0.62}$ | $\color{red}{1.01}$ | $\color{green}{0.29}$ | $\color{green}{0.71}$ | $\color{green}{0.79}$ |
| noise | $\color{green}{0.41}$ | $\color{green}{0.35}$ | $\color{green}{0.55}$ | $\color{green}{0.27}$ | $\color{green}{0.42}$ | $\color{green}{0.43}$ |
| silence | $\color{green}{0.08}$ | $\color{green}{0.07}$ | $\color{green}{0.11}$ | $\color{green}{0.07}$ | $\color{green}{0.08}$ | $\color{green}{0.07}$ |

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
| music | 34.1 | $\color{green}{11.2}$ | $\color{green}{0.33}$ | 0.06% | 1838 | 1838 | 1.00 | 47.9% |
| speech | 30.6 | $\color{green}{6.7}$ | $\color{green}{0.22}$ | 0.03% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 32.4 | $\color{green}{11.5}$ | $\color{green}{0.35}$ | 0.06% | 1008 | $\color{green}{997}$ | $\color{green}{0.99}$ | 26.0% |
| noise | 32.8 | $\color{green}{7.2}$ | $\color{green}{0.22}$ | 0.04% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 20.6 | $\color{green}{1.0}$ | $\color{green}{0.05}$ | 0.00% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 30.4 | $\color{green}{9.3}$ | $\color{green}{0.31}$ | 0.05% | 1702 | 1702 | 1.00 | 48.2% |
| speech | 26.9 | $\color{green}{5.7}$ | $\color{green}{0.21}$ | 0.03% | 1063 | $\color{red}{1064}$ | 1.00 | 30.1% |
| tone | 29.1 | $\color{green}{9.6}$ | $\color{green}{0.33}$ | 0.05% | 920 | 920 | 1.00 | 26.1% |
| noise | 29.4 | $\color{green}{6.0}$ | $\color{green}{0.20}$ | 0.03% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 19.7 | $\color{green}{0.9}$ | $\color{green}{0.05}$ | 0.00% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 53.7 | $\color{green}{21.1}$ | $\color{green}{0.39}$ | 0.11% | 3454 | 3454 | 1.00 | 45.0% |
| speech | 46.0 | $\color{green}{12.3}$ | $\color{green}{0.27}$ | 0.06% | 2226 | $\color{red}{2228}$ | 1.00 | 29.0% |
| tone | 50.3 | $\color{green}{22.1}$ | $\color{green}{0.44}$ | 0.11% | 1651 | $\color{green}{1481}$ | $\color{green}{0.90}$ | 19.3% |
| noise | 52.0 | $\color{green}{14.0}$ | $\color{green}{0.27}$ | 0.07% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 31.4 | $\color{green}{1.8}$ | $\color{green}{0.06}$ | 0.01% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 19.9 | $\color{green}{4.9}$ | $\color{green}{0.25}$ | 0.02% | 950 | 950 | 1.00 | 49.5% |
| speech | 20.3 | $\color{green}{5.3}$ | $\color{green}{0.26}$ | 0.03% | 1148 | 1148 | 1.00 | 59.8% |
| tone | 19.4 | $\color{green}{3.6}$ | $\color{green}{0.19}$ | 0.02% | 512 | 512 | 1.00 | 26.7% |
| noise | 20.1 | $\color{green}{3.6}$ | $\color{green}{0.18}$ | 0.02% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 15.4 | $\color{green}{0.8}$ | $\color{green}{0.05}$ | 0.00% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 31.7 | $\color{green}{8.0}$ | $\color{green}{0.25}$ | 0.04% | 456 | 456 | 1.00 | 23.7% |
| speech | 28.1 | $\color{green}{4.7}$ | $\color{green}{0.17}$ | 0.02% | 250 | $\color{red}{251}$ | 1.00 | 13.1% |
| tone | 32.6 | $\color{green}{11.2}$ | $\color{green}{0.34}$ | 0.06% | 476 | $\color{green}{470}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 32.1 | $\color{green}{6.8}$ | $\color{green}{0.21}$ | 0.03% | 1002 | $\color{red}{1006}$ | 1.00 | 52.4% |
| silence | 21.0 | $\color{green}{1.0}$ | $\color{green}{0.05}$ | 0.00% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 37.3 | $\color{green}{13.7}$ | $\color{green}{0.37}$ | 0.07% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 32.1 | $\color{green}{8.2}$ | $\color{green}{0.25}$ | 0.04% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 35.6 | $\color{green}{14.1}$ | $\color{green}{0.40}$ | 0.07% | 2321 | $\color{green}{2283}$ | $\color{green}{0.98}$ | 39.6% |
| noise | 36.0 | $\color{green}{8.3}$ | $\color{green}{0.23}$ | 0.04% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 21.3 | $\color{green}{1.0}$ | $\color{green}{0.05}$ | 0.00% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{green}{0.45}$ | 1.00 | $\color{green}{0.98}$ | $\color{green}{0.95}$ | $\color{green}{0.15}$ | 1.00 |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.39}$ | 1.00 | $\color{green}{0.87}$ | $\color{green}{0.87}$ | $\color{green}{0.13}$ | $\color{red}{1.01}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.35}$ | 1.00 | $\color{green}{0.68}$ | $\color{green}{0.78}$ | $\color{green}{0.12}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.41}$ | 1.00 | $\color{green}{0.78}$ | $\color{green}{0.96}$ | $\color{green}{0.13}$ | 1.00 |
| Mix at −13 LUFS, wide stereo | 44.1 kHz stereo 16-bit | $\color{green}{0.34}$ | 1.00 | $\color{green}{0.61}$ | $\color{green}{0.94}$ | $\color{green}{0.14}$ | 1.00 |
| Mix at −9 LUFS, loud master | 44.1 kHz stereo 16-bit | $\color{green}{0.33}$ | 1.00 | $\color{green}{0.59}$ | $\color{green}{0.94}$ | $\color{green}{0.14}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | $\color{green}{0.47}$ | 1.00 | $\color{green}{0.97}$ | $\color{green}{0.96}$ | $\color{green}{0.16}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.33}$ | 1.00 | $\color{green}{0.61}$ | $\color{green}{0.88}$ | $\color{green}{0.13}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.35}$ | 1.00 | $\color{green}{0.63}$ | $\color{green}{0.78}$ | $\color{green}{0.14}$ | 1.00 |
| Talk over soft music | 44.1 kHz stereo 16-bit | $\color{green}{0.34}$ | 1.00 | $\color{green}{0.61}$ | $\color{green}{0.87}$ | $\color{green}{0.14}$ | 1.00 |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.24}$ | 1.00 | $\color{green}{0.34}$ | $\color{green}{0.97}$ | $\color{green}{0.16}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.29}$ | 1.00 | $\color{green}{0.48}$ | $\color{green}{0.49}$ | $\color{green}{0.11}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.32}$ | 1.00 | $\color{green}{0.58}$ | $\color{green}{0.94}$ | $\color{green}{0.13}$ | 1.00 |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.45}$ | 1.00 | $\color{green}{0.85}$ | $\color{green}{0.99}$ | $\color{green}{0.16}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.24}$ | 1.00 | $\color{green}{0.44}$ | $\color{green}{0.99}$ | $\color{green}{0.09}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms. *The run before* is this
encoder's mean block in the run of the commit before this one, and *over it*
this run's over that.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | The run before, mean µs | Over it | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 60.2 | $\color{green}{27.2}$ | $\color{green}{0.45}$ | 41.2 | $\color{green}{0.66}$ | 81.0 | $\color{green}{42.4}$ | 176 | $\color{green}{140}$ | 0.14% |
| Chopin, Fantasy Op. 49, piano | 53.2 | $\color{green}{20.8}$ | $\color{green}{0.39}$ | 32.6 | $\color{green}{0.64}$ | 69.5 | $\color{green}{31.0}$ | 230 | $\color{green}{135}$ | 0.10% |
| Shostakovich 5, orchestra, live | 35.5 | $\color{green}{12.3}$ | $\color{green}{0.35}$ | 18.5 | $\color{green}{0.67}$ | 54.0 | $\color{green}{23.1}$ | 161 | $\color{green}{118}$ | 0.06% |
| Indie rock, 2022, mastered | 38.3 | $\color{green}{15.7}$ | $\color{green}{0.41}$ | 23.9 | $\color{green}{0.66}$ | 51.3 | $\color{green}{25.9}$ | 158 | $\color{green}{119}$ | 0.08% |
| Mix at −13 LUFS, wide stereo | 30.7 | $\color{green}{10.4}$ | $\color{green}{0.34}$ | 15.4 | $\color{green}{0.68}$ | 44.7 | $\color{green}{23.9}$ | 133 | $\color{green}{132}$ | 0.05% |
| Mix at −9 LUFS, loud master | 30.7 | $\color{green}{10.2}$ | $\color{green}{0.33}$ | 15.5 | $\color{green}{0.66}$ | 45.6 | $\color{green}{18.1}$ | 142 | $\color{green}{120}$ | 0.05% |
| Limehouse Blues, 1924 78 rpm | 63.3 | $\color{green}{29.9}$ | $\color{green}{0.47}$ | 44.4 | $\color{green}{0.67}$ | 83.5 | $\color{green}{48.8}$ | 178 | $\color{green}{146}$ | 0.15% |
| Prison work song, 1947 field recording | 30.1 | $\color{green}{10.1}$ | $\color{green}{0.33}$ | 15.3 | $\color{green}{0.66}$ | 39.9 | $\color{green}{18.8}$ | 138 | $\color{green}{120}$ | 0.05% |
| The Raven, read, produced | 30.2 | $\color{green}{10.7}$ | $\color{green}{0.35}$ | 15.9 | $\color{green}{0.67}$ | 43.8 | $\color{green}{21.3}$ | 149 | $\color{green}{123}$ | 0.05% |
| Talk over soft music | 30.2 | $\color{green}{10.2}$ | $\color{green}{0.34}$ |  |  | 41.9 | $\color{green}{17.5}$ | 131 | $\color{green}{116}$ | 0.05% |
| Leviticus 13, read | 19.2 | $\color{green}{4.6}$ | $\color{green}{0.24}$ | 7.0 | $\color{green}{0.66}$ | 31.3 | $\color{green}{6.3}$ | 140 | $\color{green}{114}$ | 0.02% |
| JFK at Rice, 1962 | 32.1 | $\color{green}{9.2}$ | $\color{green}{0.29}$ | 14.0 | $\color{green}{0.65}$ | 45.7 | $\color{green}{18.6}$ | 194 | $\color{green}{135}$ | 0.05% |
| Train station, field recording | 31.3 | $\color{green}{9.9}$ | $\color{green}{0.32}$ | 15.0 | $\color{green}{0.66}$ | 48.7 | $\color{green}{18.8}$ | 145 | $\color{green}{116}$ | 0.05% |
| Pink noise | 37.7 | $\color{green}{17.0}$ | $\color{green}{0.45}$ | 25.3 | $\color{green}{0.67}$ | 55.6 | $\color{green}{29.6}$ | 163 | $\color{green}{133}$ | 0.08% |
| White noise | 36.7 | $\color{green}{8.7}$ | $\color{green}{0.24}$ | 13.1 | $\color{green}{0.66}$ | 48.9 | $\color{green}{18.5}$ | 212 | $\color{green}{120}$ | 0.04% |

### The bytes, libFLAC at level 5

*The run before* is this encoder's bytes a block in the run of the commit
before this one, and *over it* this run's over that, to a ten-thousandth.

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | The run before, bytes | Over it | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5313}$ | 1.000 | 5313 | 1.0000 | 46.1% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2478}$ | $\color{red}{1.001}$ | 2478 | 1.0000 | 32.3% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1499}$ | $\color{red}{1.001}$ | 1499 | 1.0000 | 39.0% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{red}{4556}$ | 1.000 | 4556 | 1.0000 | 79.1% |
| Mix at −13 LUFS, wide stereo | 250 | 3528 | 2325 | $\color{red}{2327}$ | $\color{red}{1.001}$ | 2327 | 1.0000 | 66.0% |
| Mix at −9 LUFS, loud master | 225 | 3528 | 2543 | $\color{red}{2547}$ | $\color{red}{1.001}$ | 2547 | 1.0000 | 72.2% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | $\color{green}{8657}$ | 1.000 | 8657 | 1.0000 | 75.1% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | $\color{red}{1421}$ | $\color{red}{1.001}$ | 1421 | 1.0000 | 40.3% |
| The Raven, read, produced | 559 | 3528 | 1362 | $\color{red}{1363}$ | 1.000 | 1363 | 1.0000 | 38.6% |
| Talk over soft music | 181 | 3528 | 1618 | $\color{red}{1620}$ | $\color{red}{1.001}$ |  |  | 45.9% |
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
| music | $\color{green}{0.32}$ | $\color{green}{0.30}$ | $\color{green}{0.34}$ | $\color{green}{0.32}$ | $\color{green}{0.27}$ | $\color{green}{0.36}$ |
| speech | $\color{green}{0.24}$ | $\color{green}{0.23}$ | $\color{green}{0.25}$ | $\color{green}{0.33}$ | $\color{green}{0.19}$ | $\color{green}{0.24}$ |
| tone | $\color{green}{0.36}$ | $\color{green}{0.33}$ | $\color{green}{0.39}$ | $\color{green}{0.25}$ | $\color{green}{0.37}$ | $\color{green}{0.36}$ |
| noise | $\color{green}{0.23}$ | $\color{green}{0.21}$ | $\color{green}{0.25}$ | $\color{green}{0.24}$ | $\color{green}{0.22}$ | $\color{green}{0.24}$ |
| silence | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.08}$ | $\color{green}{0.07}$ | $\color{green}{0.06}$ |

### Time a block on arm64, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on arm64.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.98}$ | $\color{green}{0.88}$ | $\color{red}{1.12}$ | $\color{green}{0.72}$ | $\color{green}{0.84}$ | 1.00 |
| speech | $\color{green}{0.59}$ | $\color{green}{0.55}$ | $\color{green}{0.63}$ | $\color{green}{0.75}$ | $\color{green}{0.49}$ | $\color{green}{0.55}$ |
| tone | $\color{red}{1.11}$ | $\color{green}{0.98}$ | $\color{red}{1.27}$ | $\color{green}{0.59}$ | $\color{red}{1.15}$ | $\color{red}{1.02}$ |
| noise | $\color{green}{0.63}$ | $\color{green}{0.55}$ | $\color{green}{0.70}$ | $\color{green}{0.52}$ | $\color{green}{0.65}$ | $\color{green}{0.61}$ |
| silence | $\color{green}{0.16}$ | $\color{green}{0.16}$ | $\color{green}{0.17}$ | $\color{green}{0.17}$ | $\color{green}{0.16}$ | $\color{green}{0.14}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 |
|---|---:|---:|---:|---:|---:|---:|
| music | 34.1 | 47.0 | 11.2 | 15.0 | $\color{green}{0.33}$ | $\color{green}{0.32}$ |
| speech | 30.6 | 39.1 | 6.7 | 9.3 | $\color{green}{0.22}$ | $\color{green}{0.24}$ |
| tone | 32.4 | 44.8 | 11.5 | 16.2 | $\color{green}{0.35}$ | $\color{green}{0.36}$ |
| noise | 32.8 | 43.2 | 7.2 | 10.1 | $\color{green}{0.22}$ | $\color{green}{0.23}$ |
| silence | 20.6 | 23.7 | 1.0 | 1.7 | $\color{green}{0.05}$ | $\color{green}{0.07}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 | 99th, x86 | 99th, arm64 | Time vs 0, x86 | Time vs 0, arm64 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 60.2 | 93.6 | 27.2 | 34.7 | $\color{green}{0.45}$ | $\color{green}{0.37}$ | 42.4 | 51.3 | $\color{green}{0.98}$ | $\color{red}{1.06}$ |
| Chopin, Fantasy Op. 49, piano | 53.2 | 85.6 | 20.8 | 29.8 | $\color{green}{0.39}$ | $\color{green}{0.35}$ | 31.0 | 44.1 | $\color{green}{0.87}$ | $\color{red}{1.12}$ |
| Shostakovich 5, orchestra, live | 35.5 | 46.2 | 12.3 | 16.9 | $\color{green}{0.35}$ | $\color{green}{0.37}$ | 23.1 | 24.5 | $\color{green}{0.68}$ | $\color{red}{1.07}$ |
| Indie rock, 2022, mastered | 38.3 | 53.1 | 15.7 | 20.3 | $\color{green}{0.41}$ | $\color{green}{0.38}$ | 25.9 | 31.5 | $\color{green}{0.78}$ | $\color{red}{1.02}$ |
| Mix at −13 LUFS, wide stereo | 30.7 | 42.2 | 10.4 | 14.2 | $\color{green}{0.34}$ | $\color{green}{0.34}$ | 23.9 | 21.5 | $\color{green}{0.61}$ | $\color{green}{0.93}$ |
| Mix at −9 LUFS, loud master | 30.7 | 41.9 | 10.2 | 14.1 | $\color{green}{0.33}$ | $\color{green}{0.34}$ | 18.1 | 18.9 | $\color{green}{0.59}$ | $\color{green}{0.90}$ |
| Limehouse Blues, 1924 78 rpm | 63.3 | 96.7 | 29.9 | 37.3 | $\color{green}{0.47}$ | $\color{green}{0.39}$ | 48.8 | 52.2 | $\color{green}{0.97}$ | $\color{red}{1.02}$ |
| Prison work song, 1947 field recording | 30.1 | 41.0 | 10.1 | 13.8 | $\color{green}{0.33}$ | $\color{green}{0.34}$ | 18.8 | 18.8 | $\color{green}{0.61}$ | $\color{green}{0.96}$ |
| The Raven, read, produced | 30.2 | 41.4 | 10.7 | 14.2 | $\color{green}{0.35}$ | $\color{green}{0.34}$ | 21.3 | 20.7 | $\color{green}{0.63}$ | $\color{green}{0.96}$ |
| Talk over soft music | 30.2 | 41.1 | 10.2 | 14.2 | $\color{green}{0.34}$ | $\color{green}{0.35}$ | 17.5 | 20.3 | $\color{green}{0.61}$ | $\color{green}{0.96}$ |
| Leviticus 13, read | 19.2 | 19.4 | 4.6 | 6.3 | $\color{green}{0.24}$ | $\color{green}{0.32}$ | 6.3 | 7.8 | $\color{green}{0.34}$ | $\color{green}{0.69}$ |
| JFK at Rice, 1962 | 32.1 | 43.1 | 9.2 | 12.0 | $\color{green}{0.29}$ | $\color{green}{0.28}$ | 18.6 | 19.8 | $\color{green}{0.48}$ | $\color{green}{0.64}$ |
| Train station, field recording | 31.3 | 40.7 | 9.9 | 13.1 | $\color{green}{0.32}$ | $\color{green}{0.32}$ | 18.8 | 17.7 | $\color{green}{0.58}$ | $\color{green}{0.92}$ |
| Pink noise | 37.7 | 49.9 | 17.0 | 21.1 | $\color{green}{0.45}$ | $\color{green}{0.42}$ | 29.6 | 28.8 | $\color{green}{0.85}$ | $\color{red}{1.07}$ |
| White noise | 36.7 | 49.4 | 8.7 | 12.4 | $\color{green}{0.24}$ | $\color{green}{0.25}$ | 18.5 | 23.4 | $\color{green}{0.44}$ | $\color{green}{0.63}$ |

The bytes a block of this encoder are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 5 differ between the two runs on tone at 48 kHz stereo 24-bit (2321 against 2315).

The bytes a block of libFLAC at level 0 are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 8 differ between the two runs on tone at 48 kHz stereo 24-bit (2302 against 2502).

