extern crate libc;
extern crate aligned_alloc;
extern crate region;

/// Holds the structures for managing memory.
pub mod memory;
/// Holds the structure for managing instructions.
pub mod cache;
mod entry;
mod helpers;
mod optimized_cache;