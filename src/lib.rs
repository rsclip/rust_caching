use std::collections::HashMap; // hashmap of cached args and results
use std::mem::size_of_val; // get current cache size
use std::collections::hash_map::DefaultHasher; // convert args to id
use std::hash::{Hash, Hasher}; // convert args to id

/// Struct to determine how to cache results
/// in memory (faster)
pub struct MemCache {
    pub max_size: u64, // Maximum caching size in bytes
    pub hits: u32,
    pub misses: u32,
    cache: HashMap<u64, Box<dyn Display + 'static>>,
}

impl MemCache {
    /// Create a new memory cache struct
    pub fn new(max_size: u64) -> MemCache {
        return MemCache {
            max_size,
            hits: 0u32,
            misses: 0u32,
            cache: HashMap::new(),
        };
    }

    /// Check for cache returning the value
    pub fn check_cache(&self, arg_id: u64) -> Option<i32> {
        // Match arg_id as cache key and respond accordingly
        match self.cache.get(&arg_id) {
            Some(cached_result) => {
                return Some(**cached_result);
            },
            None => {
                return None;
            }
        };
    }

    /// Register a hit
    pub fn hit(&mut self) {
        self.hits += 1;
    }

    /// Register a miss
    pub fn miss(&mut self) {
        self.misses += 1;
    }
}

/// Main macro to cache a section of code, ideally used with args! macro
/// i.e `cache!(cache_struct, args!(1, 2, 3))`
#[macro_export]
macro_rules! check_cache {
    ($s:expr, $a:expr) => {
        // $s: cache struct
        // $a: argument id
        match $s.check_cache($a) {
            std::option::Option::Some(cached_result) => {
                $s.hit();
                Some(cached_result)
            },

            std::option::Option::None => {
                $s.miss();
                None
            }
        }
    }
}

#[macro_export]
macro_rules! add_cache {
    () => {
        
    };
}

/// Get the current running function
/// Caching should be applied to an entire function generally
/// but it is possible to cache a section of code.
/// (This macro shouldn't be used)
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

/// Convert the input argument variables into a hashed ID
/// i.e `args!(Struct{}, 30)`
#[macro_export]
macro_rules! args {
    ($($x:expr), *) => {{
        let mut s = std::collections::hash_map::DefaultHasher::new();
        $(
            $x.hash(&mut s);
        )*

        function!().hash(&mut s);

        s.finish()
    };}
}