#!/bin/bash

TARGET="$1"

export RUSTFLAGS="$RUSTFLAGS -A dead_code"

CMD="cargo test --lib $TARGET -- --nocapture"
cargo-watch -s "$CMD"
