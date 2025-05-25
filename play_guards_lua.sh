set -e
# cargo run --bin luagen
cd langs/lua/examples/guards
cargo run -p executor-lua --manifest-path ../../../../Cargo.toml
