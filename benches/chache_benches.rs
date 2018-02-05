#![feature(test)]

#[macro_use]
extern crate sam;

#[cfg(test)]
mod tests {
    extern crate machina;
    extern crate test;

    use self::machina::cache::{Cache, STUB};
    use self::test::Bencher;

    #[bench]
    fn insert(b: &mut Bencher) {
        b.iter(|| {
            let mut cache = Cache::new();
            cache.insert("mov_rax_3".to_string(), sam!(x64 => "mov rax, 3"));
            let _ = cache.get("mov_rax_3".to_string());
        });
    }

    #[bench]
    fn insert_with_stub(b: &mut Bencher) {
        b.iter(|| {
            let s_asm = format!("mov rax, {}", STUB);
            let mut cache = Cache::new();
            cache.insert_with_stub("mov_rax_x".to_string(), sam!(x64 => &s_asm));
            let _ = cache.get("mov_rax_x".to_string());
        });
    }

    #[bench]
    fn get_stub(b: &mut Bencher) {
        b.iter(|| {
            let s_asm = format!("mov rax, {}", STUB);
            let mut cache = Cache::new();
            cache.insert_with_stub("mov_rax_x".to_string(), sam!(x64 => &s_asm));
            let _ = cache.get_stub("mov_rax_x".to_string(), 0x00000000);
        });
    }
}
