#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

cargo bench | tee result.log

test ${PIPESTATUS[0]} -eq 0

./run.js

today=`date +"%Y-%m-%d"`
echo $today > ./.date

if ! hash mdi 2>/dev/null; then
  cargo install mdi
fi

mdi

git add -u

git commit -m"build $today"
