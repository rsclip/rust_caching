use cache_macro::*;
mod rust_caching;

static mut cache: rust_caching::MemCache = rust_caching::MemCache::new(50000000u64);

#[test_macro(cache)]
fn test(_a: String) {
    println!("hey");
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
    println!("{:#?}", cache);
}