use indexmap::IndexMap;
use std::mem::size_of_val; // get current cache size
use std::collections::hash_map::DefaultHasher; // convert args to id
use std::hash::{Hash, Hasher}; // convert args to id
use std::any::Any;
use std::option::Option::{Some, None};

/// Struct to determine how to cache results
/// in memory (faster)
pub struct MemCache {
    pub max_size: usize, // Maximum caching size in bytes
    pub hits: u32,
    pub misses: u32,
    pub cache: Cache,
}

impl MemCache {
    /// Create a new memory cache struct
    pub fn new(max_size: usize) -> MemCache {
        return MemCache {
            max_size,
            hits: 0u32,
            misses: 0u32,
            cache: Cache::new(),
        };
    }

    /// Check for cache returning the value
    pub fn check_cache(&self, arg_id: u64) -> Option<Box<dyn Any>> {
        // Match arg_id as cache key and respond accordingly
        match self.cache.get_index(&arg_id) {
            Some(cached_index) => {
                // Move the hit cache to the top

            }
        }
    }

    /// Insert to bottom of cache
    pub fn write_cache(&mut self, arg_id: u64, return_val: Box<dyn Any>) {
        // Remove elements until enough space is allocated
        let current_size = size_of_val(&self);
        let size_to_allocate = size_of_val(&return_val);

        // while predicted size > target size
        while current_size + size_to_allocate > self.max_size {
            // Remove the last
            self.cache.pop();
        }

        self.cache.insert(arg_id, return_val);
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

/// Struct storing indexed cached values of any type
struct Cache {
    store: Vec<CachedObject>,
    size: usize
}

impl Cache {
    /// Create a new cache struct
    fn new() -> Cache {
        Cache {
            store: Vec::new(),
            size: 0
        }
    }

    /// Insert a key & value into cache at a certain position
    fn insert(&mut self, key: u64, val: Box<dyn Any>) -> usize {
        self.store.insert(0, CachedObject::new(key, val));
        self.size += 1;
        0
    }

    /// Move a cached object at an index to the front (after cache hit)
    fn move_front(&mut self, index: usize) {
        // take ownership
        let val = self.store.remove(index);
        self.store.insert(0, val);
    }

    /// Get the cached object's value at a specific index
    fn val<T: 'static>(&self, index: usize) -> &T {
        let result: &T = self.store[index].val.downcast_ref::<T>().unwrap();
        result
    }

    /// Get both the key and value of a cached object at a specific index
    fn val_full<T: 'static>(&self, index: usize) -> (u64, &T) {
        let result: &T = self.store[index].val.downcast_ref::<T>().unwrap();
        (self.store[index].key, result)
    }

    /// Get the index of a cached object based on its key
    fn index_of(&self, key: u64) -> usize {
        match self.store.iter().position(
            |x| x.key == key
        ) {
            Some(x) => {x},
            None => {panic!("key not in cache")}
        }
    }

    /// Remove the last used cache object
    fn free(&mut self) {
        self.store.pop();
        self.size -= 1;
    }
}

/// Stores a single cached result
struct CachedObject {
    key: u64,
    val: Box<dyn Any>,
}

impl CachedObject {
    /// Create a new cached result from a key and value
    fn new(key: u64, val: Box<dyn Any>) -> CachedObject {
        CachedObject {
            key,
            val
        }
    }
}

/// Main macro to cache a section of code, ideally used with args! macro
/// i.e `cache!(cache_struct, args!(1, 2, 3))`
#[macro_export]
macro_rules! check_cache {
    ($s:expr, $a:expr, $b:block) => {
        // $s: cache struct
        // $a: argument id
        match $s.check_cache($a) {
            std::option::Option::Some(cached_result) => {
                // Return cached value
                $s.hit();
                Some(cached_result)
            },

            std::option::Option::None => {
                // Execute the block of code, cache and return the return
                // value.
                $s.miss();
                let block_return_val = $b;
                $s.write_cache($a, block_return_val);
                block_return_val
            }
        }
    }
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