#!/bin/sh

./scripts/wasm-bindgen-macroquad.sh clothsim $1

# https://github.com/WebAssembly/wabt
# wasm-strip docs/wbindgen/clothsim.wasm
mv docs/wbindgen/clothsim_bg.wasm docs/
mv docs/wbindgen/clothsim.js docs/

if [ "$1" = "serve" ]
then
    # cargo install basic-http-server
    basic-http-server docs
fi
