Twist Javascript example
========================

To be able to use Rust from Javascript, one needs to export functions via
[napi](https://nodejs.org/api/n-api.html). This project imports the
[neon](https://github.com/neon-bindings/neon) crate and uses its types to
expose the `search` function from twist-rs.

## Building

To build the project, [ensure you have rust and cargo
installed](https://www.rust-lang.org/tools/install) and also the latest version
node and npm.

You will also need neon's CLI:

        npm install -g neon-cli

With all the dependencies installed. You're ready to build the npm package:

        npm install

It will run the `neon` CLI to build the Rust NAPI project and prepare the
`index.node` file needed (available at `native/index.node`).

## Running

To run the project, you just have to execute:

        node lib/index.js
