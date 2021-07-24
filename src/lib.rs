use std::collections::HashMap; // hashmap of cached args and results
use std::fmt::Display; // box cache results of any type
use std::mem::size_of_val; // get current cache size

/// Struct to determine how to cache results
/// in memory (faster)
pub struct MemCache {
    pub max_size: u64, // Maximum caching size in bytes
    hits: u32,
    misses: u32,
    cache: HashMap<String, Box<dyn Display + 'static>>,
}