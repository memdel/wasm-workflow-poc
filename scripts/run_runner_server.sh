# # permit running this chmod +x ./run_runner_server.sh

cd "$(dirname "$0")"

cargo run --manifest-path ../wasm_executor/crates/web_server/Cargo.toml
