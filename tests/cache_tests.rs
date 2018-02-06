#[macro_use]
extern crate sam;

#[cfg(test)]
mod tests {
    extern crate machina;

    use self::machina::cache::{Cache, STUB};

    #[test]
    fn insert() {
        let mut cache = Cache::new(false);
        cache.insert("mov_rax_3".to_string(), sam!(x64 => "mov rax, 3"));
        let asm = cache.get("mov_rax_3".to_string());
        assert_eq!(asm, sam!(x64 => "mov rax, 3"));
    }

    #[test]
    fn insert_with_stub() {
        let s_asm = format!("mov rax, {}", STUB);
        let mut cache = Cache::new(false);
        cache.insert_with_stub("mov_rax_x".to_string(), sam!(x64 => &s_asm));
        let asm = cache.get("mov_rax_x".to_string());
        assert_eq!(asm, sam!(x64 => &s_asm));
    }

    #[test]
    fn get_stub() {
        let s_asm = format!("mov rax, {}", STUB);
        let mut cache = Cache::new(false);
        cache.insert_with_stub("mov_rax_x".to_string(), sam!(x64 => &s_asm));
        let asm = cache.get_stub("mov_rax_x".to_string(), 0x00000000);
        assert_eq!(asm, sam!(x64 => "mov rax, 0x00000000"));
    }
}
