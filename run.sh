#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

name=$(dasel package.name -f Cargo.toml)
name=${name//\'/}

mkdir -p out
# 不直接用cargo run，是因为watchexe和cargo run一起用总是会端口冲突，不知道为什么
cargo build -Z unstable-options
pkill -9 $name || true
./target/debug/$name
