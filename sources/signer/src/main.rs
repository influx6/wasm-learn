use std::env;
use std::fs::File;
use std::io::prelude::*;

use signatory::{ed25519, Encode};
use signatory_dalek::{Ed25519Signer, Ed25519Verifier};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut file = File::open(path)?;
    let mut wasm_buf = Vec::new();
    let _bytes_read = file.read_to_end(&mut wasm_buf)?;
    let buf: &[u8] = &wasm_buf;

    let seed = ed25519::Seed::generate();
    let base64 = signatory::encoding::Base64 {};
    println!(
        "Generated a seed/private key: {}",
        seed.encode_to_string(&base64).unwrap()
    );
    let signer = Ed25519Signer::from(&seed);

    let sig = ed25519::sign(&signer, buf).unwrap();
    println!(
        "Signature for {} created: {}",
        path,
        sig.encode_to_string(&base64).unwrap()
    );

    let pk = ed25519::public_key(&signer).unwrap();
    let verifier = Ed25519Verifier::from(&pk);

    let verified = ed25519::verify(&verifier, buf, &sig).is_ok();
    println!("Signature verified: {}", verified);

    Ok(())
}
