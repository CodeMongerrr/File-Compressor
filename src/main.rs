extern crate flate2;

use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};
use std::time::Instant;

enum Operation {
    Compress,
    Decompress,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 4 {
        eprintln!("Usage: [compress|decompress] 'source' 'target'");
        return;
    }

    let operation = match args[1].as_str() {
        "compress" => Operation::Compress,
        "decompress" => Operation::Decompress,
        _ => {
            eprintln!("Usage: [compress|decompress] 'source' 'target'");
            return;
        }
    };

    let source_file = &args[2];
    let target_file = &args[3];

    match operation {
        Operation::Compress => compressor(source_file, target_file),
        Operation::Decompress => decompressor(source_file, target_file),
    }
}

fn compressor(source: &str, target: &str) {
    let input_file = File::open(source).unwrap();
    let mut input = BufReader::new(input_file);
    let output_file = File::create(target).unwrap();
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let _ = encoder.finish().unwrap();
    println!("Compression completed.");
    println!("Elapsed: {:?}", start.elapsed());
}

fn decompressor(source: &str, target: &str) {
    let input_file = File::open(source).unwrap();
    let input = BufReader::new(input_file);
    let output_file = File::create(target).unwrap();
    let mut output = BufWriter::new(output_file);
    let mut decoder = GzDecoder::new(input);
    let start = Instant::now();
    copy(&mut decoder, &mut output).unwrap();
    println!("Decompression completed.");
    println!("Elapsed: {:?}", start.elapsed());
}
