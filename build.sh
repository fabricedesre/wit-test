#!/bin/bash

set -e

mkdir -p static

wit-bindgen host js --export wit/delegate.wit \
                    --import wit/service.wit

# Useful to look at the generated binding.rs
# wit-bindgen guest rust --import wit/delegate.wit \
#                        --export wit/service.wit

rm *.d.ts
mv *.js static/

cargo build --release --target=wasm32-unknown-unknown

cp ./target/wasm32-unknown-unknown/release/wit_test.wasm static/wit_test.wasm

ls -l static/*.wasm
