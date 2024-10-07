(module
  (import "env" "memory" (memory 1))
  
  (import "env" "log" (func $log (param i32 i32)))
  (import "env" "store_token_balance" (func $store_token_balance (param i64 i64)))

  (func $process_token_account (export "process_token_account") (param $offset i32) (param $length i32)
    (call $log
      (i32.const 0)  ;; Offset of the log message in memory
      (i32.const 26) ;; Length of the log message
    )

    (local $amount i64)
    (local $owner i64)
    (set_local $amount (i64.load (get_local $offset)))
    (set_local $owner (i64.load (i32.add (get_local $offset) (i32.const 8))))

    (call $store_token_balance
      (get_local $owner)
      (get_local $amount)
    )
  )

  (data (i32.const 0) "Processing token account...")
)