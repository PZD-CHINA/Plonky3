extern crate p3_blake3;
extern crate p3_symmetric;

use p3_blake3::Blake3;
use p3_symmetric::CryptographicHasher;

fn main() {
    let input = "1234";
    let hasher = Blake3;
    let hash = hasher.hash_iter(input.bytes());
    println!("Blake3 hash of \"{}\": {:?}", input, hash);
}