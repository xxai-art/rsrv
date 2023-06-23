#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cargo test -p csdb --test test -- --nocapture 2>&1 | tee >(sed $'s/\033[[][^A-Za-z]*m//g' >out.txt)
