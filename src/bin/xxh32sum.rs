extern crate xxh32;

use xxh32::XXH32;

use std::io;
use std::io::Read;

fn main() {
    let mut xxh32 = XXH32::new();
    let mut buffer = [0 as u8; 4096];

    loop {
        match io::stdin().read(&mut buffer) {
            Ok(len) => {
                if len == 0 {
                    break;
                }
                xxh32.write(&buffer[..len])
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }

    println!("{:08x}", xxh32.finish());
}
