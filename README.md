# `bye-nm` :wave:

A trivial tool to get rid of `node_modules` by moving it to `$TMPDIR` instead of deleting it (which is must faster than doing an `rm -rf`) and then letting the OS clean it up later.

## Setup

1. Install Rust if you don't already have it.

    ```sh
    brew install gh rustup-init
    rustup-init
    source $HOME/.cargo/env
    rustup update stable
    ```

2. Clone the repo and build it.

    ```sh
    gh repo clone chriskrycho/bye-nm
    cd bye-nm
    cargo build --release

3.  Put it somewhere on your PATH:

    ```sh
    cp ./target/release/bye-nm <somewhere on your PATH>
    ```

Now, you can just run `bye-nm` anywhere you have a `node_modules` and it'll get rid of it.
