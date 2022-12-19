cargo fmt --all
cargo test -p blossom_core -p blossom_core -- --quiet
cargo clippy --workspace -- -A clippy::single_component_path_imports
cd crates/blossom_web/client
npm run fmt
npm run lint