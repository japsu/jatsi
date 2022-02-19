# Yahtzee game with configurable rules

The objective of this project is to play Yahtzee with the ["Roleplayer's Ruleset"](https://www.facebook.com/groups/634315316668818/posts/4325558567544456/) online with friends.

## Getting started

### Web front-end

Install tools needed for front-end development:

    rustup target add wasm32-unknown-unknown
    cargo install trunk wasm-bindgen-cli

Run the front-end development environment, watching for changes over all packages:

    cd jatsi_web
    trunk serve -w ..

### Server

Run the server:

    RUST_LOG=info cargo run --bin jatsi_server

Watch & restart on changes:

    cargo watch -x "run --bin jatsi_server"

### Tests

    cargo test

Watch & re-run tests on changes:

    cargo install cargo-watch
    cargo watch -x test
