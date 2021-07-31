use cache_macro::*;
use std::collections::HashMap; // hashmap of cached args and results

mod rust_caching;
static mut cache: rust_caching::MemCache = rust_caching::MemCache {
    max_size: 50000000u64,
    hits: 0u32,
    misses: 0u32,
    cache: rust_caching::MemCache::default_cache,
};

#[test_macro(cache)]
fn test(_a: String) -> i32 {
    println!("hey");
    3i32
}

/*
Output should be:
#
fn test(_a : String) {
    match cache.check_cache(){
        std :: option :: Option :: Some(cached_result) => { cached_result },
        std :: option :: Option :: None => { println! ("hey") ; }
    }
}
*/

#[test]
fn main() {
    // If the test works, S{} should be invalid
    // and H{} should work.
    // let cache = rust_caching::MemCache::new(50000000u64);
    unsafe {println!("{}", cache.cache.len());}
}