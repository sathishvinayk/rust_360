use hex;
use sha2::{Digest, Sha256, Sha512};

fn crypto256() {
    let mut hasher = Sha256::new();

    hasher.update(b"An Sha256 byte string");

    let result = hasher.finalize();

    let hex = hex::hex_string(&result);
    println!("{:?}", hex);
}

fn crypto512() {
    let mut hasher = Sha512::new();

    hasher.update(b"This is a 512 byte string");
    let result = hasher.finalize();

    let hex = hex::hex_string(&result);
    println!("{:?}", hex);
}

fn main() {
    crypto256();
    crypto512();
}