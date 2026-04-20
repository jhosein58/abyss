#!/bin/bash

set -e

rm -rf ./web/pkg
mkdir -p ./web/pkg

echo "⏳ Building WebAssembly target..."
cargo build --target wasm32-unknown-unknown --release

echo "⏳ Generating wasm-bindgen output..."
wasm-bindgen ../target/wasm32-unknown-unknown/release/abyss_script.wasm --out-dir ./web/pkg --target web

echo "✅ Build completed successfully! Check the './web/pkg' folder."

cd web
echo "🚀 Starting server"

php -S localhost:8080
