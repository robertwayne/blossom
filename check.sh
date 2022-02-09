cargo fmt --all
cargo test -p blossom_core -p blossom_core -- --quiet
cargo clippy --workspace -- -A clippy::single_component_path_imports -A clippy::from_over_into
markdownlint . --disable MD033
cd crates/blossom_web/client
yarn fmt
yarn lint