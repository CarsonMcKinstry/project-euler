const path = require('path');
const HtmlWebpackPlugin = require("html-webpack-plugin");

const entry = path.resolve(__dirname, 'web/bootstrap.ts');

const output = {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bootstrap.js'
};

const mod = {
    rules: [
        {
            test: /\.ts$/,
            use: 'ts-loader',
            exclude: /node_modules/
        }
    ]
}

const resolve = {
    extensions: ['.ts', '.js'],
    alias: {
        "@wasm": path.resolve(__dirname, 'pkg')
    }
};

const plugins = [
    new HtmlWebpackPlugin({
        template: path.resolve(__dirname, 'public/index.html'),
        inject: 'body'
    })
];

const mode = process.env.NODE_ENV || 'development';

const experiments = {
    syncWebAssembly: true,
}

const config = {
    mode,
    entry,
    output,
    resolve,
    module: mod,
    plugins,
    experiments
};

module.exports = config;