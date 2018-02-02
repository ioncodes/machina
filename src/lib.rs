extern crate libc;
extern crate aligned_alloc;
extern crate region;

pub mod machina;

#[cfg(test)]
mod tests {
    use machina::Machina;
    
    #[test]
    fn first() {
        let mut machina = Machina::new(1);
        machina.emit_bytes(vec![0x48, 0xC7, 0xC0, 0x37, 0x13, 0x00, 0x00, 0xC3]);
        let rax = machina.execute();
        drop(machina);
        assert_eq!(rax, 0x1337);
    }
}