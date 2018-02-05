#![feature(test)]

#[macro_use]
extern crate sam;

#[cfg(test)]
mod tests {
    extern crate machina;
    extern crate test;

    use self::machina::memory::Memory;
    use self::test::Bencher;

    #[bench]
    fn leet(b: &mut Bencher) {
        b.iter(|| {
            let mut memory = Memory::new(1);
            memory.emit_bytes(sam!(x64 => "mov rax, 0x1337"));
            let _ = memory.execute();
            drop(memory);
        });
    }
}
