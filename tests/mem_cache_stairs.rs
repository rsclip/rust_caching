use rust_caching::*;
use std::time::Instant; // get time elapsed

fn steps_to(cache: &mut MemCache, stair: usize) -> usize {
    check_cache!(cache, args!(stair), usize, {
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
    let mut cache = MemCache::new(100);
    let stair = 50;

    let now = Instant::now();
    let result = steps_to(&mut cache, stair);
    let elapsed = now.elapsed().as_secs();

    println!("Steps to {}: {} (Elapsed {}s)", stair, result, elapsed);
    println!("Cache size: {}", cache.size());
}