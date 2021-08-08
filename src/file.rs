use std::fs; // file system
use walkdir::WalkDir;
use std::any::Any;
use byteorder::{ByteOrder, LittleEndian};
use std::ops::Index;

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

    /// Register a cached value and index it
    pub fn register(&self, arg_id: u64, return_val: Vec<u8>) {
        // Create and write to cache
        fs::write(
            self.directory + &arg_id.to_string(),
            return_val,
        );

        // Update index
        let mut index = self.read();
        index.insert(0, arg_id);
        self.update(index);
    }

    /// Read the index
    fn read(&self) -> Vec<u64> {
        let encoded: Vec<u8> = fs::read(self.file).unwrap();
        let decoded: Vec<u64> = bincode::deserialize(&encoded).unwrap();
        decoded
    }
    
    /// Update index
    fn update(&self, index: Vec<u64>) {
        let encoded: Vec<u8> = bincode::serialize(&index).unwrap();
        fs::write(
            self.file,
            encoded
        );
    }
}

/// Implement indexing in CacheIndex
impl Index<usize> for CacheIndex {
    type Output = Vec<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        unimplemented!();
    }
}