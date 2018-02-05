#[macro_use]
extern crate sam;

#[cfg(test)]
mod tests {
    extern crate machina;

    use self::machina::memory::Memory;

    #[test]
    fn first() {
        let mut memory = Memory::new(1);
        memory.emit_bytes(sam!(x64 => "mov rax, 0x1337"));
        let rax = memory.execute();
        drop(memory);
        assert_eq!(rax, 0x1337);
    }
}
