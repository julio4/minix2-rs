use std::env;
use std::fs::File;
use std::io::Read;

use minix2_rs::header::Header;
use minix2_rs::text_segment::TextSegment;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Args validation
    if args.len() < 2 {
        println!("Usage: {} <binary file>", args[0]);
        return;
    }

    let file = File::open(&args[1]).expect("File not found");
    let binary = file
        .bytes()
        .map(|b| b.expect("Error reading binary file"))
        .collect::<Vec<u8>>();

    // Hexdump of binary
    // println!("{:?}", formatter::HexdumpFormatter(&binary));

    // Parse header
    let header = match Header::parse(&binary) {
        Ok(h) => h,
        Err(e) => {
            println!("Error parsing header: {}", e);
            return;
        }
    };

    // Parse text segment
    let text_segment = match TextSegment::parse(&binary, header.text) {
        Ok(t) => t,
        Err(e) => {
            println!("Error parsing text segment: {}", e);
            return;
        }
    };

    println!("{}", text_segment);
}
