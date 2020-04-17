const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './js/bootstrap.js',
  plugins: [
    new CopyWebpackPlugin([
      { from: './static' }
    ]),
    new WasmPackPlugin({
      crateDirectory: '.',
      extraArgs: '--no-typescript',
    })
  ],
  output: {
    filename: 'quiz.bundle.js',
    webassemblyModuleFilename: 'quiz.wasm',
    path: path.resolve(__dirname, 'dist'),
  },
};
