#[macro_use]
extern crate sam;

#[cfg(test)]
mod tests {
    extern crate machina;

    use self::machina::machina::Machina;

    #[test]
    fn first() {
        let mut machina = Machina::new(1);
        machina.emit_bytes(sam!(x64 => "mov rax, 0x1337"));
        let rax = machina.execute();
        drop(machina);
        assert_eq!(rax, 0x1337);
    }
}
