install:
    rustup toolchain install nightly --allow-downgrade && rustup default nightly && rustup target add wasm32-unknown-unknown && cargo install cargo-generate && npm install -g sass && cargo install --locked cargo-leptos

watch-npm:
    npm run watch

watch-cargo:
    cargo leptos watch