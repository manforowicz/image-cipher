# image-cipher

A simple command-line tool to [stenographically](https://en.wikipedia.org/wiki/Steganography) hide plaintext utf-8 in images. Replaces the last two bits of each pixel color with the text bits.

## Installation

Requires cargo.

    git clone https://github.com/manforowicz/image-cipher
    cd image-cipher
    cargo build --release

The executable will be in `target/release/image-cipher`.
Run it using `./image-cipher`.
