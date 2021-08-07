use std::any::Any;
use std::option::Option::{Some, None};

/// Struct to determine how to cache results in memory.
pub struct MemCache {
    pub max_size: usize, // Maximum num of cached values
    pub cache: CacheStore, // cache store
}

/// Struct storing indexed cached values of any type
pub struct CacheStore {
    pub store: Vec<CachedObject>,
    pub size: usize
}

/// Stores a single cached result
pub struct CachedObject {
    pub key: u64,
    pub val: Box<dyn Any>,
}


impl MemCache {
    /// Create a new memory cache struct of a certain size
    pub fn new(max_size: usize) -> MemCache {
        return MemCache {
            max_size,
            cache: CacheStore::new(),
        };
    }

    /// Check for cache returning the value, where T is
    /// the expected output type
    pub fn check_cache<T: 'static>(&mut self, arg_id: u64) -> Option<&T> {
        // Try to get the index of the key
        let index = match self.cache.index_of(arg_id) {
            Some(index) => {self.cache.move_front(index); 0},
            None => {return None;}
        };

        // Get the value
        let val = self.cache.val::<T>(index);

        Some(val)
    }

    /// Add a cached object to the top
    pub fn write_cache(&mut self, arg_id: u64, return_val: Box<dyn Any>) {
        // Ensure there is enough space for the new cache
        self.cache.try_pop(self.max_size);

        // Add key into cache
        self.cache.insert(arg_id, return_val);
    }

    /// Get the number of currently cached values
    pub fn size(&self) -> usize {
        self.cache.store.len()
    }
}

impl CacheStore {
    /// Create a new cache struct
    fn new() -> CacheStore {
        CacheStore {
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
    pub fn move_front(&mut self, index: usize) {
        // take ownership
        let val = self.store.remove(index);
        self.store.insert(0, val);
    }

    /// Get the cached object's value at a specific index
    fn val<T: 'static>(&self, index: usize) -> &T {
        let result: &T = self.store[index].val.downcast_ref::<T>().unwrap();
        result
    }

    /// Get the index of a cached object based on its key
    fn index_of(&self, key: u64) -> Option<usize> {
        self.store.iter().position(
            |x| x.key == key
        )
    }

    /// Ensure there is enough space in cache, else pop
    /// the last element (or remove the required amount of
    /// elements)
    fn try_pop(&mut self, max_size: usize) {
        self.store.truncate(max_size - 1);
    }
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
