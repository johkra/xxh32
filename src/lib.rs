#![no_std]

const PRIME32_1: u32 = 2_654_435_761;
const PRIME32_2: u32 = 2_246_822_519;
const PRIME32_3: u32 = 3_266_489_917;
const PRIME32_4: u32 = 668_265_263;
const PRIME32_5: u32 = 374_761_393;

pub struct XXH32 {
    seed: u32,
    total_len: usize,
    v1: u32,
    v2: u32,
    v3: u32,
    v4: u32,
    memory: [u8; 16],
    memsize: usize,
}

fn read_u32_le(bytes: &[u8]) -> u32 {
    assert_eq!(bytes.len(), 4);
    u32::from(bytes[0])
        | u32::from(bytes[1]) << 8
        | u32::from(bytes[2]) << 16
        | u32::from(bytes[3]) << 24
}

fn calc_next_chunk(val: u32, bytes: &[u8]) -> u32 {
    read_u32_le(bytes)
        .wrapping_mul(PRIME32_2)
        .wrapping_add(val)
        .rotate_left(13)
        .wrapping_mul(PRIME32_1)
}

impl Default for XXH32 {
    fn default() -> XXH32 {
        XXH32::new_with_seed(0)
    }
}

impl XXH32 {
    pub fn new_with_seed(seed: u32) -> XXH32 {
        XXH32 {
            seed,
            total_len: 0,
            v1: seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2),
            v2: seed.wrapping_add(PRIME32_2),
            v3: seed,
            v4: seed.wrapping_sub(PRIME32_1),
            memory: [0; 16],
            memsize: 0,
        }
    }
    pub fn write(&mut self, bytes: &[u8]) {
        self.total_len += bytes.len();

        if self.memsize + bytes.len() < 16 {
            self.memory[self.memsize..self.memsize + bytes.len()].clone_from_slice(bytes);
            self.memsize += bytes.len();
            return;
        }

        let mut bytesview = &bytes[(16 - self.memsize) % 16..];

        if self.memsize > 0 {
            self.memory[self.memsize..].clone_from_slice(&bytes[..16 - self.memsize]);

            self.v1 = calc_next_chunk(self.v1, &self.memory[0..4]);
            self.v2 = calc_next_chunk(self.v2, &self.memory[4..8]);
            self.v3 = calc_next_chunk(self.v3, &self.memory[8..12]);
            self.v4 = calc_next_chunk(self.v4, &self.memory[12..16]);

            self.memsize = 0;
        }

        while bytesview.len() >= 16 {
            self.v1 = calc_next_chunk(self.v1, &bytesview[0..4]);
            self.v2 = calc_next_chunk(self.v2, &bytesview[4..8]);
            self.v3 = calc_next_chunk(self.v3, &bytesview[8..12]);
            self.v4 = calc_next_chunk(self.v4, &bytesview[12..16]);

            bytesview = &bytesview[16..];
        }

        self.memory[..bytesview.len()].clone_from_slice(bytesview);
        self.memsize += bytesview.len();
    }

    pub fn finish(&self) -> u32 {
        let mut h32 = if self.total_len >= 16 {
            self.v1
                .rotate_left(1)
                .wrapping_add(self.v2.rotate_left(7))
                .wrapping_add(self.v3.rotate_left(12))
                .wrapping_add(self.v4.rotate_left(18))
        } else {
            self.seed.wrapping_add(PRIME32_5)
        };

        h32 = h32.wrapping_add(self.total_len as u32);

        let mut memoryview = &self.memory[..self.memsize];

        while memoryview.len() >= 4 {
            h32 = h32.wrapping_add(read_u32_le(&memoryview[0..4]).wrapping_mul(PRIME32_3));
            h32 = h32.rotate_left(17).wrapping_mul(PRIME32_4);

            memoryview = &memoryview[4..];
        }

        for byte in memoryview {
            let byte_u32 = u32::from(*byte);
            h32 = h32.wrapping_add(byte_u32.wrapping_mul(PRIME32_5));
            h32 = h32.rotate_left(11).wrapping_mul(PRIME32_1);
        }

        h32 = (h32 ^ (h32 >> 15)).wrapping_mul(PRIME32_2);
        h32 = (h32 ^ (h32 >> 13)).wrapping_mul(PRIME32_3);
        h32 ^ (h32 >> 16)
    }
}
