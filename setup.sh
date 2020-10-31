echo "Compiling wasm..."
RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack -vv build demo/

echo "Installing dependencies..."
yarn install