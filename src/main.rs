extern crate murmur3;

use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if env::args().count() == 2 {
        let ref file_name = args[1];
        let mut out: [u8; 16] = [0; 16];
        let file_result = File::open(file_name);
        match file_result {
            Ok(file) => {
                let mut buf = BufReader::new(file);
                murmur3::murmur3_x64_128(&mut buf, 0, &mut out);
                for i in 0..15 {
                    print!("{:02x}", out[i]);
                }
                println!("");
            }
            Err(_) => {
                println!("Failed to open {}", file_name);
            }
        }

    } else {
        println!("{} [filename]", args[0]);
    }
}
