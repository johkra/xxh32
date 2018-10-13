#![feature(test)]

extern crate test;
extern crate xxh32;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use xxh32::XXH32;

    const BUFSIZE: usize = 100 * 1024 * 1024;

    #[bench]
    fn bench_hashing(b: &mut Bencher) {
        let mut buf = vec![0u8; BUFSIZE];
        b.bytes = BUFSIZE as u64;

        for (i, el) in buf.iter_mut().enumerate() {
            *el = i as u8;
        }
        b.iter(|| {
            let mut xxh32 = XXH32::default();
            xxh32.write(&buf);
            xxh32.finish()
        })
    }
}
