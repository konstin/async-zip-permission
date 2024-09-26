#!/usr/bin/env bash
cargo run
unzip data.zip
ls -lah Readme.txt

cd data
zip regular.zip b.txt
cd ..
unzip data/regular.zip
ls -lah b.txt
