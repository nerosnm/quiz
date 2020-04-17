const merge = require('webpack-merge');
const common = require('./webpack.common.js');

const PORT = process.env.PORT || 5000;

module.exports = merge(common, {
  mode: 'development',
  devServer: {
    contentBase: './dist',
    compress: false,
    port: PORT,
    historyApiFallback: true,
  },
  watch: true,
});
