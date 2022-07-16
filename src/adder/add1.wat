(module
    (func $add (param $lf i32) (param $rt i32) (result i32)
        (i32.add
            (local.get $lf)
            (local.get $rt)
        )
    )
    (export "add" (func $add))
)