#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

name=$(dasel package.name -f Cargo.toml)
name=${name//\'/}
pkill -9 $name || true

RUST_BACKTRACE=short \
  exec cargo run
