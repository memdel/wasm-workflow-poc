# permit running this chmod +x ./run_workflow_manager_cli.sh

cd "$(dirname "$0")"

# if you want to precent install comment the below
cargo run --manifest-path ../wasm_executor/crates/workflow_manager/Cargo.toml