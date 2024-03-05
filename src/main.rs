use crate::hash::{stingray_hash};

mod hash;

fn main() {
    let  key = "hashlookup".as_bytes();
    let hash = stingray_hash(key);
    println!("MurmurHash64: {:x}", hash);
    println!("bswap64: {:x}", hash.swap_bytes())
}
