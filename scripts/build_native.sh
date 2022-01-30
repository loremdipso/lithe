#!/bin/bash
set -e

cd $(dirname "$0")
cd ..
cd lib

cargo build "$@"
