extern crate byteorder;

use byteorder::ByteOrder;
use byteorder::LittleEndian;

const PRIME32_1: u32 = 2654435761;
const PRIME32_2: u32 = 2246822519;
const PRIME32_3: u32 = 3266489917;
const PRIME32_4: u32 = 668265263;
const PRIME32_5: u32 = 374761393;

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

impl XXH32 {
    pub fn new() -> XXH32 {
        XXH32::new_with_seed(0)
    }
    pub fn new_with_seed(seed: u32) -> XXH32 {
        XXH32 {
            seed: seed,
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
            for i in 0..bytes.len() {
                self.memory[self.memsize + i] = bytes[i];
            }
            self.memsize += bytes.len();
            return;
        }

        let mut bytesview = &bytes[..];

        if self.memsize > 0 {
            for i in 0..16 - self.memsize {
                self.memory[self.memsize + i] = bytesview[i];
            }

            let mut memview = &self.memory[..];

            self.v1 = self.v1.wrapping_add(LittleEndian::read_u32(memview).wrapping_mul(PRIME32_2));
            self.v1 = self.v1.rotate_left(13).wrapping_mul(PRIME32_1);
            memview = &memview[4..];

            self.v2 = self.v2.wrapping_add(LittleEndian::read_u32(memview).wrapping_mul(PRIME32_2));
            self.v2 = self.v2.rotate_left(13).wrapping_mul(PRIME32_1);
            memview = &memview[4..];

            self.v3 = self.v3.wrapping_add(LittleEndian::read_u32(memview).wrapping_mul(PRIME32_2));
            self.v3 = self.v3.rotate_left(13).wrapping_mul(PRIME32_1);
            memview = &memview[4..];

            self.v4 = self.v4.wrapping_add(LittleEndian::read_u32(memview).wrapping_mul(PRIME32_2));
            self.v4 = self.v4.rotate_left(13).wrapping_mul(PRIME32_1);

            bytesview = &bytesview[16 - self.memsize..];
            self.memsize = 0;
        }

        while bytesview.len() >= 16 {
            self.v1 = self.v1
                          .wrapping_add(LittleEndian::read_u32(bytesview).wrapping_mul(PRIME32_2));
            self.v1 = self.v1.rotate_left(13).wrapping_mul(PRIME32_1);
            bytesview = &bytesview[4..];

            self.v2 = self.v2
                          .wrapping_add(LittleEndian::read_u32(bytesview).wrapping_mul(PRIME32_2));
            self.v2 = self.v2.rotate_left(13).wrapping_mul(PRIME32_1);
            bytesview = &bytesview[4..];

            self.v3 = self.v3
                          .wrapping_add(LittleEndian::read_u32(bytesview).wrapping_mul(PRIME32_2));
            self.v3 = self.v3.rotate_left(13).wrapping_mul(PRIME32_1);
            bytesview = &bytesview[4..];

            self.v4 = self.v4
                          .wrapping_add(LittleEndian::read_u32(bytesview).wrapping_mul(PRIME32_2));
            self.v4 = self.v4.rotate_left(13).wrapping_mul(PRIME32_1);
            bytesview = &bytesview[4..];
        }

        for byte in bytesview {
            self.memory[self.memsize] = *byte;
            self.memsize += 1;
        }

    }
    pub fn finish(&self) -> u32 {
        let mut h32 = self.seed.wrapping_add(PRIME32_5);

        if self.total_len >= 16 {
            h32 = self.v1
                      .rotate_left(1)
                      .wrapping_add(self.v2.rotate_left(7))
                      .wrapping_add(self.v3.rotate_left(12))
                      .wrapping_add(self.v4.rotate_left(18));
        }

        h32 = h32.wrapping_add(self.total_len as u32);

        let mut memview = &self.memory[..self.memsize];
        while memview.len() >= 4 {
            h32 = h32.wrapping_add(LittleEndian::read_u32(memview).wrapping_mul(PRIME32_3));
            h32 = h32.rotate_left(17).wrapping_mul(PRIME32_4);
            memview = &memview[4..]
        }

        for byte in memview {
            let byte_u32 = *byte as u32;
            h32 = h32.wrapping_add(byte_u32.wrapping_mul(PRIME32_5));
            h32 = h32.rotate_left(11).wrapping_mul(PRIME32_1);
        }

        h32 = (h32 ^ (h32 >> 15)).wrapping_mul(PRIME32_2);
        h32 = (h32 ^ (h32 >> 13)).wrapping_mul(PRIME32_3);
        h32 = h32 ^ (h32 >> 16);

        h32
    }
}
