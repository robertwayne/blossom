cargo fmt --all
cargo test -p blossom -p blossom -- --quiet
cargo clippy --workspace -- -A clippy::single_component_path_imports
cargo sqlx prepare --check --merged
cd blossom-dashboard
npm run fmt
npm run lint