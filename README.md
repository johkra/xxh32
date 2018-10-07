# xxh32

Rust implementation of the xxh32 hash used in the LZ4 compression format.
Original C implementation at https://github.com/Cyan4973/xxHash

Thanks to compiler improvements, performance is now on par with the original C
implementation:

```
$ ./xxhsum -b
./xxhsum 0.6.5 (64-bits little endian), by Yann Collet
Sample of 100 KB...
XXH32               :     102400 ->    63518 it/s ( 6202.9 MB/s)
XXH32 unaligned     :     102400 ->    63626 it/s ( 6213.5 MB/s)
XXH64               :     102400 ->   126702 it/s (12373.3 MB/s)
XXH64 unaligned     :     102400 ->   125901 it/s (12295.0 MB/s)
```

```
$ cargo run --release --bin bench
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/bench`
Hashing 2048MB in 0.351s 0.344s 0.344s
Maximum throughput: 6247 MB/s
```
