# Build with the wasm target
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown

# Generate bindings with wasm-bindgen-cli into a `generated` directory
# cargo install -f wasm-bindgen-cli && 
wasm-bindgen --out-dir generated --web target/wasm32-unknown-unknown/debug/wgpu-multiplatform.wasm