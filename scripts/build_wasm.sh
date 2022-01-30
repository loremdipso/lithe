#!/bin/bash
set -e

cd $(dirname "$0")
cd ..
cd lib

wasm-pack build --out-dir ../wasm --features wasm
sed -i '3 i \ \ "type": "module",' ../wasm/package.json
