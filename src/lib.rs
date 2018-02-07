extern crate libc;
extern crate aligned_alloc;
extern crate region;

/// Holds the structures for managing memory.
pub mod memory;
/// Holds the structure for managing instructions.
/// It also has a feature called "optimize" which will optimize the cache.
/// This feature should only be used if you're working with many instructions.
pub mod cache;
mod entry;
mod helpers;
mod optimized_cache;