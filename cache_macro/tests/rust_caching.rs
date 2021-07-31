use std::collections::HashMap; // hashmap of cached args and results
use std::fmt::Display; // box cache results of any type
use std::mem::size_of_val; // get current cache size
use std::collections::hash_map::DefaultHasher; // convert args to id
use std::hash::{Hash, Hasher}; // convert args to id

/// Struct to determine how to cache results
/// in memory (faster)
#[derive(Debug)]
pub struct MemCache {
    pub max_size: u64, // Maximum caching size in bytes
    hits: u32,
    misses: u32,
    // temp removed for printing
    //cache: HashMap<u64, Box<dyn Display + 'static>>,
}

impl MemCache {
    /// Create a new memory cache struct
    pub fn new(max_size: u64) -> MemCache {
        return MemCache {
            max_size,
            hits: 0u32,
            misses: 0u32,
            // temp removed for printing
            //cache: HashMap::new(),
        };
    }
}