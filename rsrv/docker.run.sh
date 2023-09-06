#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

source ./docker.build.sh

docker stop $name || true
docker run -it -e PG_URI=$PG_URI -p 8080:8080 --name $name --rm $name
