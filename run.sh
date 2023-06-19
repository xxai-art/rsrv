#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

name=$(dasel package.name -f Cargo.toml)
name=${name//\'/}

mkdir -p out
cargo build --out-dir out -Z unstable-options
pkill -9 $name
./out/$name
