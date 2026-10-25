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
cp docs/bench/x86.jsonl before.jsonl                   # the run before, kept before this run's log replaces it; a commit from before the JSON lines holds x86.log, whose key=value lines are made JSON objects by hand first
cargo run --release --manifest-path tools/bench/Cargo.toml > docs/bench/x86.jsonl
python3 -I tools/tables.py docs/bench/x86.jsonl
python3 -I tools/tables.py docs/bench/x86.jsonl arm64=docs/bench/arm64.jsonl windows=docs/bench/windows.jsonl   # with the arm64 and Windows runs' logs, a section each
python3 -I tools/tables.py docs/bench/x86.jsonl arm64=docs/bench/arm64.jsonl windows=docs/bench/windows.jsonl --before before.jsonl
```

In the recordings tables below, *the run before* is the fifth of the night's
runs of 2026-10-24 on this machine, the stereo split folding the four channels
for their preparing, whose log `docs/bench/x86.jsonl` was until this run's
replaced it; this run, the night's sixth, is the same machine with one change
in the encoder, the autocorrelation summed as integers from the samples scaled
to 14 bits, and in double precision on a pure tone ([plan.md](plan.md),
measured apart first and then in the tree), so the *run before* columns
compare the two encoders like for like, on one machine. The x86 run moved to
this machine with the morning's run: every run before that one was made on the
desktop, a six-core Intel i5-8500T under WSL2, which was never quiet, the
session that started the run and the system under it taking a tenth of a time
from one run to the next, and libFLAC's own times with it, so that a ratio's
last hundredths were the machine's; from the morning's run on it is made on a
host that does nothing else, over ssh, and the morning's *run before* columns,
kept two commits before, set the two machines side by side. The fifteenth
recording, the third of the user's own, one voice talking over soft music, was
added with the morning's run. The days named in this document and in
[plan.md](plan.md) before the evening of 2026-10-23 are the count the sessions
that made the earlier runs kept, which ran ahead of the machine's clock: by
the commits, which that clock stamps in Pacific time, the run of v0.0.5 was
made at noon on 2026-10-23, the run of v0.0.6 that afternoon, the run of
v0.0.7 later that evening and the run of v0.0.8 later still, the morning's run
on 2026-10-24, which is the clock's own day and is named by it from there on,
the afternoon's run with the Rice parameter, the one after it with the fold,
the third with the fixed path left out, the night's first with the refresh
halved, the second with the partitions' search stopped past the best, the
third with the block whose side is one value carried, the fourth with the
residual's loops run at the orders the compiler lays out well, the fifth with
the stereo split folding the four channels for their preparing, and this run
after that.

They were run on 2026-10-24 by the machine's clock, at night, on a KVM virtual
machine with four virtual processors of QEMU's EPYC-Rome model at 2.8 GHz, two
cores with two threads each, the host's own chip not shown to it, under Debian
13, one thread, nothing else running on it, with rustc 1.98.1 at the x86-64
baseline (SSE2, no `target-cpu`; the machine has AVX2 and the fused multiply-
add and the build asks for neither), from the tree of this commit copied over,
the recordings in place from the morning: the encoder of v0.0.8 with the Rice
parameter's estimate corrected, the residual's fold made one pass, the fixed
path left out where the fit keeps winning, the corrections refreshed every
other cycle, the partitions' search stopped three orders past the best, the
block whose side is one value carried, the fit's residual loops run at the
orders the compiler lays out well, the stereo split folding the four channels
for their preparing and the autocorrelation summed as integers from the
samples scaled to 14 bits, in double precision on a pure tone. Against the run
before this encoder's block is 0.82 of its on the private talk, 0.85 on the
Aria, the private loud master, the private wide mix, the prison song and the
mono reading, 0.87 on the Fantasy and the orchestra, 0.88 on the train
station, 0.90 on the 1924 record, the 1962 speech and the Raven, 0.91 on pink
noise, 0.93 on the indie record and 0.97 on white noise, whose fit is order 1;
on the made sounds 0.86 to 1.06, the 44.1 kHz music the 0.86 and the 24-bit
tone the 1.06, the tones 1.00 to 1.06, summed in double precision as before
and twice on the first block; libFLAC's level 5 is 0.94 to 1.07 of the run
before's, its level 0 0.95 to 1.07 and its level 8 0.96 to 1.05; the bytes are
the run before's on every line of libFLAC's three levels and on every line of
this encoder's but the indie record's, 0.10% more, the Fantasy's, 0.012%, the
train station's, 0.008%, the Aria's, 0.004%, and six made sounds' within a
fifth of a byte a block. By the machine's own counters, which the virtual
machine shows, the change is 18% fewer instructions on the Fantasy, the train
station, the prison song, the Raven, the private talk and the private wide
mix, 17% on the private loud master, 16% on the orchestra and the mono
reading, 15% on the Aria, 14% on the 1924 record, the indie record and pink
noise, 12% on the 1962 speech and 3% on white noise.
The Windows run of the last section is the night's third there, made after the
arm64 run from the tree one commit past this one, the encoder of the night's
sixth x86 run with a channel coded alone coded once a block
([plan.md](plan.md), under *Measured and taken*: the same bytes on every line
but the made 8-bit speech's tenth of a byte, and no table moved), on the
desktop, a six-core Intel i5-8500T under Windows 11, natively, with rustc 1.93
MSVC at the x86-64 baseline and libFLAC from its sys crate's Windows build,
the machine idle but for the WSL session the run was started from: the tree
copied to WSL's staging area by the devtools driver, the recordings copied
from the store there, both copied on to the Windows side, the bench built and
run there from a Windows shell through the WSL interop, two minutes after the
copy, and its log brought back as the third log of `tools/tables.py`. The run
was made three times that night, each with the machine quieter, and the third
is the one kept: this encoder is 0.95 to 1.02 of the second on the recordings
and 0.93 to 1.07 on the made sounds, libFLAC's level 5 0.97 to 1.03 and 0.93
to 1.06, its level 0 0.93 to 0.96 and its level 8 1.02 to 1.09 on the
recordings, and the first was within 4% of the second, so the desktop repeats
a time within a twentieth when it is left alone; the Fantasy's worst block,
397 µs in the second and the machine's, is 153 in the third. The run had been
retired that morning, when the x86 run moved from this desktop's WSL to the
EPYC, as a second reading of one machine; it is back as a third machine, the
i5 being a different chip from the EPYC, and is made on demand, not with every
change, so its section can be of an earlier encoder than the tables above and
this paragraph says which. Against the EPYC this encoder's block is 1.38 to
1.59 of its on the recordings and 1.20 to 1.56 on the made sounds, libFLAC's
level 5 1.12 to 1.32 of its on the stereo recordings and 1.10 on the mono
reading, its level 0 0.99 to 1.30 on the recordings and its level 8 1.05 to
1.48; so this encoder is 0.22 to 0.42 of libFLAC's time a block on the
recordings there, where the EPYC has 0.20 to 0.39, and 0.40 to 0.88 of its
level 0's, where the EPYC has 0.29 to 0.80, the 96 kHz tone alone over level
0, at 1.09, where the EPYC has it at 0.89. Against the last Windows run before
the round, the encoder of v0.0.7 on 2026-10-23, this encoder is 0.61 to 0.85
of its time on the recordings, the 1962 speech the 0.61 and the train station
the 0.85, and 0.50 to 0.96 on the made sounds, while libFLAC's level 5 is 0.85
to 1.02 of its; the bytes are the x86 run's on every line of this encoder but
that tenth of a byte, and libFLAC's differ on the 24-bit tone at levels 5 and
8, its Windows build taking another path through the ill-conditioned fit, as
its arm64 build does.
A time ratio within a twentieth of 1.00 is on the line; a bytes ratio is
exact. A worst block is one block of thousands and the machine's as much as
the encoder's, even as the least over passes, so it is read as an order of
magnitude; on this machine it is 114 to 145 µs on every recording but the
private talk's, 38, and the train station's, 84, for this encoder, the mono
reading's 30 times its mean block, which is the virtual machine's tick and not
the sound, where libFLAC's is 141 to 236. The bench also logs `worst_at`, the
block that was slowest at its least over the passes, and dumps every block's
least time and bytes when asked (`BLOCKS=dir`), which is how the late blocks
below were read, in the morning's run; every pass starts from an encoder that
has seen no block, so that what the encoder carries from block to block, the
fourth-block choice, falls on the same blocks in every pass and the least over
the passes is the block's own.

The arm64 run of the section after the recordings is the night's second there,
made from the tree one commit past this one, the encoder of the night's sixth
x86 run with a channel coded alone coded once a block, at the user's call
after that fix; the night's first there, of this commit's tree, was made when
the round of nine changes was called ready, and the second is within 1.5% of
it on every line of every coder, so the second is the one kept. The machine is
a two-core Ampere Altra (Neoverse N1) in a Debian 13 LXC container, one
thread, the machine idle, with rustc 1.98.1 at the aarch64 baseline, whose
NEON the compiler uses on its own and whose fused multiply-add the exact
autocorrelation asks for: the tree copied there, the recordings in place from
the morning, the bench run once the load had fallen under 0.3 for the file
system's compression of what was copied, six minutes, and its log brought back
as the second log of `tools/tables.py`. A run started while that compression
is still going reads two to three times slow on the made sounds, so the wait
is part of the run. The run is held there at the user's ask while a round of
changes is measured on the x86 machine alone, so that box, which builds other
things, is not slowed by a run a change; these are the round's two runs.
Against the morning's run on it, the encoder of v0.0.8 before the round, this
encoder's block is 0.78 to 0.97 of its on the recordings, the 1962 speech the
0.78, the Fantasy, pink noise and the Aria 0.92, the orchestra, the indie
record and the private talk 0.93, the Raven, the prison song and the private
wide mix 0.94, white noise 0.95, the private loud master, the mono reading and
the train station 0.96 and the 1924 record 0.97, and 0.81 to 0.99 on the made
sounds, the 8-bit speech the 0.81, while libFLAC's three levels are within 2%
of the morning's on every line, so the move is the round's; the bytes are the
x86 run's on every line of this encoder but the made 8-bit speech's tenth of a
byte, and of libFLAC's but its two tone cells at 24 bits. On the recordings
this encoder is 0.22 to 0.39 of libFLAC's time there, the 1962 speech the 0.22
and pink noise the 0.39, and against its level 0 0.50 to 1.02, the Fantasy the
1.02, the orchestra 1.00 and the Aria and the 1924 record 0.99, where the
morning had 0.64 to 1.12. This encoder's block is 1.45 to 1.65 of its EPYC
block on every recording, the Altra's double-precision sums and fused
multiply-add having bought it less than the EPYC's integer ones, libFLAC's
level 5 1.30 to 1.59 of its on the stereo recordings and 0.99 on the mono
reading, its level 0 0.84 to 1.20 on the recordings and 0.70 on the mono
reading, and its level 8 1.24 to 1.85.

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
tables carry their numbers, each labelled *Private* with its file stem and
what it sounds like, so that the three are found at a glance, and the
repository nothing else of them.

| Recording | Item | Shape | Seconds | What it is |
|---|---|---|---:|---|
| Bach, Goldberg Aria, piano | `OpenGoldbergVariations` | 96 kHz stereo 24-bit | 299 | Kimiko Ishizaka's studio recording, CC0: a quiet piano, long decays |
| Chopin, Fantasy Op. 49, piano | `musopen-chopin-complete-works-flac` | 96 kHz stereo 16-bit | 738 | Musopen's recording, CC0: piano from quiet to loud |
| Shostakovich 5, orchestra, live | `dso20060204old` | 48 kHz stereo 16-bit | 324 | a concert recording, one movement, the hall in it |
| Indie rock, 2022, mastered | `1.-golf-club-stay-awhile` | 48 kHz stereo 24-bit | 224 | Golf Club's "Stay Awhile", CC0 by the band: guitars, drums and voices in a loud modern master, the one popular record |
| Private Track10, mix at −13 LUFS, wide stereo | the user's own | 44.1 kHz stereo 16-bit | 250 | a studio mix integrated at −12.6 LUFS over a 6.4 LU range, peaks at full scale, the side channel 8 dB under the mix: 66% of its PCM |
| Private Track_1, mix at −9 LUFS, loud master | the user's own | 44.1 kHz stereo 16-bit | 225 | a louder master, −9.2 LUFS over 4.6 LU, peaks at full scale, the side channel 6.6 dB under the mix: 72% of its PCM |
| Limehouse Blues, 1924 78 rpm | `TheColumbiansCollection1924-1929DirectedByBenSelvin` | 96 kHz stereo 24-bit | 197 | a 78 rpm transfer: dance band under surface noise |
| Prison work song, 1947 field recording | `negroprisonsongs` | 44.1 kHz stereo 16-bit | 252 | Lomax's Parchman recording: voices and axes outdoors |
| The Raven, read, produced | `TheRavenByEdgarAllanPoe` | 44.1 kHz stereo 16-bit | 559 | a radio theatre reading with music and effects |
| Private Track11, talk over soft music | the user's own | 44.1 kHz stereo 16-bit | 181 | one voice speaking with a little music under it, −17.5 LUFS over 5.9 LU, the side channel 10.8 dB under the mix: 45% of its PCM |
| Leviticus 13, read | `LeviticusKJV` | 44.1 kHz mono 16-bit | 725 | one voice reading, a room's quiet between verses |
| JFK at Rice, 1962 | `NasaAudioHighlightReels` | 48 kHz stereo 24-bit | 836 | a speech through a public-address system, a crowd |
| Train station, field recording | `aporee_53029_60598` | 44.1 kHz stereo 16-bit | 141 | a passenger train arriving, the station's ambience |
| Pink noise | `TenMinutesOfWhiteNoisePinkNoiseAndBrownianNoise` | 48 kHz stereo 24-bit | 600 | noise falling 3 dB an octave, predictable a little |
| White noise | `TenMinutesOfWhiteNoisePinkNoiseAndBrownianNoise` | 48 kHz stereo 24-bit | 600 | nothing to predict: the most a stream can cost |

## What the tables say

- **Every recording and every made sound takes less than half of what
  libFLAC at its default level takes of a core, most of them about a
  third or less.** The recordings are 0.20 to 0.39 of its time a block:
  16-bit stereo 0.25 to 0.31, mono 0.20, 24-bit 0.20 to 0.39, the 1962
  speech the 0.20, white noise 0.21, the indie record 0.34, pink noise
  and the Aria 0.35 and the 1924 record 0.39. A block of a recording is
  0.02% to 0.14% of one core. The made sounds are 0.04 to 0.38: music
  0.21 to 0.32, speech 0.13 to 0.21, the tone 0.17 to 0.38, noise 0.16
  to 0.23, silence 0.04 to 0.06. The run before, the night's fifth on
  this machine, the same encoder but for the autocorrelation summed as
  integers, had them at 0.21 to 0.43 and 0.04 to 0.39, over this run by
  a twentieth or more on every recording but white noise, the change,
  whose fit is order 1, and on the made music and speech, 0.24 to 0.36
  and 0.13 to 0.24 where they are 0.21 to 0.32 and 0.13 to 0.21, the
  tones the same, summed as before; the run before it, the night's
  fourth, the same encoder but for the stereo split folding the four
  channels for their preparing, at 0.22 to 0.43 and 0.05 to 0.38, within
  a twentieth of the night's fifth on every line, the change being a
  hundredth of a stereo block; the run before it, the night's third, the
  same encoder but for the residual's loops run at the orders the
  compiler lays out well, at 0.22 to 0.45 and 0.05 to 0.39, within a
  twentieth of the night's fourth on every line but pink noise's, 0.38
  where it was 0.43, the change, whose fits are the orders the split
  loop had run scalar; the run before it, the night's second, the same
  encoder but for the block whose side is one value carried, at 0.22 to
  0.44 and 0.05 to 0.39, within a twentieth of the night's third on
  every line but the 1962 speech's, 0.22 where it was 0.27, and the made
  stereo speech's, 0.13 to 0.23 where they were 0.16 to 0.26, the
  change, both one signal on their two channels; the run before it, the
  night's first, the same encoder but for the partitions' search stopped
  past the best, at 0.22 to 0.44 and 0.04 to 0.42, within a twentieth of
  the night's second on every line but the made 96 kHz music's, tone's
  and noise's, 0.36, 0.38 and 0.23 where they were 0.40, 0.42 and 0.26,
  the change, whose six partitionings at 1920 frames are the most of any
  shape; the run before it, the afternoon's third, the same encoder but
  for the corrections refreshed every other cycle, at 0.23 to 0.46 and
  0.05 to 0.42, the one before that, the afternoon's second, the same
  encoder but for the fixed path left out, at 0.23 to 0.46 and 0.05 to
  0.42, the one before that, the afternoon's first, the same encoder but
  for the fold, at 0.23 to 0.46 and 0.05 to 0.42, the morning's, the
  same encoder but for the Rice parameter, at 0.24 to 0.47 and 0.05 to
  0.44; the run before it, the same encoder on the desktop i5 late the
  evening of 2026-10-23, at 0.29 to 0.55 and 0.05 to 0.50, the run of
  v0.0.7 before it that evening at 0.28 to 0.54 and 0.23 to 0.50, the
  run before it, the same encoder as v0.0.7's, at 0.30 to 0.57 and 0.25
  to 0.53, the run of v0.0.6 before it, that afternoon, at 0.28 to 0.52
  and 0.23 to 0.52, the run of v0.0.5 before it at 0.34 to 0.58 and 0.27
  to 0.55, the evening's before that 0.33 to 0.55 and 0.27 to 0.53, the
  afternoon's 0.31 to 0.59 and 0.25 to 0.49, the morning's 0.44 to 0.77
  and 0.40 to 0.74, for the same bytes within half a per cent but the
  tone's, which fell, and two runs before the pianos', which fell by a
  tenth of a per cent. The move from the desktop's runs to this
  machine's is the machine's, not the encoder's: this encoder's block
  here is 0.64 to 0.68 of its i5 block on the recordings and 0.60 to
  0.67 on the made sounds but the silences, and libFLAC's default level
  0.74 to 0.84 on the stereo recordings, 0.90 on the mono reading and
  0.71 to 0.90 on the made sounds, so the ratios read a fifth better
  here for the same work. On arm64, the night's second run there, they
  are 0.22 to 0.39 and 0.06 to 0.37; on the desktop under Windows, the
  night's third run there, 0.22 to 0.42 and 0.05 to 0.45.
- **The bytes are libFLAC's at its default level, within 0.2%, and on a
  pure tone under them.** Every recording is 1.000 but the Fantasy, the
  prison song, the orchestra, the indie record, the two mixes and the
  talk at 1.001, a byte to seven a block, which is what fitting only the
  carried pair between full choices and coding it the fit first cost
  them, and the indie record the autocorrelation's scaling besides (the
  measurements are in [plan.md](plan.md)); before the pair was chosen on
  the fit the train station was 1.006 and the Fantasy 1.004, and before
  it was chosen on the four channels' counts the Fantasy was 1.002 and
  the Aria 1.001. Against the run before every line of the three libFLAC
  coders comes to the same bytes, and this encoder's the same on every
  line but ten: the indie record 4.7 bytes a block over of 4557, 0.10%,
  one loud, resonant passage at 24 bits, 400 blocks of 11 194, whose fit
  feels the autocorrelation's 14-bit scaling, the Fantasy three tenths
  of a byte, the Aria two and the train station one, and five made
  musics and the 44.1 kHz speech a tenth or two either way, the mono
  music the two under. The run before's encoder, against the one before
  it, had brought every line to the same bytes, the stereo split folding
  the four channels being to how they are prepared and not to what is
  chosen, as had the one before it, the residual's loops run at other
  orders; the one before that had the three made stereo speeches a tenth
  of a byte over, the block whose side is one value carried; the one
  before that the 1924 record 2.7 bytes a block over of 8657, 0.03%, and
  the made 8-bit speech a tenth of a byte of 251, the partitions' search
  stopped three orders past the best, which on those two now and then
  leaves a finer partitioning that would have won by a sliver after a
  longer plateau; the one before that had six lines a tenth of a byte
  over and the orchestra two tenths, each channel's correction brought
  up to date once in eight blocks rather than four; the one before that
  had seven lines a tenth either way, the fixed path left out while the
  fit keeps winning; the one before that had brought every line to the
  same bytes, the change being to how the residual is folded and summed
  and not to what is chosen, and the encoder before that, against the
  one before it, every line to the same bytes within half a byte a block
  but two: the 8-bit noise, 4.5 bytes a block fewer, 1.0001 of libFLAC's
  where it was 1.0046, and the 96 kHz tone, 35 fewer, 0.876 where it was
  0.90, the Rice parameter and the partitioning now chosen on the
  estimate less the fractions it drops, which on a quiet residual had
  favoured the smaller parameter and the finer partitioning by a bias
  and not by bits, the 8-bit noise's mid channel at a parameter under
  libFLAC's and its side in two to eight partitions where libFLAC had
  one; 8-bit speech 0.2 of a byte a block over and the recordings within
  0.02%, the wide mix the 0.02%. The encoder before those, against the
  one before it, had brought the Raven, the train station and the
  orchestra a tenth of a byte a block over, 0.01%, and the made 8-bit
  speech 0.3 of a byte, 0.12%: blocks where the fit, skipped after two
  losses by an eighth of a bit, would have won. The encoder before that,
  against the afternoon of 2026-10-23's, had brought every recording to
  the same bytes or fewer, the Fantasy 0.9983, the Aria 0.9989, the
  train station 0.9996, the Raven and the prison song 0.9999, which is
  the pair chosen on the counts where the estimates could not tell left
  and right from mid and side, and every made sound to the same bytes
  but the pure tone at 96 kHz, 0.990 of the afternoon's. The tone is
  0.99 of libFLAC's at 48 kHz, 1.00 at 44.1 kHz where it was 1.02, 0.88
  at 96 kHz where it was 0.95 and then 0.90, 0.98 at 24 bits where it
  was 1.05 and 0.99 at 8 bits: a tone is predicted away by order 2 and
  what its frame then costs is the rounding of the coefficients, which
  the order choice now weighs exactly, from the autocorrelation, where
  before it was blind to it and the orders tied. The made speech, whose
  two channels are one signal, is 0.05% over libFLAC and 0.12% at 8
  bits, the skip's: its side channel is a constant, and a block with a
  constant channel is coded the fit first like any other where it had
  every channel coded both ways. Against libFLAC's smallest level, 8,
  which tries every order to 12 and every stereo pairing in six to
  fourteen times this encoder's time on this machine, the made sounds
  are 0.98 to 1.01, the tone at 96 kHz and 24 bits 0.98 and 0.99 where
  they were 1.07 and 1.06, and the recordings 1.000 to 1.009, the
  orchestra the 1.009.
- **The late block is still the fourth block, where the pair is chosen
  afresh, by a quarter to a third of a carried block.** Each block's
  least time over the passes, each pass from an encoder that has seen no
  block, read by the block's place in the cycle of four, in a run on
  this machine after the morning's tables, of that run's encoder, with
  every block dumped (`BLOCKS=dir`), which the three changes since do
  not move, the first two spread over every block alike and the third
  taking the fixed path off carried blocks: the fourth block is 14.3 µs
  against 10.9 for a carried block on the orchestra, where the run
  before had 21.4 against 16.5 on the i5; 31.3 against 23.4 on the Aria,
  from 48.6 against 36.6; 24.4 against 19.3 on the Fantasy, from 36.7
  against 29.6; 12.2 against 9.2 on the Raven, from 18.4 against 14.2;
  11.5 against 9.4 on the wide mix, from 17.3 against 14.3; on the made
  tone 15.1 against 9.7, from 22.1 against 14.7, the fit skipped on the
  channel where it loses, and on the made music, where the estimates
  settle the pair and the two other channels are not counted, 11.3
  against 10.2, from 16.8 against 15.4; every pair of them the i5's at
  0.64 to 0.68, the fourth block and the carried one moved alike by the
  machine. Where the estimates do not settle it, the fourth block has
  the two other channels' residuals and folds over what it had, which is
  the Aria's and the Fantasy's rise, and against that the run before's
  choice coded 1.9 pairs on the orchestra's mean fourth block, which is
  why that block did not move there. The two blocks after every other
  choice, where each channel of the pair has both codings made once in
  eight blocks to bring its correction up to date, are about half a
  microsecond over a plain carried block at 16 bits, 11.4 and 10.6
  against 10.6 on the orchestra, and 2.4 on the Aria. This encoder's own
  99th-percentile block is set by the loud blocks as much as by the
  fourth, 1.5 to 2.1 times its mean on the stereo recordings and 1.6 on
  the mono reading, and its worst block on this machine is the virtual
  machine's tick: 119 to 159 µs on every recording but the talk's, 47,
  and the train station's, 64, as the least over the passes of each
  pass's worst, where no one block is slow in every pass, the slowest
  block of the orchestra at its least over the passes being 29 µs, the
  Aria's 43 and the Fantasy's 34, so the tick falls on another block
  each pass and a worst block past a hundred µs is the machine's. A
  single pass's 99th percentile is higher still for both encoders, which
  is scheduling and cache, not the sound. On the recordings this
  encoder's 99th-percentile block is 0.19 to 0.51 of libFLAC's, 6 to 41
  µs against 31 to 79, and its worst block 38 to 145 µs, at most 0.7% of
  the 20 ms the block is, where libFLAC's worst is 141 to 236. Where two
  channels are one signal, the 1962 speech and the made speech, the
  fourth block counted the three equivalent channels, each the same bits
  within the estimates' margin, 12.8 µs against 7.5 on that recording
  and 7.8 against 5.8 on the made speech, the blocks between coding one
  channel and a constant, until this run's encoder, which reads the side
  one value as the three being one signal and carries that block too:
  dumped again on this machine before the run, 12.5 against 6.8 became
  6.8 against 6.8 on the 1962 speech and 7.4 against 5.2 became 6.2
  against 4.8 to 6.2 on the made speech at 48 kHz, and in the run the
  1962 speech's mean block fell from 8.7 to 7.2 µs, 0.27 of libFLAC's to
  0.22, and its 99th-percentile block from 17 to 13, 0.33 of libFLAC's
  to 0.28.
- **The pair is chosen on the four channels' bits every fourth block,
  and carried between.** On the fourth block every channel is estimated
  from its autocorrelation, the mid's derived from left's, right's and
  side's so three passes serve four channels, and where one pair is
  estimated under every other by a quarter of a bit a sample it is
  taken. Otherwise every channel is counted: its fit's residual is taken
  and counted in one Rice partition, the fold and the finest sums the
  partition search needs anyway, a true count of the channel's bits
  within what the partitions chosen gain, and the fixed predictor of the
  order the autocorrelation chooses is counted the same way beside it
  where the fit is not clear; the pair whose two counts are least is
  taken, and its two channels are finished, the partitions searched and
  the fixed path run where the fit was not clear. That is libFLAC's
  choice, every pair by its bits, for a residual pass and a fold on the
  two channels not coded. The estimates are good at differences within a
  channel and poor between channels: on the piano recordings left and
  right against mid and side are within a twentieth of a bit a sample on
  99% of the fourth blocks and the estimates' pair is wrong on half of
  them, and on 8-bit music it is wrong on every block; chosen on the
  estimates and coded again where the coded pair's real bits said
  another was cheaper, as the run before did, 1.9 pairs were coded on
  the mean fourth block of the orchestra for a pair that stood on 85% of
  them, which is where the fourth block's time went. The estimates
  settle every fourth block of the made 16-bit and 24-bit music, half
  the made speech's, a third of the noise's and a quarter to a third of
  the mixes', and none of the piano's; past a fifth of a bit no count
  changed a choice on any sound measured, and an eighth was not safe at
  8 bits, where all of the 8% of the blocks the estimates separate by a
  tenth are wrong. On the three blocks between, the pair is coded again,
  each channel the fit first: where the fit is estimated to beat the
  best fixed predictor by an eighth of a bit a sample after a correction
  the channel learned the last time both were coded, the fit alone is
  coded and the order search and the fixed partitions are not run, and
  otherwise both are and compared; each channel of the pair has both
  coded once in two cycles whatever the estimate, on the two blocks
  after every other choice, which keeps the correction within eight
  blocks: left to the blocks where the fit was not clear, it went stale
  and cost the orchestra 0.4%, and made once a cycle, as it was until
  this run, it took 1.3% to 2.8% more instructions on every stereo
  recording for at most 0.013% fewer bytes, the orchestra's 0.2 of a
  byte a block. The correction is what the estimates miss, the window
  and the coefficients' rounding: on the orchestra at 16 bits the fit is
  coded alone on a third of the carried channels, on the Raven on three
  in five, on the mixes nine in ten, on the Aria at 24 bits nearly all,
  and on 8-bit music none, where the fixed coding wins every block and
  the fit never does, losing by a quarter of a bit a sample while
  estimated to win by a tenth. The two other channels are not looked at
  past a constant check and a bound from one pass over their
  differences, and the pair moves where the bound says another is
  certainly cheaper, 160 times in 12 000 blocks of the wide mix and once
  in the Raven's. A channel on its own, mono or more than two, is coded
  the fit first on every block. Where the fit is estimated not to win on
  a channel of fixed order 0, which is noise and a quiet desktop, it is
  skipped for seven blocks, and so it is where it has lost to the fixed
  coding, both coded, by an eighth of a bit a sample two blocks running,
  which is 8-bit music and one channel of the made tone, and took 8-bit
  music from 20 µs a block to 13 and the tone from 21 to 18 on the i5
  for the same bytes, which on this machine are 7.8 and 10.6. The fixed
  path is left out the same way where the fit, both coded, has won by a
  sixteenth of a bit a sample two blocks running, for seven blocks but
  the two in eight the schedule has both coded on: on the side channel
  of the 48 kHz tone the fit wins every block by 0.12 bits a sample,
  just under the margin after the correction, so both codings were made
  on every block, and on the speech recordings' and the mixes' carried
  channels the fit wins by under an eighth on a third to a half of the
  blocks both are coded on; with the rule both are coded on a sixth to a
  fifth fewer channel-blocks of those recordings and on the schedule's
  alone of the tone, which took the 48 kHz tone from 11.5 µs a block to
  10.6 on this machine and 2% of the instructions off the orchestra, the
  talk and the Raven, for a tenth of a byte a block either way on seven
  lines. The margin is the point: on the recordings the fit loses on a
  tenth to a quarter of the blocks where both are coded, but mostly
  within a sixteenth of a bit, and the next block's fit wins, so a loss
  under the eighth ends the run of losses; skipping on any loss cost the
  prison song 0.7% of its bytes, and on two losses past an eighth it and
  every recording are within 0.01%.
- **Where the time goes**, by perf's cpu-clock over this encoder alone
  on this machine, the afternoon after the morning's run and before the
  round's two changes, the bench built with line tables and the blocks
  15% longer under the sampler, each part as a share of the encoder's
  own samples (the fold and the finest sums, a tenth of the partitions'
  share and a half, are one pass since, 2% of the instructions fewer):
  on the orchestra at 16-bit stereo the two autocorrelations with their
  windowing are 19%, the fits from them 6%, the pair's residuals 14%,
  the partitions, the fold, the finest sums and the search over the
  orders, 20%, the fixed order search and the two bounds 9%, the Rice
  codes out 17%, the frame's CRC 5%, the stereo split with the header 4%
  and the four channels' preparing 4%. At 44.1 kHz, on the Raven and the
  talk, the partitions are 6% to 7%, since a block of 882 frames has one
  factor of two and two partitionings to try where 960 has six, and the
  autocorrelation is 22%, the residuals 16% to 18%, the writer 19% to
  20%. On the Fantasy at 96 kHz the residuals are 18% and the partitions
  17%; on the Aria at 24 bits the residual, which runs as two 16-bit
  passes, 27%, the writer 16% and the CRC 7%; on the 1924 record the
  residual 22%, the writer 21% and the CRC 10%; on pink noise the writer
  23%, the partitions 17% and the CRC 14%; on 24-bit noise the writer
  29% and the CRC 20%; on the 48 kHz tone the fixed path 15.5%, mostly
  the order search run on the side channel every block beside a fit that
  wins within the margin, which the round's third change leaves out but
  on the schedule's blocks. The i5's timers, in a copy of the tree on
  the runs before, put the writer at a quarter of a 16-bit block, 4.4 µs
  of 16.9, the two autocorrelations at 3.6, the fits 0.7, the residuals
  2.3, the partitions 1.9, the folds and bounds 1.2, the split 0.7 and
  the CRC 0.7, with a fixed coding of 1.7 for each channel whose fit is
  not clear; on this machine the writer is a sixth to a fifth, Zen 2
  shifting by a variable count in one µop where the i5 takes three, and
  the partitions and the CRC are larger shares. At 24 bits the writer
  takes three codes to a word at the parameters a 24-bit residual takes,
  where four to a word, falling to pairs, took 14 µs of the 1924
  record's 42 on the i5 against 11. A 17-bit side channel whose samples
  fit 16 bits takes the 16-bit pass. A block of noise, where order 0
  wins, takes one difference pass in the order search where it took
  four. What each part has left to give is in [plan.md](plan.md), under
  the open items. The partitions' share is the profile's before the
  search was stopped three orders past the best, which took 2% to 4% of
  the instructions off a 48 or 96 kHz recording's block and nothing off
  a 44.1 kHz one, whose 882 frames have two partitionings.
- **Against libFLAC's fastest level, 0, which fits no predictor and
  codes stereo as it comes**, this encoder takes less time on every
  recording on this machine: 0.55 to 0.83 of its time on the 16-bit
  stereo recordings, the Fantasy at 96 kHz the 0.83, the orchestra 0.63
  and the six at 44.1 kHz 0.55 to 0.57, 0.46 on the 1962 speech, 0.39 on
  white noise, 0.34 on the mono reading, and 0.77 to 0.92 on the four at
  24 bits with sound in them, the Aria the 0.92 this run, 0.95 the run
  before and 1.01 the one before that, and the 1924 record 0.91, for
  0.49 to 0.99 of its bytes: the three speeches 0.49, 0.78 and 0.87, the
  music 0.78 to 0.96, noise 0.99. On the made sounds it is 0.47 to 0.78
  on stereo music, 8-bit music the 0.47 and 96 kHz the 0.78, 0.58 to
  0.87 on the tone, 48 kHz 0.61 as the run before had it, where the one
  before had 0.66 and the morning 0.74, its fixed path left out on the
  side channel but for the schedule's blocks, two in eight, and 96 kHz
  the 0.87, which the morning's run had at 1.01 and the run before at
  0.95, 0.28 to 0.47 on speech, 0.33 to 0.48 on noise, 0.07 to 0.37 on
  mono and 0.07 to 0.11 on silence. The day's earlier runs had the same
  cells within a twentieth but the 48 kHz tone's; the run before it, on
  the i5, had the 16-bit stereo recordings at 0.85 to 1.09, the 1962
  speech 0.69, white noise 0.61, the mono reading 0.59 and the 24-bit
  four at 1.10 to 1.24, the made stereo music and tone at 0.76 to 1.11
  and 0.90 to 1.29: the column moved by a quarter with the machine, all
  of it the machine's, this encoder's block being 0.64 to 0.68 of its i5
  block and level 0's 0.80 to 0.96 of its on the stereo recordings, 1.14
  on the mono reading, and 0.82 to 1.22 on the made sounds, the mono
  shape the over, so the EPYC runs this encoder half as fast again as
  the i5 and level 0's fixed path about as fast. On arm64, the night's
  second run there, it reads 0.85 to 1.02 on the 16-bit stereo
  recordings, the Fantasy the 1.02 and the orchestra 1.00, the two mixes
  0.85 and 0.88, the talk 0.88, the 1962 speech and white noise 0.50 and
  0.60, the mono reading 0.65, and the 24-bit four 0.95 to 0.99, where
  the morning had 0.90 to 1.12, 0.64 and 0.63, 0.69 and 1.02 to 1.07; on
  the made sounds 0.82 to 1.03 on stereo music, 48 kHz 0.91 and 8-bit
  0.82, and 0.96 to 1.19 on the tone, 48 kHz 0.98 and 8-bit 1.10. What
  is over 1.00 on the Altra is 96 kHz, the long block, its music 1.03
  and its tone 1.19, the 8-bit tone 1.10, where the side channel's fit
  wins by an eighth of a bit and both codings are still made, and the
  Fantasy; on the EPYC nothing in this run, the Aria by a hundredth in
  the morning's and the 96 kHz tone in the one before it, the fixed path
  being cheaper on the Altra against this encoder's work than on either
  x86 machine.
- **On arm64 the encoder stands where it does on x86 against libFLAC's
  default level, and further from level 0.** The night's second run on
  the two-core Ampere Altra, at the user's call after the fix, within
  1.5% of the night's first on every line, in the section after the
  tables: the recordings are 0.22 to 0.39 of libFLAC's time a block
  where the EPYC is 0.20 to 0.39, the made sounds 0.06 to 0.37 against
  0.04 to 0.38 and silence 0.06 to 0.08 against 0.04 to 0.06. This
  encoder's mean block is longer there than on the EPYC on every line,
  1.25 to 1.83 of the night's, the recordings 1.45 to 1.65 where before
  the round they were 1.24 to 1.43, the round's integer sums having
  bought the EPYC more than the Altra's fused multiply-add bought it,
  while libFLAC's default level is 1.30 to 1.59 of its EPYC time on the
  stereo recordings and 0.99 on the mono reading, and its level 0 0.84
  to 1.20 on the recordings and 0.70 on the mono reading; this encoder's
  99th-percentile block is 8 to 53 µs there against 6 to 41, and its
  worst 43 to 96 against 38 to 145, the EPYC's being its tick. Against
  level 0 it is 0.85 to 1.02 of its time on the 16-bit stereo recordings
  but the 1962 speech and white noise, 0.50 and 0.60, where the EPYC is
  0.29 to 0.80, and 0.65 on the mono reading where the EPYC is 0.29:
  level 0's fixed path is cheap on the Altra, 0.78 to 0.99 of its i5
  time and 0.84 to 1.20 of its EPYC time, and this encoder's block is
  0.61 to 0.69 of its Altra block on the EPYC. Against the night's first
  run there, of the encoder before the fix, this encoder is 0.99 to 1.00
  of its block on every recording and made sound, libFLAC's three levels
  within 1.5% on every line, for the same bytes but the made 8-bit
  speech's tenth of a byte; against the morning's run, the encoder of
  v0.0.8 before the round, 0.78 to 0.97 on the recordings and 0.81 to
  0.99 on the made sounds, libFLAC's three levels within 2%. The one
  thing in the encoder written with a machine in mind is the multiply
  and add of the exact autocorrelation's sums, a pure tone's block's,
  fused into one instruction where the machine has it, which the Altra
  has and the x86-64 baseline has not, though this EPYC does, and it is
  a question to the compiler, `cfg!`, not an instruction of its own.
  LLVM vectorizes the residual of a fit in four 16-bit lanes with NEON's
  widening multiply where SSE2 has its own, the autocorrelation in two
  f64 lanes, fused there, and the fixed predictors' residual sums in
  eight 16-bit lanes; the Rice coding and the bit writer are scalar on
  both. This encoder's bytes a block were the same on both machines to a
  tenth of a byte on every made sound and every recording in the
  morning's two runs, so a frame does not depend on the machine, the one
  rounding of the fused sum against the two differing in a last bit no
  fit is decided by; against this run's they differ where the Rice
  change moved them, the 8-bit noise and the 96 kHz tone by whole bytes
  and a dozen lines by fractions of one, which the note under the tables
  lists and which is the encoder's, not the machine's; libFLAC's differ
  on the 24-bit tone, 2321 against 2315 at level 5 and 2302 against 2502
  at level 8, its arm64 build taking another arithmetic path through the
  ill-conditioned fit.
- **On the desktop under Windows the encoder stands a little further
  from libFLAC than on the EPYC, and about where it does on the Altra.**
  The night's third run on the i5-8500T, natively, in the last section:
  the recordings are 0.22 to 0.42 of libFLAC's time a block where the
  EPYC is 0.20 to 0.39 and the Altra 0.22 to 0.39, the 1924 record the
  0.42, the 1962 speech the 0.22; the 16-bit stereo recordings 0.32 to
  0.36 where the EPYC is 0.25 to 0.31, the three private tracks 0.32 to
  0.36 against 0.26 to 0.27, since the i5 runs this encoder 1.38 to 1.59
  times slower than the EPYC and libFLAC's default level only 1.12 to
  1.32; the made sounds 0.05 to 0.45, the 96 kHz tone the 0.45. Against
  level 0 the recordings are 0.40 to 0.88, the 1924 record the 0.88 and
  the Aria 0.84, under 1.00 on every one; of the made sounds only the 96
  kHz tone is over, 1.09. This encoder's 99th-percentile block is 7 to
  51 µs there against libFLAC's 34 to 114, and its worst 58 to 153, the
  153 the Fantasy's, where libFLAC's is 93 to 264. The bytes are the
  EPYC run's on every line of this encoder but the made 8-bit speech's
  tenth of a byte, the one block coded once; the night's three Windows
  runs agree within 7% on every line of this encoder and of libFLAC's
  level 5, and within 5% on the recordings.

## The made sounds, against libFLAC at level 5 as v0.0.4 ran it

### Time a block, this encoder over libFLAC

Under 1.00 is less of a core than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.26}$ | $\color{green}{0.24}$ | $\color{green}{0.31}$ | $\color{green}{0.21}$ | $\color{green}{0.22}$ | $\color{green}{0.32}$ |
| speech | $\color{green}{0.17}$ | $\color{green}{0.16}$ | $\color{green}{0.20}$ | $\color{green}{0.21}$ | $\color{green}{0.13}$ | $\color{green}{0.20}$ |
| tone | $\color{green}{0.30}$ | $\color{green}{0.32}$ | $\color{green}{0.38}$ | $\color{green}{0.17}$ | $\color{green}{0.29}$ | $\color{green}{0.37}$ |
| noise | $\color{green}{0.19}$ | $\color{green}{0.18}$ | $\color{green}{0.23}$ | $\color{green}{0.16}$ | $\color{green}{0.16}$ | $\color{green}{0.20}$ |
| silence | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.06}$ | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.04}$ |

### Bytes a block, this encoder over libFLAC

Under 1.00 is a smaller frame than libFLAC at its default level.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | $\color{green}{0.99}$ | 1.00 | $\color{green}{0.88}$ | 1.00 | $\color{green}{0.99}$ | $\color{green}{0.98}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, against libFLAC at its two ends

### Time a block, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, which fits no linear predictor and codes stereo as it comes.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.50}$ | $\color{green}{0.45}$ | $\color{green}{0.68}$ | $\color{green}{0.31}$ | $\color{green}{0.44}$ | $\color{green}{0.62}$ |
| speech | $\color{green}{0.29}$ | $\color{green}{0.26}$ | $\color{green}{0.36}$ | $\color{green}{0.32}$ | $\color{green}{0.22}$ | $\color{green}{0.31}$ |
| tone | $\color{green}{0.61}$ | $\color{green}{0.60}$ | $\color{green}{0.89}$ | $\color{green}{0.25}$ | $\color{green}{0.60}$ | $\color{green}{0.76}$ |
| noise | $\color{green}{0.36}$ | $\color{green}{0.32}$ | $\color{green}{0.47}$ | $\color{green}{0.24}$ | $\color{green}{0.33}$ | $\color{green}{0.42}$ |
| silence | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.10}$ | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.07}$ |

### Bytes a block, this encoder over libFLAC at level 8

Under 1.00 is a smaller frame than libFLAC's smallest level, which tries every order to 12 and every stereo pairing.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{red}{1.01}$ | $\color{red}{1.01}$ | 1.00 | $\color{red}{1.01}$ | 1.00 | 1.00 |
| speech | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| tone | $\color{red}{1.01}$ | 1.00 | $\color{green}{0.98}$ | 1.00 | $\color{green}{0.99}$ | $\color{green}{0.99}$ |
| noise | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |
| silence | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

## The made sounds, each shape apart

libFLAC is at level 5. The time is one block's, the least of five passes;
a block is 20 ms of sound, so a share of one core is that time over 20 ms.

### 48 kHz stereo 16-bit, 960 frames a block, 3840 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 33.9 | $\color{green}{8.7}$ | $\color{green}{0.26}$ | 0.04% | 1838 | 1838 | 1.00 | 47.9% |
| speech | 29.4 | $\color{green}{5.1}$ | $\color{green}{0.17}$ | 0.03% | 1151 | 1151 | 1.00 | 30.0% |
| tone | 32.6 | $\color{green}{9.7}$ | $\color{green}{0.30}$ | 0.05% | 1008 | $\color{green}{997}$ | $\color{green}{0.99}$ | 26.0% |
| noise | 33.3 | $\color{green}{6.4}$ | $\color{green}{0.19}$ | 0.03% | 2922 | 2922 | 1.00 | 76.1% |
| silence | 20.3 | $\color{green}{0.9}$ | $\color{green}{0.05}$ | 0.00% | 16 | 16 | 1.00 | 0.4% |

### 44.1 kHz stereo 16-bit, 882 frames a block, 3528 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 31.2 | $\color{green}{7.6}$ | $\color{green}{0.24}$ | 0.04% | 1702 | 1702 | 1.00 | 48.2% |
| speech | 26.5 | $\color{green}{4.3}$ | $\color{green}{0.16}$ | 0.02% | 1063 | $\color{red}{1064}$ | 1.00 | 30.2% |
| tone | 29.2 | $\color{green}{9.2}$ | $\color{green}{0.32}$ | 0.05% | 920 | 920 | 1.00 | 26.1% |
| noise | 29.3 | $\color{green}{5.4}$ | $\color{green}{0.18}$ | 0.03% | 2686 | 2686 | 1.00 | 76.1% |
| silence | 19.4 | $\color{green}{0.9}$ | $\color{green}{0.05}$ | 0.00% | 16 | 16 | 1.00 | 0.5% |

### 96 kHz stereo 16-bit, 1920 frames a block, 7680 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 55.7 | $\color{green}{17.1}$ | $\color{green}{0.31}$ | 0.09% | 3454 | 3454 | 1.00 | 45.0% |
| speech | 47.3 | $\color{green}{9.3}$ | $\color{green}{0.20}$ | 0.05% | 2226 | $\color{red}{2228}$ | 1.00 | 29.0% |
| tone | 51.3 | $\color{green}{19.7}$ | $\color{green}{0.38}$ | 0.10% | 1651 | $\color{green}{1447}$ | $\color{green}{0.88}$ | 18.8% |
| noise | 52.7 | $\color{green}{11.9}$ | $\color{green}{0.23}$ | 0.06% | 5830 | 5830 | 1.00 | 75.9% |
| silence | 30.2 | $\color{green}{1.7}$ | $\color{green}{0.06}$ | 0.01% | 16 | 16 | 1.00 | 0.2% |

### 48 kHz mono 16-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 20.4 | $\color{green}{4.2}$ | $\color{green}{0.21}$ | 0.02% | 950 | 950 | 1.00 | 49.5% |
| speech | 20.6 | $\color{green}{4.3}$ | $\color{green}{0.21}$ | 0.02% | 1148 | 1148 | 1.00 | 59.8% |
| tone | 19.7 | $\color{green}{3.4}$ | $\color{green}{0.17}$ | 0.02% | 512 | 512 | 1.00 | 26.7% |
| noise | 20.4 | $\color{green}{3.2}$ | $\color{green}{0.16}$ | 0.02% | 1468 | 1468 | 1.00 | 76.4% |
| silence | 15.9 | $\color{green}{0.8}$ | $\color{green}{0.05}$ | 0.00% | 13 | 13 | 1.00 | 0.7% |

### 48 kHz stereo 8-bit, 960 frames a block, 1920 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 31.8 | $\color{green}{7.0}$ | $\color{green}{0.22}$ | 0.04% | 456 | 456 | 1.00 | 23.7% |
| speech | 28.3 | $\color{green}{3.5}$ | $\color{green}{0.13}$ | 0.02% | 250 | $\color{red}{251}$ | 1.00 | 13.1% |
| tone | 31.3 | $\color{green}{9.2}$ | $\color{green}{0.29}$ | 0.05% | 476 | $\color{green}{470}$ | $\color{green}{0.99}$ | 24.5% |
| noise | 33.7 | $\color{green}{5.5}$ | $\color{green}{0.16}$ | 0.03% | 1002 | 1002 | 1.00 | 52.2% |
| silence | 21.1 | $\color{green}{1.0}$ | $\color{green}{0.05}$ | 0.00% | 14 | 14 | 1.00 | 0.7% |

### 48 kHz stereo 24-bit, 960 frames a block, 5760 bytes of PCM

| Sound | libFLAC 5, µs | This encoder, µs | Time ratio | Of one core | libFLAC 5, bytes | This encoder, bytes | Bytes ratio | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| music | 37.0 | $\color{green}{11.8}$ | $\color{green}{0.32}$ | 0.06% | 3763 | $\color{green}{3762}$ | 1.00 | 65.3% |
| speech | 31.8 | $\color{green}{6.2}$ | $\color{green}{0.20}$ | 0.03% | 2114 | 2114 | 1.00 | 36.7% |
| tone | 36.3 | $\color{green}{13.6}$ | $\color{green}{0.37}$ | 0.07% | 2321 | $\color{green}{2283}$ | $\color{green}{0.98}$ | 39.6% |
| noise | 39.7 | $\color{green}{8.1}$ | $\color{green}{0.20}$ | 0.04% | 4842 | 4842 | 1.00 | 84.1% |
| silence | 21.7 | $\color{green}{0.9}$ | $\color{green}{0.04}$ | 0.00% | 18 | 18 | 1.00 | 0.3% |

## The recordings

The recordings of `tools/samples.txt`, from archive.org, each coded
whole in blocks of 20 ms, the time a block the least of three passes' mean.

### Against libFLAC at levels 5, 0 and 8

This encoder over libFLAC: the time a block and the bytes a block, at each of
the three levels.

| Recording | Shape | Time, 5 | Bytes, 5 | Time, 0 | Bytes, 0 | Time, 8 | Bytes, 8 |
|---|---|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 96 kHz stereo 24-bit | $\color{green}{0.35}$ | 1.00 | $\color{green}{0.75}$ | $\color{green}{0.95}$ | $\color{green}{0.11}$ | 1.00 |
| Chopin, Fantasy Op. 49, piano | 96 kHz stereo 16-bit | $\color{green}{0.31}$ | 1.00 | $\color{green}{0.67}$ | $\color{green}{0.87}$ | $\color{green}{0.10}$ | $\color{red}{1.01}$ |
| Shostakovich 5, orchestra, live | 48 kHz stereo 16-bit | $\color{green}{0.28}$ | 1.00 | $\color{green}{0.54}$ | $\color{green}{0.78}$ | $\color{green}{0.10}$ | $\color{red}{1.01}$ |
| Indie rock, 2022, mastered | 48 kHz stereo 24-bit | $\color{green}{0.34}$ | 1.00 | $\color{green}{0.65}$ | $\color{green}{0.96}$ | $\color{green}{0.11}$ | 1.00 |
| Private Track10, mix at −13 LUFS, wide stereo | 44.1 kHz stereo 16-bit | $\color{green}{0.26}$ | 1.00 | $\color{green}{0.47}$ | $\color{green}{0.94}$ | $\color{green}{0.11}$ | 1.00 |
| Private Track_1, mix at −9 LUFS, loud master | 44.1 kHz stereo 16-bit | $\color{green}{0.27}$ | 1.00 | $\color{green}{0.47}$ | $\color{green}{0.94}$ | $\color{green}{0.11}$ | 1.00 |
| Limehouse Blues, 1924 78 rpm | 96 kHz stereo 24-bit | $\color{green}{0.39}$ | 1.00 | $\color{green}{0.80}$ | $\color{green}{0.96}$ | $\color{green}{0.14}$ | 1.00 |
| Prison work song, 1947 field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.25}$ | 1.00 | $\color{green}{0.46}$ | $\color{green}{0.88}$ | $\color{green}{0.10}$ | 1.00 |
| The Raven, read, produced | 44.1 kHz stereo 16-bit | $\color{green}{0.28}$ | 1.00 | $\color{green}{0.49}$ | $\color{green}{0.78}$ | $\color{green}{0.11}$ | 1.00 |
| Private Track11, talk over soft music | 44.1 kHz stereo 16-bit | $\color{green}{0.26}$ | 1.00 | $\color{green}{0.48}$ | $\color{green}{0.87}$ | $\color{green}{0.11}$ | 1.00 |
| Leviticus 13, read | 44.1 kHz mono 16-bit | $\color{green}{0.20}$ | 1.00 | $\color{green}{0.29}$ | $\color{green}{0.97}$ | $\color{green}{0.13}$ | 1.00 |
| JFK at Rice, 1962 | 48 kHz stereo 24-bit | $\color{green}{0.20}$ | 1.00 | $\color{green}{0.34}$ | $\color{green}{0.49}$ | $\color{green}{0.08}$ | 1.00 |
| Train station, field recording | 44.1 kHz stereo 16-bit | $\color{green}{0.25}$ | 1.00 | $\color{green}{0.45}$ | $\color{green}{0.94}$ | $\color{green}{0.10}$ | 1.00 |
| Pink noise | 48 kHz stereo 24-bit | $\color{green}{0.34}$ | 1.00 | $\color{green}{0.64}$ | $\color{green}{0.99}$ | $\color{green}{0.12}$ | 1.00 |
| White noise | 48 kHz stereo 24-bit | $\color{green}{0.21}$ | 1.00 | $\color{green}{0.38}$ | $\color{green}{0.99}$ | $\color{green}{0.08}$ | 1.00 |

### The block times, libFLAC at level 5

A block is 20 ms of sound. The 99th-percentile and worst blocks are the
least over the passes, so a pass the machine disturbed does not set them;
*of one core* is the mean block over 20 ms. *The run before* is this
encoder's mean block in the run of the commit before this one, and *over it*
this run's over that.

| Recording | libFLAC, mean µs | This encoder, mean µs | Ratio | The run before, mean µs | Over it | libFLAC, 99th | This encoder, 99th | libFLAC, worst | This encoder, worst | Of one core |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 58.8 | $\color{green}{20.4}$ | $\color{green}{0.35}$ | 24.0 | $\color{green}{0.85}$ | 74.9 | $\color{green}{33.7}$ | 172 | $\color{green}{145}$ | 0.10% |
| Chopin, Fantasy Op. 49, piano | 54.1 | $\color{green}{16.6}$ | $\color{green}{0.31}$ | 19.1 | $\color{green}{0.87}$ | 76.2 | $\color{green}{26.6}$ | 222 | $\color{green}{132}$ | 0.08% |
| Shostakovich 5, orchestra, live | 34.8 | $\color{green}{9.7}$ | $\color{green}{0.28}$ | 11.1 | $\color{green}{0.87}$ | 50.5 | $\color{green}{17.5}$ | 156 | $\color{green}{118}$ | 0.05% |
| Indie rock, 2022, mastered | 38.6 | $\color{green}{13.1}$ | $\color{green}{0.34}$ | 14.1 | $\color{green}{0.93}$ | 53.7 | $\color{green}{23.5}$ | 151 | $\color{green}{129}$ | 0.07% |
| Private Track10, mix at −13 LUFS, wide stereo | 31.1 | $\color{green}{8.1}$ | $\color{green}{0.26}$ | 9.6 | $\color{green}{0.85}$ | 44.5 | $\color{green}{14.7}$ | 141 | $\color{green}{118}$ | 0.04% |
| Private Track_1, mix at −9 LUFS, loud master | 30.9 | $\color{green}{8.2}$ | $\color{green}{0.27}$ | 9.7 | $\color{green}{0.85}$ | 44.4 | $\color{green}{15.2}$ | 143 | $\color{green}{114}$ | 0.04% |
| Limehouse Blues, 1924 78 rpm | 62.3 | $\color{green}{24.4}$ | $\color{green}{0.39}$ | 27.1 | $\color{green}{0.90}$ | 79.0 | $\color{green}{40.6}$ | 182 | $\color{green}{136}$ | 0.12% |
| Prison work song, 1947 field recording | 31.3 | $\color{green}{7.9}$ | $\color{green}{0.25}$ | 9.3 | $\color{green}{0.85}$ | 46.3 | $\color{green}{14.2}$ | 142 | $\color{green}{115}$ | 0.04% |
| The Raven, read, produced | 30.9 | $\color{green}{8.5}$ | $\color{green}{0.28}$ | 9.4 | $\color{green}{0.91}$ | 44.8 | $\color{green}{16.9}$ | 229 | $\color{green}{121}$ | 0.04% |
| Private Track11, talk over soft music | 30.6 | $\color{green}{8.1}$ | $\color{green}{0.26}$ | 9.9 | $\color{green}{0.82}$ | 42.1 | $\color{green}{14.4}$ | 146 | $\color{green}{38.4}$ | 0.04% |
| Leviticus 13, read | 19.5 | $\color{green}{3.8}$ | $\color{green}{0.20}$ | 4.5 | $\color{green}{0.85}$ | 30.5 | $\color{green}{5.8}$ | 198 | $\color{green}{115}$ | 0.02% |
| JFK at Rice, 1962 | 32.0 | $\color{green}{6.4}$ | $\color{green}{0.20}$ | 7.0 | $\color{green}{0.91}$ | 46.9 | $\color{green}{11.1}$ | 160 | $\color{green}{117}$ | 0.03% |
| Train station, field recording | 31.4 | $\color{green}{7.8}$ | $\color{green}{0.25}$ | 8.8 | $\color{green}{0.88}$ | 48.2 | $\color{green}{13.2}$ | 142 | $\color{green}{84.4}$ | 0.04% |
| Pink noise | 37.6 | $\color{green}{12.9}$ | $\color{green}{0.34}$ | 14.2 | $\color{green}{0.91}$ | 55.8 | $\color{green}{21.0}$ | 161 | $\color{green}{137}$ | 0.06% |
| White noise | 36.6 | $\color{green}{7.7}$ | $\color{green}{0.21}$ | 7.9 | $\color{green}{0.97}$ | 50.6 | $\color{green}{15.2}$ | 236 | $\color{green}{121}$ | 0.04% |

### The bytes, libFLAC at level 5

*The run before* is this encoder's bytes a block in the run of the commit
before this one, and *over it* this run's over that, to a ten-thousandth.

| Recording | Seconds | PCM a block | libFLAC, bytes | This encoder, bytes | Ratio | The run before, bytes | Over it | Of the PCM |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 299 | 11520 | 5312 | $\color{red}{5313}$ | 1.000 | 5313 | 1.0000 | 46.1% |
| Chopin, Fantasy Op. 49, piano | 738 | 7680 | 2476 | $\color{red}{2478}$ | $\color{red}{1.001}$ | 2478 | $\color{red}{1.0001}$ | 32.3% |
| Shostakovich 5, orchestra, live | 324 | 3840 | 1498 | $\color{red}{1500}$ | $\color{red}{1.001}$ | 1500 | 1.0000 | 39.1% |
| Indie rock, 2022, mastered | 223 | 5760 | 4555 | $\color{red}{4562}$ | $\color{red}{1.001}$ | 4557 | $\color{red}{1.0010}$ | 79.2% |
| Private Track10, mix at −13 LUFS, wide stereo | 250 | 3528 | 2325 | $\color{red}{2327}$ | $\color{red}{1.001}$ | 2327 | 1.0000 | 66.0% |
| Private Track_1, mix at −9 LUFS, loud master | 225 | 3528 | 2543 | $\color{red}{2547}$ | $\color{red}{1.001}$ | 2547 | 1.0000 | 72.2% |
| Limehouse Blues, 1924 78 rpm | 197 | 11520 | 8660 | 8660 | 1.000 | 8660 | 1.0000 | 75.2% |
| Prison work song, 1947 field recording | 252 | 3528 | 1420 | $\color{red}{1421}$ | $\color{red}{1.001}$ | 1421 | 1.0000 | 40.3% |
| The Raven, read, produced | 559 | 3528 | 1362 | $\color{red}{1363}$ | 1.000 | 1363 | 1.0000 | 38.6% |
| Private Track11, talk over soft music | 181 | 3528 | 1618 | $\color{red}{1620}$ | $\color{red}{1.001}$ | 1620 | 1.0000 | 45.9% |
| Leviticus 13, read | 725 | 1764 | 986 | $\color{red}{987}$ | 1.000 | 987 | 1.0000 | 55.9% |
| JFK at Rice, 1962 | 836 | 5760 | 1798 | $\color{green}{1797}$ | 1.000 | 1797 | 1.0000 | 31.2% |
| Train station, field recording | 141 | 3528 | 1272 | 1272 | 1.000 | 1272 | $\color{red}{1.0001}$ | 36.1% |
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
| music | $\color{green}{0.30}$ | $\color{green}{0.29}$ | $\color{green}{0.31}$ | $\color{green}{0.31}$ | $\color{green}{0.27}$ | $\color{green}{0.34}$ |
| speech | $\color{green}{0.20}$ | $\color{green}{0.20}$ | $\color{green}{0.21}$ | $\color{green}{0.32}$ | $\color{green}{0.15}$ | $\color{green}{0.21}$ |
| tone | $\color{green}{0.32}$ | $\color{green}{0.33}$ | $\color{green}{0.37}$ | $\color{green}{0.24}$ | $\color{green}{0.36}$ | $\color{green}{0.34}$ |
| noise | $\color{green}{0.22}$ | $\color{green}{0.21}$ | $\color{green}{0.24}$ | $\color{green}{0.23}$ | $\color{green}{0.20}$ | $\color{green}{0.23}$ |
| silence | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.07}$ | $\color{green}{0.08}$ | $\color{green}{0.07}$ | $\color{green}{0.06}$ |

### Time a block on arm64, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on arm64.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.91}$ | $\color{green}{0.85}$ | $\color{red}{1.03}$ | $\color{green}{0.70}$ | $\color{green}{0.82}$ | $\color{green}{0.94}$ |
| speech | $\color{green}{0.50}$ | $\color{green}{0.47}$ | $\color{green}{0.54}$ | $\color{green}{0.72}$ | $\color{green}{0.40}$ | $\color{green}{0.47}$ |
| tone | $\color{green}{0.98}$ | $\color{green}{0.96}$ | $\color{red}{1.19}$ | $\color{green}{0.57}$ | $\color{red}{1.10}$ | $\color{green}{0.97}$ |
| noise | $\color{green}{0.60}$ | $\color{green}{0.54}$ | $\color{green}{0.66}$ | $\color{green}{0.50}$ | $\color{green}{0.60}$ | $\color{green}{0.58}$ |
| silence | $\color{green}{0.15}$ | $\color{green}{0.15}$ | $\color{green}{0.17}$ | $\color{green}{0.17}$ | $\color{green}{0.15}$ | $\color{green}{0.13}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 |
|---|---:|---:|---:|---:|---:|---:|
| music | 33.9 | 47.1 | 8.7 | 14.1 | $\color{green}{0.26}$ | $\color{green}{0.30}$ |
| speech | 29.4 | 39.1 | 5.1 | 8.0 | $\color{green}{0.17}$ | $\color{green}{0.20}$ |
| tone | 32.6 | 45.1 | 9.7 | 14.3 | $\color{green}{0.30}$ | $\color{green}{0.32}$ |
| noise | 33.3 | 43.4 | 6.4 | 9.7 | $\color{green}{0.19}$ | $\color{green}{0.22}$ |
| silence | 20.3 | 23.7 | 0.9 | 1.6 | $\color{green}{0.05}$ | $\color{green}{0.07}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, x86 µs | libFLAC, arm64 µs | This encoder, x86 µs | This encoder, arm64 µs | Ratio, x86 | Ratio, arm64 | 99th, x86 | 99th, arm64 | Time vs 0, x86 | Time vs 0, arm64 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 58.8 | 93.4 | 20.4 | 32.1 | $\color{green}{0.35}$ | $\color{green}{0.34}$ | 33.7 | 46.6 | $\color{green}{0.75}$ | $\color{green}{0.99}$ |
| Chopin, Fantasy Op. 49, piano | 54.1 | 85.5 | 16.6 | 27.4 | $\color{green}{0.31}$ | $\color{green}{0.32}$ | 26.6 | 41.8 | $\color{green}{0.67}$ | $\color{red}{1.02}$ |
| Shostakovich 5, orchestra, live | 34.8 | 46.2 | 9.7 | 15.8 | $\color{green}{0.28}$ | $\color{green}{0.34}$ | 17.5 | 23.0 | $\color{green}{0.54}$ | 1.00 |
| Indie rock, 2022, mastered | 38.6 | 53.0 | 13.1 | 18.9 | $\color{green}{0.34}$ | $\color{green}{0.36}$ | 23.5 | 26.6 | $\color{green}{0.65}$ | $\color{green}{0.95}$ |
| Private Track10, mix at −13 LUFS, wide stereo | 31.1 | 42.2 | 8.1 | 13.4 | $\color{green}{0.26}$ | $\color{green}{0.32}$ | 14.7 | 18.4 | $\color{green}{0.47}$ | $\color{green}{0.87}$ |
| Private Track_1, mix at −9 LUFS, loud master | 30.9 | 42.0 | 8.2 | 13.5 | $\color{green}{0.27}$ | $\color{green}{0.32}$ | 15.2 | 17.9 | $\color{green}{0.47}$ | $\color{green}{0.85}$ |
| Limehouse Blues, 1924 78 rpm | 62.3 | 96.9 | 24.4 | 36.2 | $\color{green}{0.39}$ | $\color{green}{0.37}$ | 40.6 | 52.7 | $\color{green}{0.80}$ | $\color{green}{0.99}$ |
| Prison work song, 1947 field recording | 31.3 | 40.9 | 7.9 | 13.0 | $\color{green}{0.25}$ | $\color{green}{0.32}$ | 14.2 | 18.1 | $\color{green}{0.46}$ | $\color{green}{0.89}$ |
| The Raven, read, produced | 30.9 | 41.4 | 8.5 | 13.3 | $\color{green}{0.28}$ | $\color{green}{0.32}$ | 16.9 | 19.1 | $\color{green}{0.49}$ | $\color{green}{0.88}$ |
| Private Track11, talk over soft music | 30.6 | 41.3 | 8.1 | 13.3 | $\color{green}{0.26}$ | $\color{green}{0.32}$ | 14.4 | 18.6 | $\color{green}{0.48}$ | $\color{green}{0.88}$ |
| Leviticus 13, read | 19.5 | 19.4 | 3.8 | 6.0 | $\color{green}{0.20}$ | $\color{green}{0.31}$ | 5.8 | 7.5 | $\color{green}{0.29}$ | $\color{green}{0.65}$ |
| JFK at Rice, 1962 | 32.0 | 43.3 | 6.4 | 9.4 | $\color{green}{0.20}$ | $\color{green}{0.22}$ | 11.1 | 12.1 | $\color{green}{0.34}$ | $\color{green}{0.50}$ |
| Train station, field recording | 31.4 | 40.7 | 7.8 | 12.7 | $\color{green}{0.25}$ | $\color{green}{0.31}$ | 13.2 | 16.9 | $\color{green}{0.45}$ | $\color{green}{0.88}$ |
| Pink noise | 37.6 | 49.9 | 12.9 | 19.4 | $\color{green}{0.34}$ | $\color{green}{0.39}$ | 21.0 | 25.6 | $\color{green}{0.64}$ | $\color{green}{0.98}$ |
| White noise | 36.6 | 49.2 | 7.7 | 11.8 | $\color{green}{0.21}$ | $\color{green}{0.24}$ | 15.2 | 22.8 | $\color{green}{0.38}$ | $\color{green}{0.60}$ |

The bytes a block of this encoder differ between the two runs on speech at 48 kHz stereo 8-bit (251 against 251).

The bytes a block of libFLAC at level 5 differ between the two runs on tone at 48 kHz stereo 24-bit (2321 against 2315).

The bytes a block of libFLAC at level 0 are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 8 differ between the two runs on tone at 48 kHz stereo 24-bit (2302 against 2502).

## On Windows

The same run on the desktop, an Intel i5-8500T under Windows 11, natively,
beside the x86 run above, which is on an AMD EPYC Rome machine: this
encoder's time a block over libFLAC's on each. A ratio under 1.00 is less of
a core than libFLAC on that machine, and a block's µs are that machine's.
This run is made on demand, not with every change to the tuning as the two
above are, so it can be of an earlier encoder than theirs; the prose above
says which.

### Time a block on Windows, this encoder over libFLAC at level 5

Under 1.00 is less of a core than libFLAC at its default level, on Windows.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.29}$ | $\color{green}{0.30}$ | $\color{green}{0.34}$ | $\color{green}{0.27}$ | $\color{green}{0.26}$ | $\color{green}{0.34}$ |
| speech | $\color{green}{0.21}$ | $\color{green}{0.21}$ | $\color{green}{0.23}$ | $\color{green}{0.26}$ | $\color{green}{0.15}$ | $\color{green}{0.21}$ |
| tone | $\color{green}{0.35}$ | $\color{green}{0.39}$ | $\color{green}{0.45}$ | $\color{green}{0.22}$ | $\color{green}{0.36}$ | $\color{green}{0.41}$ |
| noise | $\color{green}{0.23}$ | $\color{green}{0.23}$ | $\color{green}{0.26}$ | $\color{green}{0.19}$ | $\color{green}{0.21}$ | $\color{green}{0.24}$ |
| silence | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.05}$ | $\color{green}{0.06}$ | $\color{green}{0.05}$ | $\color{green}{0.05}$ |

### Time a block on Windows, this encoder over libFLAC at level 0

Under 1.00 is less of a core than libFLAC's fastest level, on Windows.

| Sound | 48 kHz stereo 16-bit | 44.1 kHz stereo 16-bit | 96 kHz stereo 16-bit | 48 kHz mono 16-bit | 48 kHz stereo 8-bit | 48 kHz stereo 24-bit |
|---|---:|---:|---:|---:|---:|---:|
| music | $\color{green}{0.63}$ | $\color{green}{0.57}$ | $\color{green}{0.81}$ | $\color{green}{0.48}$ | $\color{green}{0.58}$ | $\color{green}{0.74}$ |
| speech | $\color{green}{0.38}$ | $\color{green}{0.33}$ | $\color{green}{0.42}$ | $\color{green}{0.47}$ | $\color{green}{0.29}$ | $\color{green}{0.38}$ |
| tone | $\color{green}{0.79}$ | $\color{green}{0.76}$ | $\color{red}{1.09}$ | $\color{green}{0.40}$ | $\color{green}{0.80}$ | $\color{green}{0.91}$ |
| noise | $\color{green}{0.45}$ | $\color{green}{0.41}$ | $\color{green}{0.55}$ | $\color{green}{0.34}$ | $\color{green}{0.45}$ | $\color{green}{0.48}$ |
| silence | $\color{green}{0.09}$ | $\color{green}{0.09}$ | $\color{green}{0.12}$ | $\color{green}{0.10}$ | $\color{green}{0.10}$ | $\color{green}{0.09}$ |

### The made sounds at 48 kHz stereo 16-bit, run by run

libFLAC is at level 5; the time is one block's, the least of five passes.

| Sound | libFLAC, x86 µs | libFLAC, Windows µs | This encoder, x86 µs | This encoder, Windows µs | Ratio, x86 | Ratio, Windows |
|---|---:|---:|---:|---:|---:|---:|
| music | 33.9 | 43.1 | 8.7 | 12.7 | $\color{green}{0.26}$ | $\color{green}{0.29}$ |
| speech | 29.4 | 35.7 | 5.1 | 7.6 | $\color{green}{0.17}$ | $\color{green}{0.21}$ |
| tone | 32.6 | 43.2 | 9.7 | 15.2 | $\color{green}{0.30}$ | $\color{green}{0.35}$ |
| noise | 33.3 | 42.1 | 6.4 | 9.8 | $\color{green}{0.19}$ | $\color{green}{0.23}$ |
| silence | 20.3 | 24.0 | 0.9 | 1.2 | $\color{green}{0.05}$ | $\color{green}{0.05}$ |

### The recordings, run by run

libFLAC is at level 5 but in the last two columns; the time is the mean
block, and the 99th-percentile block is this encoder's, the least over the passes.

| Recording | libFLAC, x86 µs | libFLAC, Windows µs | This encoder, x86 µs | This encoder, Windows µs | Ratio, x86 | Ratio, Windows | 99th, x86 | 99th, Windows | Time vs 0, x86 | Time vs 0, Windows |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Bach, Goldberg Aria, piano | 58.8 | 77.1 | 20.4 | 29.8 | $\color{green}{0.35}$ | $\color{green}{0.39}$ | 33.7 | 43.8 | $\color{green}{0.75}$ | $\color{green}{0.84}$ |
| Chopin, Fantasy Op. 49, piano | 54.1 | 68.9 | 16.6 | 24.8 | $\color{green}{0.31}$ | $\color{green}{0.36}$ | 26.6 | 38.5 | $\color{green}{0.67}$ | $\color{green}{0.82}$ |
| Shostakovich 5, orchestra, live | 34.8 | 42.0 | 9.7 | 14.8 | $\color{green}{0.28}$ | $\color{green}{0.35}$ | 17.5 | 22.3 | $\color{green}{0.54}$ | $\color{green}{0.76}$ |
| Indie rock, 2022, mastered | 38.6 | 48.6 | 13.1 | 18.1 | $\color{green}{0.34}$ | $\color{green}{0.37}$ | 23.5 | 25.6 | $\color{green}{0.65}$ | $\color{green}{0.79}$ |
| Private Track10, mix at −13 LUFS, wide stereo | 31.1 | 37.7 | 8.1 | 12.2 | $\color{green}{0.26}$ | $\color{green}{0.32}$ | 14.7 | 17.2 | $\color{green}{0.47}$ | $\color{green}{0.62}$ |
| Private Track_1, mix at −9 LUFS, loud master | 30.9 | 35.6 | 8.2 | 12.5 | $\color{green}{0.27}$ | $\color{green}{0.35}$ | 15.2 | 17.6 | $\color{green}{0.47}$ | $\color{green}{0.64}$ |
| Limehouse Blues, 1924 78 rpm | 62.3 | 82.2 | 24.4 | 34.3 | $\color{green}{0.39}$ | $\color{green}{0.42}$ | 40.6 | 51.4 | $\color{green}{0.80}$ | $\color{green}{0.88}$ |
| Prison work song, 1947 field recording | 31.3 | 35.1 | 7.9 | 12.5 | $\color{green}{0.25}$ | $\color{green}{0.36}$ | 14.2 | 18.1 | $\color{green}{0.46}$ | $\color{green}{0.68}$ |
| The Raven, read, produced | 30.9 | 35.1 | 8.5 | 12.6 | $\color{green}{0.28}$ | $\color{green}{0.36}$ | 16.9 | 20.3 | $\color{green}{0.49}$ | $\color{green}{0.66}$ |
| Private Track11, talk over soft music | 30.6 | 35.0 | 8.1 | 12.4 | $\color{green}{0.26}$ | $\color{green}{0.36}$ | 14.4 | 17.9 | $\color{green}{0.48}$ | $\color{green}{0.66}$ |
| Leviticus 13, read | 19.5 | 21.4 | 3.8 | 5.8 | $\color{green}{0.20}$ | $\color{green}{0.27}$ | 5.8 | 7.2 | $\color{green}{0.29}$ | $\color{green}{0.45}$ |
| JFK at Rice, 1962 | 32.0 | 39.5 | 6.4 | 8.9 | $\color{green}{0.20}$ | $\color{green}{0.22}$ | 11.1 | 11.7 | $\color{green}{0.34}$ | $\color{green}{0.40}$ |
| Train station, field recording | 31.4 | 35.1 | 7.8 | 12.3 | $\color{green}{0.25}$ | $\color{green}{0.35}$ | 13.2 | 16.8 | $\color{green}{0.45}$ | $\color{green}{0.67}$ |
| Pink noise | 37.6 | 46.1 | 12.9 | 18.2 | $\color{green}{0.34}$ | $\color{green}{0.39}$ | 21.0 | 26.2 | $\color{green}{0.64}$ | $\color{green}{0.80}$ |
| White noise | 36.6 | 45.6 | 7.7 | 11.1 | $\color{green}{0.21}$ | $\color{green}{0.24}$ | 15.2 | 23.4 | $\color{green}{0.38}$ | $\color{green}{0.48}$ |

The bytes a block of this encoder differ between the two runs on speech at 48 kHz stereo 8-bit (251 against 251).

The bytes a block of libFLAC at level 5 differ between the two runs on tone at 48 kHz stereo 24-bit (2321 against 2465).

The bytes a block of libFLAC at level 0 are the same in both runs, to a tenth of a byte, on every made sound and every recording.

The bytes a block of libFLAC at level 8 differ between the two runs on tone at 48 kHz stereo 24-bit (2302 against 2279).

