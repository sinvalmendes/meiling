#!/bin/bash
# script to build and execute meiling at once

echo $1
cargo build
cp target/debug/meiling .
./meiling $1 $2
