use rust_caching::*;

fn expensive_fn(cache: &mut memory::MemCache, num: i32) -> i32 {
    check_cache!(cache, args!(num), i32, {
        // Function code here
        4i32
    })
}

#[test]
fn main() {
    let cache_size = 10;
    println!("Cache size {}", cache_size);
    let mut cache = memory::MemCache::new(cache_size);
    for i in 1..20 {
        for _ in 0..2 {
            expensive_fn(&mut cache, i as i32);
            println!("Cache size: {}/{}", cache.size(), cache.max_size);
        }
    }
}