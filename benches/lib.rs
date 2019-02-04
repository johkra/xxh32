use bencher::Bencher;
use bencher::{benchmark_group, benchmark_main};
use xxh32::XXH32;

const BUFSIZE: usize = 100 * 1024;

fn bench_aligned(b: &mut Bencher) {
    let mut buf = [0u8; BUFSIZE];
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

fn bench_unaligned(b: &mut Bencher) {
    let mut buf = [0u8; BUFSIZE + 1024];
    b.bytes = BUFSIZE as u64;

    for (i, el) in buf.iter_mut().enumerate() {
        *el = i as u8;
    }
    b.iter(|| {
        let mut xxh32 = XXH32::default();
        xxh32.write(&buf[1..1 + BUFSIZE]);
        xxh32.finish()
    })
}

benchmark_group!(benches, bench_aligned, bench_unaligned);
benchmark_main!(benches);
