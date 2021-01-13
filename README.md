# rust-ts-template

## About

This is a highly opinionated template for writing projects using TypeScript and WebAssembly.

## Pre-Requisites

- [Node.js](https://nodejs.org/en/)
- [rust](https://www.rust-lang.org/)
- [yarn](https://yarnpkg.com/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Usage

### Commands Available via `yarn`

- `yarn build`: builds your rust package, adds it to the npm dependencies, and builds the application using webpack
- `yarn dev`: does two things:
  - starts a `nodemon` process that watches your rust files, and automatically re-adds the package to the npm dependencies
  - starts a the `webpack-dev-server`
- `yarn test`: run jest to test TypeScript files

### Commands Available via `wasm-pack`

- `wasm-pack build`: builds your rust package
- `wasm-pack test --headless --firefox`: test your rust code in a headless browser

### Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/CarsonMcKinstry/rust-ts-template.git --name my-project
cd my-project
yarn install
```
