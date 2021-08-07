use rust_caching::*;
use std::time::Instant; // get time elapsed

fn steps_to(cache: &mut memory::MemCache, stair: u128) -> u128 {
    check_cache!(cache, args!(stair), u128, {
        match stair {
        1 => { 1 },
        2 => { 2 },
        3 => { 4 },
        _ => {
            steps_to(cache, stair - 3)
            + steps_to(cache, stair - 2)
            + steps_to(cache, stair - 1)
        }
    }})
    
}

#[test]
fn main() {
    // stair = 36 would take about 10 seconds
    // as stair increases, the time taken expotentially increases
    let mut cache = memory::MemCache::new(100);
    let stair = 120u128;

    let now = Instant::now();
    let result = steps_to(&mut cache, stair);
    let elapsed = now.elapsed().as_secs();

    println!("Steps to {}: {} (Elapsed {}s)", stair, result, elapsed);
    println!("Cache size: {}", cache.size());
}