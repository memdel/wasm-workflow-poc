(module
  (import "" "" (func $host_function_square (param i32) (result i32)))
  (import "" "" (func $host_function_add (param i32 i32) (result i32)))
    (func (export "main") (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $host_function_add)
)
