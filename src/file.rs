use std::fs; // file system
use walkdir::WalkDir;
use std::any::Any;
use byteorder::{ByteOrder, LittleEndian};

/// Struct to manage a file system for caching
pub struct FileCache {
    pub max_size: usize,
    pub cache: CacheIndex,
}

/// Manage and index cached files
pub struct CacheIndex {
    pub file: String, // "cache/map"
    pub directory: String, // "cache/"
}

impl FileCache {
    /// Create a new file cache struct of a certain size
    pub fn new(max_size: usize) -> FileCache {
        FileCache {
            max_size,
            cache: CacheIndex::new(
                String::from("cache/"),
                String::from("cache/map"),
            ),
        }
    }

    /// Register a serialized cached value
    pub fn write_cache(&self, arg_id: u64, return_val: Vec<u8>) {
        // Make sure there is enough space
        self.free_space();

        // Cache return_val and register in index
        self.cache.register(arg_id, return_val);
    }
}

impl CacheIndex {
    /// Create a new cache index
    pub fn new(file: String, directory: String) -> CacheIndex {
        CacheIndex {
            file,
            directory,
        }
    }
}