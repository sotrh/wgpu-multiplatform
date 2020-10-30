echo "Compiling wasm..."
RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack -vv build

# TODO: figure out how to use yarn workspaces with wasm
echo "Setting up `yarn link` to generated wasm package..."
cd pkg
yarn link
cd ..
yarn link "wgpu-multiplatform"

echo "Installing dependencies..."
yarn install