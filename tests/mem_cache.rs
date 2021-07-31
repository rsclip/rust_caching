use rust_caching::*;
use std::option::Option::{Some, None}

fn expensive_fn(cache: MemCache, num: i32) -> i32 {
    match cache!(cache, args!(num)) {
        Some(val) => {val},
        None => {
            // main function here
            4i32
        }
    }
}

fn main() {
    let mut cache = MemCache::new(50000000u64);
}