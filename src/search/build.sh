#!/usr/bin/env bash
set -euo pipefail

echo "Compiling the backend"
SEARCH_DIR="$(dirname "$0")"
TARGET="wasm32-unknown-unknown"

cargo build --manifest-path "$SEARCH_DIR/Cargo.toml" --target $TARGET --release

cargo install ic-cdk-optimizer --version 0.3.1 --root "$SEARCH_DIR"/../../target
STATUS=$?

if [ "$STATUS" -eq "0" ]; then
      "$SEARCH_DIR"/../../target/bin/ic-cdk-optimizer \
      "$SEARCH_DIR/../../target/$TARGET/release/search.wasm" \
      -o "$SEARCH_DIR/../../target/$TARGET/release/search.wasm"

  true
else
  echo Could not install ic-cdk-optimizer.
  false
fi