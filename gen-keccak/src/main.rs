extern crate tiny_keccak;
use tiny_keccak::Hasher;
use tiny_keccak::Keccak;

fn main() {
    let input = "1234";
    let mut hasher = Keccak::v256();
    hasher.update(input.as_bytes());
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    println!("Keccak hash of \"{}\": {:?}", input, output);
}