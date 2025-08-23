#! /bin/sh

wasm-pack build --release --no-opt
wasm-opt pkg/rxing_wasm_bg.wasm -o pkg/rxing_wasm_bg-opt.wasm --enable-nontrapping-float-to-int --vacuum --enable-bulk-memory -O1 --inlining-optimizing --optimize-casts -Oz -O4
mv pkg/rxing_wasm_bg-opt.wasm pkg/rxing_wasm_bg.wasm
