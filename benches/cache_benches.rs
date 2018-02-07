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

    #[bench]
    #[cfg(not(feature = "optimize"))]
    fn get_stub_unoptimized(b: &mut Bencher) {
        let mut cache = Cache::new();
        b.iter(|| {
            let a_asm = format!("mov rax, {}", STUB);
            let b_asm = format!("add rax, {}", STUB);
            let c_asm = format!("mov rbx, {}", STUB);
            let names = ["mov_rax_x","add_rax_x","mov_rbx_x"];
            cache.insert_with_stub("mov_rax_x".to_string(), sam!(x64 => &a_asm));
            cache.insert_with_stub("add_rax_x".to_string(), sam!(x64 => &b_asm));
            cache.insert_with_stub("mov_rbx_x".to_string(), sam!(x64 => &c_asm));
            for _ in 0..50 {
                for name in names.iter() {
                    let _ = cache.get_stub(name.to_string(), 0x00000000);
                }
            }
        });
    }

    #[bench]
    #[cfg(feature = "optimize")]
    fn get_stub_optimized(b: &mut Bencher) {
        // must be run with --features "optimize"
        let mut cache = Cache::new();
        b.iter(|| {
            let a_asm = format!("mov rax, {}", STUB);
            let b_asm = format!("add rax, {}", STUB);
            let c_asm = format!("mov rbx, {}", STUB);
            let names = ["mov_rax_x","add_rax_x","mov_rbx_x"];
            cache.insert_with_stub("mov_rax_x".to_string(), sam!(x64 => &a_asm));
            cache.insert_with_stub("add_rax_x".to_string(), sam!(x64 => &b_asm));
            cache.insert_with_stub("mov_rbx_x".to_string(), sam!(x64 => &c_asm));
            for _ in 0..50 {
                for name in names.iter() {
                    let _ = cache.get_stub(name.to_string(), 0x00000000);
                }
            }
        });
    }
}
