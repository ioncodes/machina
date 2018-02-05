use std::mem;
use std::ops::{Index, IndexMut};
use region::{View, Protection};
use libc;
use aligned_alloc;

extern "C" {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

pub const PAGE_SIZE: usize = 4096; // TODO: Reconsider this.

pub struct Machina {
    memory: *mut u8,
    memory_size: usize,
    memory_ptr: *mut (),
    pc: usize,
}

impl Machina {
    pub fn new(amount_pages: usize) -> Machina {
        let memory: *mut u8;
        let memory_size = amount_pages * PAGE_SIZE;
        let ptr: *mut libc::c_void;
        unsafe {
            ptr = aligned_alloc::aligned_alloc(PAGE_SIZE, memory_size) as *mut libc::c_void;
            let mut view = View::new(ptr as *const u8, memory_size).unwrap();
            view.set_prot(Protection::ReadWriteExecute.into()).unwrap();
            memset(ptr, 0xc3, memory_size);

            memory = mem::transmute(ptr);
        }
        let memory_ptr = ptr as *mut ();
        Machina {
            memory,
            memory_size,
            memory_ptr,
            pc: 0,
        }
    }

    pub fn fill(&mut self, asm: u8) {
        for i in 0..self.memory_size {
            self[i] = asm;
        }
    }

    pub fn reset(&mut self) {
        self.fill(0xc3);
        self.pc = 0;
    }

    pub fn emit(&mut self, asm: u8) {
        let pc = self.pc;
        self[pc] = asm;
        self.pc += 1;
    }

    pub fn emit_bytes(&mut self, asm: Vec<u8>) {
        for b in asm {
            self.emit(b);
        }
    }

    pub fn emit32(&mut self, asm: u32) {
        self.emit((asm & 0xFF) as u8);
        self.emit(((asm >> 8) & 0xFF) as u8);
        self.emit(((asm >> 16) & 0xFF) as u8);
        self.emit(((asm >> 24) & 0xFF) as u8);
    }

    pub fn emit64(&mut self, asm: u64) {
        self.emit32((asm & 0xFFFFFFFF) as u32);
        self.emit32(((asm >> 32) & 0xFFFFFFFF) as u32);
    }

    pub fn execute(&self) -> usize {
        let func: (fn() -> usize);
        unsafe {
            func = mem::transmute(self.memory);
        }

        func()
    }
}

impl Index<usize> for Machina {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe { &*self.memory.offset(_index as isize) }
    }
}

impl IndexMut<usize> for Machina {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe { &mut *self.memory.offset(_index as isize) }
    }
}

impl Drop for Machina {
    fn drop(&mut self) {
        unsafe {
            aligned_alloc::aligned_free(self.memory_ptr);
        }
    }
}
