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
  beyond `thiserror`: `src/bits.rs` writes bits and the CRCs, `src/lpc.rs`
  fits a linear predictor to a channel, `src/subframe.rs` decides and writes
  one channel, `src/lib.rs` the frame and the stereo pair.
  Its tuning is for sound heard as it is made with little of a core: a change
  to it is measured first, against what it replaces, on 20 ms blocks, in time a
  block and bytes a block, and the module docs carry the numbers.
  `tools/bench` measures the crate against libFLAC as v0.0.4 linked it, on
  made sounds and on the recordings of `tools/samples.txt`, and
  `tools/tables.py` makes the tables of `docs/benchmarks.md` from its log,
  a section of its own from the log of the same run on an arm64 machine
  (`arm64=`), and the recordings tables' *run before* columns from the
  log of the commit before; the logs are JSON lines, a JSON object a
  line, kept in `docs/bench/` as `.jsonl`. The x86 run is made on an idle
  Linux amd64 host over ssh, never on the desktop the session runs on,
  and the arm64 run on the arm64 build box, both named in the gitignored
  `CLAUDE.local.md`; both are made again, and the doc's prose and
  `docs/plan.md` brought to the run, when the tuning changes. A third run,
  on the desktop's i5 under Windows natively (`windows=`), is made on
  demand, from WSL through the interop, and its section says which
  encoder it is of. The
  recordings live in the artifact store (`tools/samples.sh fetch`, once)
  and are copied to the gitignored `tmp/samples` for a run
  (`tools/samples.sh copy`); nothing in the repository holds one, and the
  store is read, never written by a run.
- After changes run `cargo test` and `cargo clippy --all-targets -- -D warnings`.
  The encoder gets an independent decoder in its tests, symphonia's, which reads
  every frame back bit for bit.
- Do not run `cargo fmt`. Errors are `thiserror`: every caller branches on them.
- A release is the version in `Cargo.toml` bumped and tagged `v<version>` on
  `main`; users pin the tag.
