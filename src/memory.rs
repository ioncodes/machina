use std::mem;
use std::ops::{Index, IndexMut};
use region::{View, Protection};
use libc;
use aligned_alloc;

extern "C" {
    fn memset(s: *mut libc::c_void, c: libc::uint32_t, n: libc::size_t) -> *mut libc::c_void;
}

/// Default page size
pub const PAGE_SIZE: usize = 1024;

/// Handles memory management.
pub struct Memory {
    memory: *mut u8,
    memory_size: usize,
    memory_ptr: *mut (),
    pc: usize,
}

impl Memory {
    /// Constructs a new `Memory`.
    /// 
    /// * `amount_pages` - Amount pages to allocate
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::memory::Memory;
    ///
    /// let memory = Memory::new(1);
    /// ```
    pub fn new(amount_pages: usize) -> Memory {
        let memory: *mut u8;
        let memory_size = amount_pages * PAGE_SIZE;
        let ptr: *mut libc::c_void;
        unsafe {
            ptr = aligned_alloc::aligned_alloc(memory_size, memory_size) as *mut libc::c_void;
            let mut view = View::new(ptr as *const u8, memory_size).unwrap();
            view.set_prot(Protection::ReadWriteExecute.into()).unwrap();
            memset(ptr, 0xc3, memory_size);

            memory = mem::transmute(ptr);
        }
        let memory_ptr = ptr as *mut ();
        Memory {
            memory,
            memory_size,
            memory_ptr,
            pc: 0,
        }
    }

    /// Fills the entire memory block with the same byte
    /// 
    /// * `asm` - The byte
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::memory::Memory;
    ///
    /// let mut memory = Memory::new(1);
    /// memory.fill(0xc3); // ret
    /// ```
    pub fn fill(&mut self, asm: u8) {
        for i in 0..self.memory_size {
            self[i] = asm;
        }
    }

    /// Resets the memory to its initial state, which is a block full of `ret` instructions.
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::memory::Memory;
    ///
    /// let mut memory = Memory::new(1);
    /// memory.reset();
    /// ```
    pub fn reset(&mut self) {
        self.fill(0xc3);
        self.pc = 0;
    }

    /// Add 1 byte to the memory.
    /// 
    /// * `asm` - The byte
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::memory::Memory;
    ///
    /// let mut memory = Memory::new(1);
    /// memory.emit(0xc3); // ret
    /// ```
    pub fn emit(&mut self, asm: u8) {
        let pc = self.pc;
        self[pc] = asm;
        self.pc += 1;
    }

    /// Appends all bytes to the memory.
    /// 
    /// * `asm` - The bytes
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::memory::Memory;
    ///
    /// let mut memory = Memory::new(1);
    /// memory.emit_bytes(vec![0x48, 0xff, 0xc0]); // mov rax, 3
    /// ```
    pub fn emit_bytes(&mut self, asm: Vec<u8>) {
        for b in asm {
            self.emit(b);
        }
    }

    /// Appends an u32.
    /// This will convert the value into Little Endian.
    /// 
    /// * `asm` - The value (for example an address)
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::memory::Memory;
    ///
    /// let mut memory = Memory::new(1);
    /// memory.emit32(0x12341234);
    /// ```
    pub fn emit32(&mut self, asm: u32) {
        self.emit((asm & 0xFF) as u8);
        self.emit(((asm >> 8) & 0xFF) as u8);
        self.emit(((asm >> 16) & 0xFF) as u8);
        self.emit(((asm >> 24) & 0xFF) as u8);
    }

    /// Appends an u64.
    /// This will convert the value into Little Endian.
    /// 
    /// * `asm` - The value (for example an address)
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::memory::Memory;
    ///
    /// let mut memory = Memory::new(1);
    /// memory.emit64(0x12341234);
    /// ```
    pub fn emit64(&mut self, asm: u64) {
        self.emit32((asm & 0xFFFFFFFF) as u32);
        self.emit32(((asm >> 32) & 0xFFFFFFFF) as u32);
    }

    /// Executes the entire memory block and returns the value of the register `rax`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use machina::memory::Memory;
    ///
    /// let mut memory = Memory::new(1);
    /// let _ = memory.execute();
    /// ```
    pub fn execute(&self) -> usize {
        let func: (fn() -> usize);
        unsafe {
            func = mem::transmute(self.memory);
        }

        func()
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, _index: usize) -> &u8 {
        unsafe { &*self.memory.offset(_index as isize) }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe { &mut *self.memory.offset(_index as isize) }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            aligned_alloc::aligned_free(self.memory_ptr);
        }
    }
}
