use cache_macro::*;
mod rust_caching;

#[test_macro("a")] // this test macro will convert
fn test() {
    println!("hey");
}

#[test]
fn main() {
    // If the test works, S{} should be invalid
    // and H{} should work.
    let cache = rust_caching::MemCache{
        max_size: 500000000u64,
    };
    println!("h");
}