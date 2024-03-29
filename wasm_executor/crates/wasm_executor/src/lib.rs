use anyhow::Result;
use std::thread;
use std::time::Duration;
use wasmtime::{Config, Engine, Func, FuncType, Instance, Module, Store, Val, ValType};

/// squares the input
async fn square(number_to_square: i32) -> i32 {
    // simulate delay
    let duration_to_park = Duration::from_secs(2);
    thread::park_timeout(duration_to_park);
    number_to_square * number_to_square
}

/// adds inputs together
async fn add(a: i32, b: i32) -> i32 {
    // simulate delay
    let duration_to_park = Duration::from_secs(2);
    thread::park_timeout(duration_to_park);
    a + b
}

/// executes WASM that is passed in as byte array
/// also takes params and passed them as an input
pub async fn execute_binary_with_parameters(binary: Vec<u8>, params: Vec<i32>) -> Result<i32> {
    let engine = Engine::new(Config::default().async_support(true))?;
    let mut store = Store::<()>::new(&engine, ());
    let square_type = FuncType::new([ValType::I32], Some(ValType::I32));
    let add_type = FuncType::new([ValType::I32, ValType::I32], Some(ValType::I32));

    // host provided function wrapper
    let host_function_square =
        Func::new_async(&mut store, square_type, |_caller, params, results| {
            Box::new(async move {
                if let Val::I32(param_1) = params.get(0).unwrap() {
                    let count = square(param_1.to_owned()).await;
                    results[0] = Val::I32(count);
                }
                Ok(())
            })
        });

    // host provided function wrapper
    let host_function_add = Func::new_async(&mut store, add_type, |_caller, params, results| {
        Box::new(async move {
            if let Val::I32(param_a) = params.get(0).unwrap() {
                if let Val::I32(param_b) = params.get(1).unwrap() {
                    let count = add(param_a.to_owned(), param_b.to_owned()).await;
                    results[0] = Val::I32(count);
                }
            }
            Ok(())
        })
    });

    let module = Module::from_binary(store.engine(), &binary)?;
    let instance = Instance::new_async(
        &mut store,
        &module,
        &[host_function_square.into(), host_function_add.into()],
    )
    .await;

    let call_add_twice = instance
        .unwrap()
        .get_typed_func::<(i32, i32), i32>(&mut store, "main")?;

    call_add_twice
        .call_async(&mut store, (params[0], params[1]))
        .await
}
