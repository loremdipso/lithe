#!/bin/bash
set -e

cd $(dirname "$0")
cd ..

echo "Pulling all submodules"
git submodule update --init --recursive

echo "test: Installing dependencies"
cd test
npm install

echo "Svelte: Installing dependencies"
cd ../svelte
npm install

echo "Svelte: building"
npm run build

echo "Lithe: building"
cd ../lib
cargo build

echo "Done :)"
