# File Compressor

This Rust program compresses files using the gzip compression algorithm. It leverages the `flate2` crate for handling the gzip compression efficiently.

## Features

- Simple command-line interface
- Uses gzip for compression
- Fast compression speeds due to buffered I/O

## Prerequisites

Before you run this program, make sure you have Rust and Cargo installed on your machine. You can install them from [the official Rust website](https://rust-lang.org).

## Installation

Clone this repository to your local machine using the following command:

```bash
git clone https://github.com/yourusername/file-compressor.git
cd file-compressor
```
## Usage

To use this program, you need to specify the source file you want to compress and the target file name for the compressed output. Here is how you can run the program:

```bash
cargo run -- <source_file> <compressed_output>
```
## Example

To compress a file named example.txt and save it as example.gz, you would use:

```bash
cargo run -- example.txt example.gz
```
This will compress example.txt using gzip compression and save it as example.gz. The program also prints the original and compressed file sizes, as well as the time taken to compress.

## How It Works

The program reads command-line arguments to get the source and target file names. It reads the source file into a buffer, compresses it using `GzEncoder` from the `flate2` crate, and writes the compressed data to the target file. It also measures and prints the time taken to perform the compression.

## Error Handling

The program uses unwrap() for simplicity, which will cause the program to panic and exit if an error occurs (e.g., if the specified file cannot be opened). For production use, it is recommended to handle errors more gracefully.

## Contributing

Contributions to this project are welcome. Please feel free to fork the repository and submit a pull request.
