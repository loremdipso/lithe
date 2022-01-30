#!/bin/bash

set -e

cd $(dirname "$0")
cd ..

# Only needed for wasm, but we load wasm even if we're only testing native
NODE_ARGS="--experimental-wasm-modules"
TEST_ARGS="--time --file test.svelte"

if [[ $* == *--skip_build* || $* == *--skip_compile* ]]
then
	echo "Skipping build..."
else
	if [[ $* == *--wasm* ]]
	then
		echo "WASM..."
		if [[ $* == *--release* ]]
		then
			echo "Building release..."
			./scripts/build_wasm.sh --release
		else
			echo "Building debug..."
			./scripts/build_wasm.sh
		fi
	fi

	if [[ $* == *--native* ]]
	then
		echo "Native..."
		if [[ $* == *--release* ]]
		then
			echo "Building release..."
			./scripts/build_native.sh --release
		fi

		# if not release, or if do debug, build debug
		if [[ $* != *--release* || $* == *--debug* ]]
		then
			echo "Building debug..."
			./scripts/build_native.sh
		fi
	fi
fi

cd ./test
pwd
if [[ $* == *--show_output* ]]
then
	node $NODE_ARGS ./test.mjs $TEST_ARGS --only_js --show_output "$@"
else
	node $NODE_ARGS ./test.mjs $TEST_ARGS "$@"
fi
