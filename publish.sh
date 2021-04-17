#!/bin/sh

./cargo.sh build --release

mkdir target/publish
cp target/release/pkg-install target/publish
cp target/release/config.json target/publish
