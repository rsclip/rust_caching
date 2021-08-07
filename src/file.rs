use std::fs; // file system
use walkdir::WalkDir;
use std::any::Any;


/// Struct to handle caching with a file system
pub struct FileCache<'a> {
    pub max_size: usize, // maximum cached filed
    pub directory: &'a str, // directory to store cache
    pub cache: CacheMap<'a>, // index files in directory
}

/// Store and manage and ordered list of cached values
pub struct CacheMap<'a> {
    file: &'a str,
}

impl<'a> CacheMap<'a> {
    /// Create a new CacheMap instance attached to a cache directory
    fn from(file: &'a str) -> CacheMap<'a> {
        return CacheMap {
            file,
        };
    }
}

impl<'a> FileCache<'a> {
    /// Create a new file cache struct of a certain size
    pub fn new(max_size: usize) -> FileCache<'a> {
        let directory = "cache/";
        let file = "cache/map";

        return FileCache {
            max_size,
            directory: directory,
            cache: CacheMap::from(file),
        };
    }

    /// Check for cached files and return the value as a specific type `T`
    pub fn check_cache<T: 'static>(&mut self, arg_id: u64) -> Option<&T> {
        None
    }

    /// Add a cached file
    pub fn write_cache(&mut self, arg_id: u64, return_val: Box<dyn Any>) {
        // Make sure there is sufficient space for cache
        self.free_space();
    }

    pub fn size(&self) -> usize {
        WalkDir::new(self.directory).into_iter().count()
    }

    fn free_space(&mut self) {
        let current_size = self.size();
        let target_size = self.max_size - 1;
        if current_size > target_size {
            self.remove(current_size - target_size);
        }
    }

    fn remove(&mut self, num: usize) {
        unimplemented!();
    }
}