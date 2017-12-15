#!/bin/bash

## first build the test_lib
cd test_lib && cargo build
cd -

## then analyze it 
cargo run -- test_lib  --extern test_lib=./test_lib/target/debug/libtest_lib.rlib
