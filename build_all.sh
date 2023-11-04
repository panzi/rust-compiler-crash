#!/usr/bin/bash

set -eo pipefail

rm rustc-ice-* rustc-out-* 2>/dev/null || true

for i in {1..6}; do
    echo "building crash $i"
    cargo clean >/dev/null 2>&1
    cargo build --features="crash$i" > "rustc-out-crash$i.txt" 2>&1 || true
    mv rustc-ice-*-*-*T*_*_*-*.txt "rustc-ice-crash$i.txt"
done
