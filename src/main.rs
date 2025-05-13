use std::path::Path;

mod hashing;
mod preprocessing;

fn main() {
    let path = Path::new("image.jpg");
    println!("{}", hashing::p_hash(path));
    println!("{}", hashing::d_hash(path));
}