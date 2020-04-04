#!/bin/bash

./cargo.sh build --release

mkdir target/publish
cp target/release/arch-post-install target/publish
cp target/release/config.json target/publish
