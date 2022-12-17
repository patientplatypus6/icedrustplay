#!/bin/bash

echo "inside rungui.sh"
echo "First Parameter of the script is $1"

cd /Users/peterweyand/Code/rustprojects/project1.1/project1_1
cargo build && cargo run -- $1