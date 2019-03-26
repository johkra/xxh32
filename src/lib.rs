#![no_std]
use core::num::Wrapping;

const PRIME32_1: Wrapping<u32> = Wrapping(2_654_435_761);
const PRIME32_2: Wrapping<u32> = Wrapping(2_246_822_519);
const PRIME32_3: Wrapping<u32> = Wrapping(3_266_489_917);
const PRIME32_4: Wrapping<u32> = Wrapping(668_265_263);
const PRIME32_5: Wrapping<u32> = Wrapping(374_761_393);

#[derive(Debug)]
pub struct XXH32 {
    total_len: usize,
    v1: Wrapping<u32>,
    v2: Wrapping<u32>,
    v3: Wrapping<u32>,
    v4: Wrapping<u32>,
    memory: [u8; 16],
    mem_used: usize,
}

impl Default for XXH32 {
    fn default() -> XXH32 {
        XXH32::new_with_seed(0)
    }
}

fn wu32_from_le_bytes(bytes: &[u8]) -> Wrapping<u32> {
    Wrapping(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

const fn wu32_rotate_left(wu32: Wrapping<u32>, n: u32) -> Wrapping<u32> {
    Wrapping(wu32.0.rotate_left(n))
}

impl XXH32 {
    pub fn new_with_seed(seed: u32) -> XXH32 {
        let seed = Wrapping(seed);
        XXH32 {
            total_len: 0,
            v1: seed + PRIME32_1 + PRIME32_2,
            v2: seed + PRIME32_2,
            v3: seed,
            v4: seed - PRIME32_1,
            memory: [0; 16],
            mem_used: 0,
        }
    }
    pub fn write(&mut self, bytes: &[u8]) {
        self.total_len += bytes.len();

        let mut main = bytes;
        if self.mem_used > 0 {
            if self.mem_used + bytes.len() < 16 {
                self.memory[self.mem_used..self.mem_used + bytes.len()].copy_from_slice(bytes);
                self.mem_used += bytes.len();
                return;
            }

            let (fill, remaining) = bytes.split_at(16 - self.mem_used);
            self.memory[self.mem_used..].copy_from_slice(fill);

            let mut vars = [&mut self.v1, &mut self.v2, &mut self.v3, &mut self.v4];
            let vars_iter = vars.iter_mut();
            let chunks = self.memory.chunks_exact(4).map(wu32_from_le_bytes);
            assert!(chunks.len() == vars_iter.len());
            for (var, val) in vars_iter.zip(chunks) {
                **var += val * PRIME32_2;
                **var = wu32_rotate_left(**var, 13) * PRIME32_1;
            }

            self.mem_used = 0;

            if remaining.is_empty() {
                return;
            }
            main = remaining;
        };

        let mut iter = main.chunks_exact(16);
        for chunk in iter.by_ref() {
            let mut vars = [&mut self.v1, &mut self.v2, &mut self.v3, &mut self.v4];
            let vars_iter = vars.iter_mut();
            let chunks = chunk.chunks_exact(4).map(wu32_from_le_bytes);
            assert!(chunks.len() == vars_iter.len());
            for (var, val) in vars_iter.zip(chunks) {
                **var += val * PRIME32_2;
                **var = wu32_rotate_left(**var, 13) * PRIME32_1;
            }
        }

        if iter.remainder().is_empty() {
            return;
        }
        self.memory[..iter.remainder().len()].copy_from_slice(iter.remainder());
        self.mem_used += iter.remainder().len()
    }

    pub fn finish(&self) -> u32 {
        let mut h32 = if self.total_len >= 16 {
            wu32_rotate_left(self.v1, 1)
                + wu32_rotate_left(self.v2, 7)
                + wu32_rotate_left(self.v3, 12)
                + wu32_rotate_left(self.v4, 18)
        } else {
            // self.v3 == seed
            self.v3 + PRIME32_5
        };

        h32 += Wrapping(self.total_len as u32);

        let mut iter = self.memory[..self.mem_used].chunks_exact(4);
        for chunk in iter.by_ref() {
            h32 += wu32_from_le_bytes(chunk) * PRIME32_3;
            h32 = wu32_rotate_left(h32, 17) * PRIME32_4;
        }

        for byte in iter.remainder() {
            h32 += Wrapping(u32::from(*byte)) * PRIME32_5;
            h32 = wu32_rotate_left(h32, 11) * (PRIME32_1);
        }

        h32 ^= h32 >> 15;
        h32 *= PRIME32_2;
        h32 ^= h32 >> 13;
        h32 *= PRIME32_3;
        h32 ^= h32 >> 16;
        h32.0
    }
}
