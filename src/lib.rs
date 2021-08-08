//! rust_caching allows you to cache functions (or sections of codes)
//! easily and simply using the LRU strategy; either in memory or
//! with a file system.
//! 
//! [ ADD AN EXAMPLE AND STUFF ]

extern crate walkdir; // count files in directory
extern crate bincode;
extern crate byteorder;

pub mod memory;
pub mod file;

/// Memory macro to cache a section of code, ideally used with args! macro
/// Usage:
/// ```edition2018
/// check_cache!(
///     &mut MemCache,                          // MemCache struct to store cache
///     args!("arguments", "for", "function"),  // All input arguments to match with cache
///     i32,                                    // output type
///     { ... }                                 // Regular function code to run and cache
/// }
/// ```
#[macro_export]
macro_rules! cache_mem {
    ($s:expr, $a:expr, $r:ty, $b:block) => {
        // $s: cache struct
        // $a: argument id
        // $r: return type
        // $b: block of code
        match $s.check_cache::<$r>($a) {
            std::option::Option::Some(cached_result) => {
                // Return cached value
                *cached_result
            },

            std::option::Option::None => {
                // Execute the block of code, cache and return the return
                // value.
                let block_return_val = $b;
                $s.write_cache($a, Box::new(block_return_val));
                block_return_val
            }
        }
    }
}

#[macro_export]
macro_rules! cache_file {
    ($s:expr, $a:expr, $r:ty, $b:block) => {
        // $s: cache struct
        // $a: argument id
        // $r: return type
        // $b: block of code
        match $s.check_cache::<$r>($a) {
            std::option::Option::Some(cached_result) => {
                // Return cached value
                *cached_result
            },

            std::option::Option::None => {
                // Execute the block of code, cache and return the return
                // value.
                let block_return_val = $b;
                $s.write_cache($a, Box::new(block_return_val));
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
        use std::hash::{Hash, Hasher};
        let mut s = std::collections::hash_map::DefaultHasher::new();
        $(
            $x.hash(&mut s);
        )*

        function!().hash(&mut s);

        s.finish()
    };}
}