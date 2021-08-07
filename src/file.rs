use std::fs; // file system
use walkdir::WalkDir;
use std::any::Any;

/// Struct to handle caching with a file system
pub struct FileCache<'a> {
    pub max_size: usize, // maximum cached filed
    pub directory: &'a str, // directory to store cache
    pub cache: CacheMap, // index files in directory
}

/// Store and manage and ordered list of cached values
struct CacheMap<'a> {
    directory: &'a str,
}

impl<'a> CacheMap<'a> {
    /// Create a new CacheMap instance attached to a cache directory
    fn new(directory: &'a str) -> CacheMap<'a> {
        return CacheMap {
            directory: self.format_directory(directory),
        };
    }

    /// Format a caching directory
    fn format_directory(directory: &str) -> &str {
        let point = directory.len() - 1;
        let directory: &str = match &directory[point..] {
            "/" | "\\" | "//" => {
                &directory[..point]
            },
            _ => {directory}
        };

        directory + "map"
    }
}

impl<'a> FileCache<'a> {
    /// Create a new file cache struct of a certain size
    pub fn new(max_size: usize) -> FileCache<'a> {
        let directory = "cache";
        fs::create_dir_all(directory).unwrap();

        return FileCache {
            max_size,
            directory: directory,
            cache: CacheMap::new(directory),
        };
    }

    /// Create a new file cache struct of a certain size in a specific directory
    pub fn new_dir(max_size: usize, directory: &'a str) -> FileCache<'a> {
        fs::create_dir_all(directory).unwrap();

        return FileCache {
            max_size,
            directory,
            cache: Vec::new(),
        }
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