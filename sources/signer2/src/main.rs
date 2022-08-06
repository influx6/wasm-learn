use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use signatory::{ed25519, Encode};
use signatory_dalek::{Ed25519Signer, Ed25519Verifier};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];
    let output = &args[2];

    let infile = load_file(input)?;
    let inbytes: &[u8] = &infile;
    let base64 = signatory::encoding::Base64 {};

    // This seed is a private key - store this in a safe place,
    // Obviously, you'll want to persist this somewhere instead of
    // just using it once in memory...
    let seed = ed25519::Seed::generate();
    let signer = Ed25519Signer::from(&seed);
    let sig = ed25519::sign(&signer, inbytes).unwrap();
    let sig_encoded = sig.encode_to_string(&base64).unwrap();

    let pk = ed25519::public_key(&signer).unwrap();
    let verifier = Ed25519Verifier::from(&pk);

    {
        let mut out_file = File::create(output)?;
        out_file.write(sig.as_bytes())?;
        out_file.write_all(inbytes)?;
    }

    println!(
        "Embedded signature into {} - output {}\n\t-->{}",
        input, output, sig_encoded
    );

    let mut sigbuf = [0; 64];
    let mut wasmbuf = vec![];
    {
        let in_file = File::open(output)?;
        let mut br = BufReader::new(in_file);
        br.read_exact(&mut sigbuf)?;
        br.read_to_end(&mut wasmbuf)?;
    }

    let wasmbytes: &[u8] = &wasmbuf;
    let insig = ed25519::Signature::new(sigbuf);

    let verify_res = ed25519::verify(&verifier, wasmbytes, &insig).is_ok();
    println!("Verification result on new bytes - {}", verify_res);

    Ok(())
}

fn load_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    let _bytes_read = file.read_to_end(&mut buf)?;

    Ok(buf)
}
