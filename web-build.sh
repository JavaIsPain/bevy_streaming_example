#!/bin/bash
cd "$(dirname "$0")"

NAME=$(sed -n 's/^name = "\(.*\)"/\1/p' < ./game_client/Cargo.toml)

cargo build --release --target wasm32-unknown-unknown --bin game_client

wasm-bindgen --out-dir ./dist/ --target web ./target/wasm32-unknown-unknown/release/$NAME.wasm 

cp index.html ./dist/index.html

sed -i "s/GAME_TITLE/$NAME/" ./dist/index.html

cp -R ./assets ./dist/assets