mod args;
use args::Args;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use std::str;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum OpenSSLErrors {
    RsaBitsNotSupported,
}

fn main() -> Result<(), OpenSSLErrors> {
    let args: Args = Args::new();

    println!("{:?}", args);
    return create_primary_key(&args.path, args.rsa_bits);
}

fn create_primary_key(path: &str, bits: u32) -> Result<(), OpenSSLErrors> {
    if bits < 2096 || bits > 4096 {
        return Err(OpenSSLErrors::RsaBitsNotSupported);
    }

    let rsa: Rsa<Private> = Rsa::generate(bits).unwrap();
    let pkey: PKey<Private> = PKey::from_rsa(rsa).unwrap();

    let pub_key: Vec<u8> = pkey.public_key_to_pem().unwrap();

    let pub_key_txt: &str = str::from_utf8(pub_key.as_slice()).unwrap();

    let mut file_pub_key: File =
        File::create(path).expect("Error encountered while creating file!");

    file_pub_key
        .write_all(pub_key_txt.as_bytes())
        .expect("Error trying to write public key");

    Ok(())
}
