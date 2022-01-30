#!/bin/bash

set -e

cd $(dirname "$0")
cd ..
cd test

cargo build && diff -y -b <(node test.mjs --only_js --svelte "$@") <(node test.mjs --only_js --lithe "$@")
