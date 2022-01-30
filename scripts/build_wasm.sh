#!/bin/bash
set -e

cd $(dirname "$0")
cd ..
cd lib

if [[ $* == *--release* ]]
then
	echo "Building release..."
	wasm-pack build --out-dir wasm --release --features wasm
else
	wasm-pack build --out-dir wasm --features wasm
fi
sed -i '3 i \ \ "type": "module",' ./wasm/package.json
