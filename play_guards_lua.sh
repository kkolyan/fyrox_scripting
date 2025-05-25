set -e
# cargo run --bin luagen
cd langs/lua/examples/guards
cargo run -p fyrox-lite-lua --manifest-path ../../../../Cargo.toml
