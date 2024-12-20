#! /bin/sh

wasm-pack build --release
wasm-opt pkg/rxing_wasm_bg.wasm -o pkg/rxing_wasm_bg-opt.wasm --enable-bulk-memory -O1 --inlining-optimizing --heap-store-optimization --optimize-casts -O4
mv pkg/rxing_wasm_bg-opt.wasm pkg/rxing_wasm_bg.wasm
