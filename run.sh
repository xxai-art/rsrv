#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

name=$(dasel package.name -f Cargo.toml)
name=${name//\'/}

mkdir -p out

exe=./target/debug/$name
rm $exe

# 不用 cargo run，因为 watchexe 和 cargo run 一起用总是会端口冲突，不知道为什么
cargo build

GREEN='\033[0;92m'
NC='\033[0m'

echo -e "\n${GREEN}❯ $exe$NC\n"
pkill -9 $name || true
if [ -f "$exe" ]; then
  exec $exe
fi
