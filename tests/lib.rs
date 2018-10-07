extern crate xxh32;

#[cfg(test)]
mod tests {
    use xxh32::XXH32;

    #[test]
    fn empty_hash() {
        let mut xxh32 = XXH32::new();
        xxh32.write(b"");
        assert_eq!(xxh32.finish(), 0x02cc5d05)
    }

    #[test]
    fn to_16_hashes() {
        let content = b"0123456789abcdef";
        let hashes: [u32; 16] = [
            0x48454cb2, 0x034d0471, 0x48009497, 0x0a0b4c93, 0x8aa3b71c, 0x994e4577, 0x1907ad24,
            0x189bbfbf, 0x0493d634, 0x950c9c0a, 0xd6509106, 0xba508d84, 0x6ba381b4, 0xf45aca85,
            0x1dbdfa0f, 0xc2c45b69,
        ];

        for i in 0..16 {
            let mut xxh32 = XXH32::new();
            xxh32.write(&content[0..i + 1]);
            assert!(
                xxh32.finish() == hashes[i],
                format!(
                    "{} failed: {:08x} != {:08x}",
                    i + 1,
                    xxh32.finish(),
                    hashes[i]
                )
            );
        }
    }

    #[test]
    fn small_updates() {
        let mut xxh32 = XXH32::new();
        xxh32.write(b"0");
        xxh32.write(b"1");
        assert_eq!(xxh32.finish(), 0x034d0471);
    }

    #[test]
    fn multi_updates() {
        let mut xxh32 = XXH32::new();
        xxh32.write(b"0123456789abc");
        xxh32.write(b"0123456789abc");
        assert_eq!(xxh32.finish(), 0x2cf3b22b);
    }

    #[test]
    fn mpl_hash() {
        let mpl = b"This Source Code Form is subject to the terms of the Mozilla \
Public License, v. 2.0. If a copy of the MPL was not distributed \
with this file, You can obtain one at https://mozilla.org/MPL/2.0/.\n";
        let mut xxh32 = XXH32::new();
        xxh32.write(mpl);
        assert_eq!(xxh32.finish(), 0xe52c5e91);
    }
}
