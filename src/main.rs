use std::path::Path;
use crate::hashing::p_hash;

mod hashing;
mod preprocessing;

fn main() {
    let path = Path::new("examples/b0ecc859-44e9-430c-8724-dc1be3be8218.jpeg");
    println!("{:#?}", hashing::p_hash(path));
}
