# Rebuild the Rust libraries
cargo clean
cargo build --workspace --all-targets

# Recreate the offline sqlx-data.json file
rm sqlx-data.json
cargo sqlx prepare --merged

# Rebuild the TypeScript dashboard
cd blossom-dashboard
rm -rf node_modules
npm install
npm run build
cd ..