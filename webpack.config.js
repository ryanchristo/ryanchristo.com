const autoprefixer = require('autoprefixer')
const path = require('path')
const precss = require('precss')

module.exports = {
  target: 'web',
  entry: {
    main: path.join(__dirname, '/client/index.jsx')
  },
  output: {
    filename: 'index.min.js',
    path: path.join(__dirname, '/public')
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: [
          {
            loader: 'babel-loader',
            query: 'presets[]=latest,presets[]=react'
          }
        ]
      },
      {
        test: /\.scss$/,
        use: [
          {
            loader: 'style-loader'
          },
          {
            loader: 'css-loader',
            query: {
              importLoaders: 1,
              localIdentName: '[name]__[local]___[hash:base64:5]',
              modules: true
            }
          },
          {
            loader: 'postcss-loader',
            options: {
              plugins() {
                return [
                  precss,
                  autoprefixer
                ]
              }
            }
          }
        ]
      }
    ]
  },
  resolve: {
    extensions: ['.js', '.jsx']
  }
}
