#!/usr/bin/env bash
# The checks AGENTS.md asks for after a change, `cargo test` and clippy with
# warnings as errors, on whatever Unix machine runs this: a dev machine or
# the box ci/unix/remote.sh copies the tree to. It installs nothing and
# changes no machine state. The bench (tools/bench) is not built here: a run
# of it is made by hand on an idle host, as docs/benchmarks.md describes.
set -euo pipefail

# Invoked over ssh the working directory is the login user's home, not the
# checkout, so anchor to the repo root this script sits in. A non-interactive
# ssh shell also skips the profile that puts rustup's bin dir on PATH.
cd "$(dirname "$0")/../.."
export PATH="$HOME/.cargo/bin:$PATH"

step() {
    local name=$1; shift
    echo ''
    echo "== $name =="
    echo "   cargo $*"
    cargo "$@"
}

echo '== toolchain =='
uname -sm
rustc --version
cargo --version
cargo clippy --version
[ -n "${CARGO_TARGET_DIR:-}" ] && echo "   CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

step 'Test' test
step 'Clippy' clippy --all-targets -- -D warnings
