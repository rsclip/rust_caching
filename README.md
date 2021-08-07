# rust_caching

## Description
A simple safe Rust library to cache functions (or sections of code) in memory, mainly ported from my Python module [SimpleCache](https://github.com/Cyclip/SimpleCache). 

## Usage

### Implementing into a cargo project
Add `rust_caching` to your dependencies in `Cargo.toml`:  
```
# Cargo.toml
[dependencies]
rust_caching = "0.1.0"
```

### Demo

Here's a demo which caches an expensive function to calculate the number of possible steps to a specific stair; assuming you can only take up to 3:
```rust
extern crate rust_caching;
use rust_caching::*;

fn steps_to(cache: &mut MemCache, stair: u128) -> u128 {
    // check the `cache` variable with input arguments `stair`,
    // with an output type `u128`
    check_cache!(cache, args!(stair), u128, {
        // If it isn't cached, it will run this block of code
        // and cache it afterwards automatically.
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

fn main() {
    // stair = 36 would take about 10 seconds
    // as stair increases, the time taken expotentially increases
    let mut cache = MemCache::new(100); // create a new memory cache struct
    let stair = 120u128;
   
    let result = steps_to(&mut cache, stair);

    println!("Steps to {}: {}", stair, result);
    println!("Cache size: {}", cache.size());
}
```

The `args!` macro converts a group of arguments into a hashed ID which will then be compared with in the future. 
The `check_cache!` macro takes in the cache struct, input arguments, output type and a block of code to cache and handles it for you.

## Todo
- Identify performance issues
- Update macro to wrap around an entire function
  - Removes the need for specifying input arguments and output type
  - Easy to implement to an existing function
- Add file caching

## Built with
- [Rust](https://www.rust-lang.org/)

## License
rust_caching is licensed with the [MIT License](LICENSE) and is created by [Cyclip](https://www.github.com/Cyclip)
