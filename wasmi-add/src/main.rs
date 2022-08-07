extern crate wasmi;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = Vec::new();
    {
        let mut f = File::open("./add1.wasm")?;
        f.read_to_end(&mut buffer)?;
    }

    let module = wasmi::Module::from_buffer(buffer)?;
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("Failed to instantiate WASM module")
        .assert_no_start();

    // call the add function - 1. create the arguments
    let mut args = Vec::<RuntimeValue>::new();
    args.push(RuntimeValue::from(42));
    args.push(RuntimeValue::from(1));


    // call the add function - 1. call the function and collect result
    let result: Option<RuntimeValue> = instance.invoke_export("add", &args, &mut NopExternals)?;

    match result {
        Some(RuntimeValue::I32(v)) => {
            println!("The answer to your additon was {}", v);
        }
        Some(_) => {
            println!("Got a value of an unexpected data type");
        }
        None => {
            println!("Failed to get a result from wasm invocation");
        }
    }

    Ok(())
}
