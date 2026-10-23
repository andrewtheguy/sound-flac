# sound-flac

FLAC for a desktop's sound, as [wlshare](https://github.com/andrewtheguy/wlshare)
and the [zDesk](https://github.com/andrewtheguy/zdesk) gateway both code it.

- `Stream`: what the two ends agree on before any sound — the rate, the
  channels, the sample width and the block, the frames of samples in every FLAC
  frame.
- `Encoder`: one FLAC frame of every block, made the moment the block is whole
  and of that block alone, numbered zero: a decoder takes each frame as it
  comes, and a frame lost on the way costs its own samples and nothing after.
  The stream header, `STREAMINFO`, is never sent; `Stream::streaminfo` is the
  one a decoder builds to read the frames behind.

Encode only: the listeners decode with a browser's own FLAC decoder, and this
crate is the sender's.

The encoder is the crate's own, in plain Rust with no `unsafe`, so the build
compiles no C and nothing of FLAC is installed or loaded where the binary runs.
It is tuned for sound heard as it is made: the fixed predictor of the best
order on every channel, a linear predictor of up to order 8 fitted to each
and weighed, the cheapest stereo pairing chosen on both and settled on the
bits as coded, the fit skipped on noise, Rice partitions that cost least and
wasted bits shifted out. On 20 ms blocks of 16-bit stereo at 48 kHz a block
of music takes about 26 µs of one core and is 48% of its PCM, speech 15 µs
and 30%, silence 3 µs and sixteen bytes: the bytes libFLAC's default level
makes, in three fifths of its time on music and less than half on speech, and
on recordings 0.44 to 0.77 of its time for its bytes within 0.2%.
Every shape, made sound and
recording measured against libFLAC is in
[docs/benchmarks.md](docs/benchmarks.md), and what is taken up next in
[docs/plan.md](docs/plan.md).

Use it by release tag:

```toml
sound-flac = { git = "https://github.com/andrewtheguy/sound-flac", tag = "v0.0.5" }
```
