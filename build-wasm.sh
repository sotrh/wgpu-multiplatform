# New build command
RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build


# OLD BUILD COMMANDS
# Needed for pure HTML5 example
RUSTFLAGS=--cfg=web_sys_unstable_apis cargo build --target wasm32-unknown-unknown
wasm-bindgen --out-dir generated --web target/wasm32-unknown-unknown/debug/wgpu-multiplatform.wasm

# Generate bindings with wasm-bindgen-cli into a `generated` directory
# cargo install -f wasm-bindgen-cli && 

# Copy the generated folder to various places it's needed
# cp -r generated vuepress/.vuepress/public/
