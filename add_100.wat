(module
  ;; we pass a function from the host
  (import "" "" (func $host_function_square (param i32) (result i32)))

  ;; we pass a function from the host
  (import "" "" (func $host_function_add (param i32 i32) (result i32)))

    ;; then we define the main function of this wat file
    (func (export "main") (param i32 i32) (result i32)
    ;; add 1o to stack
    i32.const 100

    ;; add first input to stack
    local.get 0

    ;; we could call a function that was passed in from the host 
    ;; call $host_function_add)
    
    ;; we just sum the values in the stack
    i32.add
    )
)
