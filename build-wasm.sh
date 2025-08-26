#! /bin/sh
set -eu

# wasm-pack build --release --no-opt
echo "Building rxing-wasm..."
cargo build --release --target wasm32-unknown-unknown
echo "Generating bindings..."
wasm-bindgen ./target/wasm32-unknown-unknown/release/rxing_wasm.wasm --out-dir ./pkg --out-name rxing_wasm
echo "Optimizing wasm..."
wasm-opt pkg/rxing_wasm_bg.wasm -o pkg/rxing_wasm_bg-opt.wasm --new-wat-parser --enable-nontrapping-float-to-int --enable-bulk-memory --vacuum --strip-debug --strip-producers --inlining-optimizing --optimize-casts --alignment-lowering --global-refining --gufa-optimizing --dce -Oz -O4
echo "Finalizing..."
mv pkg/rxing_wasm_bg-opt.wasm pkg/rxing_wasm_bg.wasm
cp ./LICENSE ./pkg/LICENSE
cp ./README.md ./pkg/README.md
echo "Syncing version..."
ver=$(awk '/^\[package\]/{p=1;next} p&&/^\[/{exit} p&&/^version[[:space:]]*=/ {match($0,/"([^"]+)"/,m); print m[1]; exit}' ./Cargo.toml) && sed -i -E 's/("version"[[:space:]]*:[[:space:]]*")[^"]+(")/\1'"$ver"'\2/' ./pkg/package.json