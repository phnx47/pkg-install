#!/bin/sh

DIR="$(dirname "$0")"

if cargo "$@"; then
  [ -d "$DIR/target/debug" ] && cp -r "$DIR/config.json" "$DIR/target/debug"
  [ -d "$DIR/target/release" ] && cp -r "$DIR/config.json" "$DIR/target/release"
fi
