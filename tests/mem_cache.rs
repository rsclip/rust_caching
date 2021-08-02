use rust_caching::*;
use std::option::Option::{Some, None};
use std::hash::{Hash, Hasher}; // convert args to id

fn expensive_fn(cache: &mut MemCache, num: i32) -> i32 {
    match cache!(cache, check_cache!(num)) {
        Some(val) => {
            println!("[{}] Cached", num);
            val
        },
        None => {
            // main function here
            println!("[{}] Uncached", num);
            4i32
        }
    }
}

#[test]
fn main() {
    println!("running main");
    let mut cache = MemCache::new(50000000u64);
    expensive_fn(&mut cache, 3i32);
    expensive_fn(&mut cache, 3i32);
    expensive_fn(&mut cache, 4i32);
    expensive_fn(&mut cache, 4i32);
    expensive_fn(&mut cache, 4i32);
    expensive_fn(&mut cache, 5i32);
}