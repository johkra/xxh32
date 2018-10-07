extern crate time;
extern crate xxh32;

use xxh32::XXH32;

const BUFSIZE: usize = 2 * 1024 * 1024 * 1024;
const TRIES: usize = 3;

const NS_IN_SEC: f64 = 1000.0 * 1000.0 * 1000.0;

fn calculate_hash(buf: &[u8]) -> u32 {
    let mut xxh32 = XXH32::new();
    xxh32.write(buf);
    return xxh32.finish();
}

fn main() {
    let mut buf = vec![0; BUFSIZE];

    for x in 0..BUFSIZE {
        buf[x] = x as u8;
    }

    let mut times: [u64; TRIES] = [0; TRIES];

    print!("Hashing {}MB in ", BUFSIZE / 1024 / 1024);
    for try in 0..TRIES {
        let start = time::precise_time_ns();
        calculate_hash(&buf);
        times[try] = time::precise_time_ns() - start;
        print!("{:.*}s ", 3, times[try] as f64 / NS_IN_SEC);
    }
    println!("");

    let min_time = times
        .iter()
        .fold(std::u64::MAX, |acc, &x| std::cmp::min(acc, x));
    let max_throughput = 1000 * BUFSIZE as u64 / min_time;

    println!("Maximum throughput: {} MB/s", max_throughput);
}
