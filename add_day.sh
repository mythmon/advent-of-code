#!/usr/bin/env bash

set -eu

usage() {
    echo "$0 DAY"
}

if [ $# -ne 1 ]; then
    usage
    exit 1
fi

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DAY=$1

DAY_DIR="${DIR}/src/day${DAY}/"

mkdir -p "${DAY_DIR}"
if [[ ! -e "${DAY_DIR}/part1.rs" ]]; then
    cp "${DIR}/template/code.rs" "${DAY_DIR}/part1.rs"
fi
if [[ ! -e "${DAY_DIR}/part2.rs" ]]; then
    cp "${DIR}/template/code.rs" "${DAY_DIR}/part2.rs"
fi

if ! grep "day${DAY}_part1" "${DIR}/Cargo.toml" > /dev/null; then
    (
        echo
        cat "${DIR}/template/bin.toml.in" | sed -e "s/{D}/${DAY}/" -e "s/{P}/1/"
        echo
        cat "${DIR}/template/bin.toml.in" | sed -e "s/{D}/${DAY}/" -e "s/{P}/2/"
    ) >> "${DIR}/Cargo.toml"
fi

wget "https://adventofcode.com/2017/day/${DAY}/input" \
    -q -O "${DAY_DIR}/input.txt" \
    || (echo "Error: could not fetch input"; rm -f "${DAY_DIR}/input.txt")
