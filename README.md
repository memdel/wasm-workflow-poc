# WASM executor

## How to run it
```bash
# start the server
# permit execution: chmod +x ./scripts/run_runner_server.sh
./scripts/run_runner_server.sh

# start the web client
# permit execution: chmod +x ./scripts/run_web_client.sh
./scripts/run_web_client.sh
```

## How to create those `.wasm` files
* for compiling `.wat` files to `.wasm` you need a tool such as
  https://github.com/WebAssembly/wabt but this is just useful if you are
  learning wat syntax or playing around. after installing it you can do:
  ```bash
  wat2wasm sum_of_2_ints.wat # creates sum_of_2_ints.wasm
  ```
* for compiling Rust projects to `.wasm` you can use the Rusts native compiler
  cababilities and use `cargo build --target wasm32-unknown-unknown`. But be
  aware that as of now this project do not really cope with `WASI`, there there
  is not any system interface functionalities. Additionally the signature if not
  checked out dynamically from the produced `.wasm` files. So maybe best to
  stick to some hand written toy web assembly files for now.

DEMO:
http://workflow-runner.s3-website.eu-central-1.amazonaws.com

* you can use the `.wasm` files at the root of this repository, or write your
own. They take two `i32` and return `i32`.

<div style="display: flex; justify-content: center;"><img width="640px" src="app-screenshot.png">
</div >


This is a simple POC that does the following:

* has a web application client that allows user to select WASM files from the file system
* turns those files to byte arrays
* contains a server side program that has an endpoint accepting those WASM files as byte arrays
* uses Wasmtime to execute the WASM file
* returns the calculation result back to the client that made the call

Some goals:
* the goal is to have these executor run in serverless environments
* provide a fs that persists to S3
* have some common side effect, such as network calls or calls to some databases
* allow defining work flows in JSON
  * provide a UI editor for creating those work flow definitions
* provide SDK for the consumers to be aware of those side effects
