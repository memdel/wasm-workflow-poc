(module
  (import "" "" (func $host_function_square (param i32) (result i32)))
  (import "" "" (func $host_function_add (param i32 i32) (result i32)))
    (func (export "main") (param i32 i32) (result i32)
    i32.const 100
    local.get 0
    call $host_function_add)
)
