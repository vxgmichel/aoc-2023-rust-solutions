#!/bin/bash
set -e
BASEDIR=$(realpath $(dirname $0))
cargo fmt
cargo clippy
cargo build --profile=release-with-overflow-checks
for i in {01..25}
do
    if [ ! -d "$BASEDIR/day$i" ]; then continue; fi;
    echo "Day $i"
    echo "------"
    cd $BASEDIR/day$i
    /usr/bin/time -f "(in %U seconds)" cargo run --profile=release-with-overflow-checks -q < data.txt
    echo
done
