use std::collections::HashMap; // hashmap of cached args and results
use std::fmt::Display; // box cache results of any type
use std::mem::size_of_val; // get current cache size
use std::collections::hash_map::DefaultHasher; // convert args to id
use std::hash::{Hash, Hasher}; // convert args to id

/// Struct to determine how to cache results
/// in memory (faster)
pub struct MemCache {
    pub max_size: u64, // Maximum caching size in bytes
    hits: u32,
    misses: u32,
    cache: HashMap<u64, Box<dyn Display + 'static>>,
}

macro_rules! args_id { 
    ($($x:expr), *) => {{
        let mut s = DefaultHasher::new();
        let mut v: Vec<u64> = Vec::new();
        $(
            $x.hash(&mut s);
        )*

        s.finish()
    };}
}

fn main() {
    args_id!();
}