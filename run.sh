#!/usr/bin/env bash

cd ./frontend || exit
trunk build --release
cd ..
cargo run -p backend
