use std::collections::HashMap;
use std::mem::transmute;
use entry::Entry;
use helpers::find_subsequence;
use optimized_cache::OptimizedCache;

/// The default stub value
pub const STUB: usize = 0x13371337;

/// Manages the instructions.
pub struct Cache {
    stub: usize,
    cache: HashMap<String, Entry>,
    optimized_cache: HashMap<OptimizedCache, Vec<u8>>,
    optimize: bool,
}

impl Cache {
    /// Constructs a new `Cache`.
    /// 
    /// * `optimize` - Optimize cache
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::cache::Cache;
    ///
    /// let cache = Cache::new(true);
    /// ```
    pub fn new(optimize: bool) -> Cache {
        Cache {
            stub: STUB,
            cache: HashMap::<String, Entry>::new(),
            optimized_cache: HashMap::<OptimizedCache, Vec<u8>>::new(),
            optimize: optimize
        }
    }

    /// Sets the stub.
    /// 
    /// * `stub` - The new stub
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::cache::Cache;
    ///
    /// let mut cache = Cache::new(false);
    /// cache.set_stub(0x31313131);
    /// ```
    pub fn set_stub(&mut self, stub: usize) {
        self.stub = stub;
    }

    /// Insert without stub
    /// 
    /// * `name` - The identifier
    /// * `asm` - The bytes
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::cache::Cache;
    ///
    /// let mut cache = Cache::new(false);
    /// cache.insert("inc_rax".to_string(), vec![0x48, 0xff, 0xc0]);
    /// ```
    pub fn insert(&mut self, name: String, asm: Vec<u8>) {
        self.cache.insert(name, Entry {
            asm: asm,
            stub: 0
        });
    }

    /// Insert with stub
    /// 
    /// * `name` - The identifier
    /// * `asm` - The bytes
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::cache::{Cache,STUB};
    ///
    /// let mut cache = Cache::new(false);
    /// cache.insert_with_stub("mov_rax_x".to_string(), vec![0x48, 0xc7, 0xc0, 0x37, 0x13, 0x37, 0x13]); // bytes with `STUB` (default: 0x13371337)
    /// ```
    pub fn insert_with_stub(&mut self, name: String, asm: Vec<u8>) {
        self.cache.insert(name, Entry {
            asm: asm,
            stub: self.stub
        });
    }

    /// Get without loading the stub
    /// 
    /// * `name` - The identifier
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::cache::Cache;
    ///
    /// let mut cache = Cache::new(false);
    /// cache.insert("inc_rax".to_string(), vec![0x48, 0xff, 0xc0]);
    /// let _ = cache.get("inc_rax".to_string());
    /// ```
    pub fn get(&self, name: String) -> Vec<u8> {
        self.cache.get(&name).unwrap().asm.to_owned()
    }

    /// Get with loading the stub
    /// 
    /// * `name` - The identifier
    /// * `value` - The stub replacement
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::cache::{Cache,STUB};
    ///
    /// let mut cache = Cache::new(false);
    /// cache.insert_with_stub("mov_rax_x".to_string(), vec![0x48, 0xc7, 0xc0, 0x37, 0x13, 0x37, 0x13]); // bytes with STUB (default: 0x13371337)
    /// let _ = cache.get_stub("mov_rax_x".to_string(), 0x69696969); // replace STUB with 0x69696969
    /// ```
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
