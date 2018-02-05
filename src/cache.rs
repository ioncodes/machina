use std::collections::HashMap;
use std::mem::transmute;
use entry::Entry;
use helpers::find_subsequence;

pub const STUB: usize = 0x13371337;

pub struct Cache {
    stub: usize,
    cache: HashMap<String, Entry>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            stub: STUB,
            cache: HashMap::<String, Entry>::new(),
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

    pub fn get(&self, name: String) -> &Vec<u8> {
        &self.cache.get(&name).unwrap().asm
    }

    pub fn get_stub(&mut self, name: String, value: usize) -> &Vec<u8> {
        let asm = &mut self.cache.get_mut(&name).unwrap().asm;
        let bytes: [u8; 4] = unsafe { transmute((self.stub as u32).to_le()) };
        let actual_bytes: [u8; 4] = unsafe { transmute((value as u32).to_le()) };
        let pos = find_subsequence(&asm, &bytes).unwrap();
        for i in pos..pos + 4 {
            asm[i] = actual_bytes[i - pos];
        }
        asm
    }
}
