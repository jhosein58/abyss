#!/bin/bash

set -e

echo "Building WebAssembly target..."
cargo build --lib --target wasm32-unknown-unknown --release

echo "Generating wasm-bindgen output..."
wasm-bindgen ../../target/wasm32-unknown-unknown/release/abyss_wasm.wasm --out-dir ./pkg --target web

echo "✅ Build completed successfully! Check the './pkg' folder."

echo "🚀 Starting server"

php -S localhost:8080
