const CopyPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
  plugins: [
    new CopyPlugin({
        patterns: ["index.html"]
    })
  ],
  experiments: {
    topLevelAwait: true
  }
};
