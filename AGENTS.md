# Repository instructions

- Strict no backward-compatibility or legacy paths no matter what.
- One crate, `sound-flac`: the one place a desktop's sound is made FLAC for the
  wlshare daemon and the zDesk gateway, which each pin it by a release tag of
  this repository. How a block becomes a frame and how the encoder is tuned
  change here and reach them as a pin bump.
  Encode only: nothing here decodes, the listeners having decoders of their own.
  Nothing about a wire belongs here: how a frame is framed in a message, how a
  stream's shape is agreed and what a sample's bytes are belong to each user.
- The encoder is the crate's own, plain Rust without `unsafe` or a dependency
  beyond `thiserror`: `src/bits.rs` writes bits and the CRCs, `src/subframe.rs`
  decides and writes one channel, `src/lib.rs` the frame and the stereo pair.
  Its tuning is for sound heard as it is made with little of a core: a change
  to it is measured first, against what it replaces, on 20 ms blocks, in time a
  block and bytes a block, and the module docs carry the numbers.
- After changes run `cargo test` and `cargo clippy --all-targets -- -D warnings`.
  The encoder gets an independent decoder in its tests, symphonia's, which reads
  every frame back bit for bit.
- Do not run `cargo fmt`. Errors are `thiserror`: every caller branches on them.
- A release is the version in `Cargo.toml` bumped and tagged `v<version>` on
  `main`; users pin the tag.
