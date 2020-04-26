const path = require('path');

module.exports = {
  mode: 'development',
  entry: './src/index.js',
  output: {
    filename: 'bundle.js',
    path: path.join(__dirname, 'dist/'),
  },
  devServer: {
    contentBase: path.join(__dirname, 'dist/'),
    port: 8080,
    open: true,
  }
};
