RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build

# TODO: figure out how to use yarn workspaces with wasm
cd pkg
yarn link
cd ..
yarn link "wgpu-multiplatform"

yarn install