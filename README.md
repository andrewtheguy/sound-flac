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
It is tuned for sound heard as it is made: fixed prediction of the best order,
the cheapest stereo pairing, Rice partitions that cost least and wasted bits
shifted out, never an LPC fit. On 20 ms blocks of 16-bit stereo at 48 kHz a
block of music takes about 24 µs of one core and is half its PCM, speech a
third, silence sixteen bytes; libFLAC at its default level took twice as long
for frames 7% smaller on music and the same on speech.

Use it by release tag:

```toml
sound-flac = { git = "https://github.com/andrewtheguy/sound-flac", tag = "v0.0.5" }
```
