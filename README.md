# machina [![crates.io](https://img.shields.io/crates/v/machina.svg)](https://crates.io/crates/machina) [![docs.rs](https://docs.rs/machina/badge.svg)](https://docs.rs/machina)  
Manage and execute assembly at runtime.

## Crossplatform?
Yap.

## Why?
Emulators, JIT, dynarec, etc.

## Install
To build the tests you need [sam](https://github.com/ioncodes/sam).  
In ```Cargo.toml```:
```toml
[dependencies]
machina = "*"
```
If you work with many instructions, you may want to enable optimization (only applied to ```Cache```):
```toml
[dependencies.machina]
version = "*"
features = ["optimize"]
```

## Example
```rust
extern crate machina;
#[macro_use]
extern crate sam; // just to keep it simple

use machina::cache::{Cache, STUB};
use machina::memory::Memory;

fn main() {
    let asm = format!("mov rax, {}", STUB); // fill in a dummy address
    let dummy = 64;
    let address = &dummy as *const _; // address of dummy

    let cache = Cache::new();
    let memory = Memory::new(1); // 1 page = 1024 bytes
    cache.insert("mov_rax_x".to_string(), sam!(x64 => &asm)); // create the bytes at compile time via sam

    memory.emit_bytes(cache.get_stub("mov_rax_x".to_string(), address as u64)) // get "mov_rax_x" and fill in a dynamic address
    let rax = memory.execute(); // returns rax
}
```
