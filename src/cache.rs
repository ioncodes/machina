use std::collections::HashMap;
use std::mem::transmute;
use entry::Entry;
use helpers::find_subsequence;
use optimized_cache::OptimizedCache;

pub const STUB: usize = 0x13371337;

pub struct Cache {
    stub: usize,
    cache: HashMap<String, Entry>,
    optimized_cache: HashMap<OptimizedCache, Vec<u8>>,
    optimize: bool,
}

impl Cache {
    pub fn new(optimize: bool) -> Cache {
        Cache {
            stub: STUB,
            cache: HashMap::<String, Entry>::new(),
            optimized_cache: HashMap::<OptimizedCache, Vec<u8>>::new(),
            optimize: optimize
        }
    }

    pub fn set_stub(&mut self, stub: usize) {
        self.stub = stub;
    }

    pub fn insert(&mut self, name: String, asm: Vec<u8>) {
        self.cache.insert(name, Entry {
            asm: asm,
            stub: 0
        });
    }

    pub fn insert_with_stub(&mut self, name: String, asm: Vec<u8>) {
        self.cache.insert(name, Entry {
            asm: asm,
            stub: self.stub
        });
    }

    pub fn get(&self, name: String) -> Vec<u8> {
        self.cache.get(&name).unwrap().asm.to_owned()
    }

    pub fn get_stub(&mut self, name: String, value: usize) -> Vec<u8> {
        let other = OptimizedCache { name: (&name).to_string(), value };
        let mut asm = self.cache.get(&name).unwrap().asm.to_owned();
        let cached = self.check_cache(&other);
        if self.optimize && cached.is_ok() {
            cached.unwrap().to_owned()
        } else {
            let bytes: [u8; 4] = unsafe { transmute((self.stub as u32).to_le()) };
            let actual_bytes: [u8; 4] = unsafe { transmute((value as u32).to_le()) };
            let pos = find_subsequence(&asm, &bytes).unwrap();
            asm[pos..pos + 4].clone_from_slice(&actual_bytes[(pos - pos)..(pos + 4 - pos)]);
            self.optimized_cache.insert(other, asm.clone());
            asm.to_owned()
        }
    }

    fn check_cache(&self, other: &OptimizedCache) -> Result<Vec<u8>, String> {
        if !self.optimize {
            return Err("Not optimizing.".to_string());
        }
        if let Some(cache) = self.optimized_cache.get(other) {
            Ok(cache.to_owned())
        } else {
            Err("Not in cache.".to_string())
        }
    }
}
