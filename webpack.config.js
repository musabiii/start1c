const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");

module.exports = {
  mode: "development",
  context: path.join(__dirname, "src-front/src"),
  entry: "./index.js",
  output: {
    filename: "bundle.js",
    path: path.join(__dirname, "dist"),
  },
  plugins: [
    new HtmlWebpackPlugin({ template: "index.html" }),
    new CleanWebpackPlugin(),
  ],
};
