(module
  (memory $mem 1)


  (func $distance (param $x i32)(param $y i32)(result i32)
    (i32.sub (get_local $x) (get_local $y))
  )

  (func $validJumpDistance (param $from i32) (param $to i32) (result i32)
    (local $d i32)
    (set_local $d
    (if (result i32)
      (i32.gt_s (get_local $to) (get_local $from))
      (then
        (call $distance (get_local $to) (get_local $from))
      )
      (else
        (call $distance (get_local $from) (get_local $to))
      ))
    )
    (i32.le_u
      (get_local $d)
      (i32.const 2)
    )
  )

  (export "distance" (func $distance))
  (export "validJumpDistance" (func $validJumpDistance))
)
