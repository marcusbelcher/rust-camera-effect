# rust-learning

Repository presumes you are on macOS. This is a very inefficient demo on how to flip a camera feed from RGB to BGR via WebGL and Rust.

## Installation

- Install VSCode [VSCode]()
- Install VSCode plugin [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- Install rust by entering `"curl https://sh.rustup.rs -sSf | sh"` into the terminal (preusmes default installation)
- Install Clippy by entering `rustup component add clippy` into the terminal
- Install Rustfmt by entering `rustup component add rustfmt` into the terminal
- Install VSCode plugin [Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

  - Close VSCode and reopen it to make sure rls has installed properly and formatting works

- Install [Wasm-pack](curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh)
- Run `cargo install cargo-generate`
- Install [Node.JS](https://nodejs.org/en/) or update NPM via `npm install npm@latest -g`

## Build

For each rust code change `wasm-pack build && cd www && npm install && npm start && cd ..`

## Useful links

- https://doc.rust-lang.org/
- https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/
- https://rustwasm.github.io/docs/book/
