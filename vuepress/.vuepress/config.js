const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");



module.exports = {
    title: 'WebGPU Vuepress Example',
    description: 'Display WebGPU content in Vuepress',
    configureWebpack: {
        plugins: [
            new WasmPackPlugin({
                crateDirectory: __dirname,
            })
        ]
    }
}