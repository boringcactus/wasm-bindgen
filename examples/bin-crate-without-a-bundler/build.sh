#!/bin/sh

set -ex

wasm-pack build --target web --bin
python3 -m http.server
