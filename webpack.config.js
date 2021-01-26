const path = require('path');
const webpack = require('webpack');




/*
 * We've enabled Postcss, autoprefixer and precss for you. This allows your app
 * to lint  CSS, support variables and mixins, transpile future CSS syntax,
 * inline images, and more!
 *
 * To enable SASS or LESS, add the respective loaders to module.rules
 *
 * https://github.com/postcss/postcss
 *
 * https://github.com/postcss/autoprefixer
 *
 * https://github.com/jonathantneal/precss
 *
 */

const autoprefixer = require('autoprefixer');
const precss = require('precss');




/*
 * We've enabled MiniCssExtractPlugin for you. This allows your app to
 * use css modules that will be moved into a separate CSS file instead of inside
 * one of your module entries!
 *
 * https://github.com/webpack-contrib/mini-css-extract-plugin
 *
 */

const MiniCssExtractPlugin = require('mini-css-extract-plugin');




module.exports = {
  mode: 'development',
  entry: './frontend/ts/index.ts',

  output: {
    path: path.resolve(__dirname, 'public/js/')
  },

  plugins: [
    new webpack.ProgressPlugin(),
    new MiniCssExtractPlugin({ filename:'main.[contenthash].css' })
  ],

  module: {
    rules: [{
      test: /\.(ts|tsx)$/,
      loader: 'ts-loader',
      include: [path.resolve(__dirname, 'frontend/ts')],
      exclude: [/node_modules/]
    }, {
      test: /.css$/,

      use: [{
        loader: MiniCssExtractPlugin.loader
      }, {
        loader: "css-loader",

        options: {
          importLoaders: 1,
          sourceMap: true
        }
      }, {
        loader: "postcss-loader",

        options: {
          plugins: function () {
                        return [
                          precss,
                          autoprefixer
                        ];
                      }
        }
      }]
    }]
  },

  resolve: {
    extensions: ['.tsx', '.ts', '.js']
  },

  devServer: {
    open: true,
    host: 'localhost'
  }
}