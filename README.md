# image-cipher

A simple command-line tool to [stenographically](https://en.wikipedia.org/wiki/Steganography) hide plaintext utf-8 in images. Replaces the last two bits of each pixel color with the text bits.

## Installation

Requires cargo.

    git clone https://github.com/manforowicz/image-cipher
    cd image-cipher
    cargo build --release

The executable will be in `target/release/image-cipher`.
Run it using `./image-cipher`.

## Usage

`image_cipher <COMMAND>`

Commands:
- `encode`  Write a message to an image. Saved as encoded-{old_name}.png
- `decode`  Read a message from an image
- `help`    Print this message or the help of the given subcommand(s)


### Encode

Usage: `image_cipher encode <IMAGE_FILE> <TEXT_FILE>`

Arguments:
-  `<IMAGE_FILE>`  Image to write message on
-  `<TEXT_FILE>`   The message to write
  
### Decode

Usage: `image_cipher decode <IMAGE_FILE> [TEXT_FILE]`

Arguments:
-  `<IMAGE_FILE>`  Image to read message from
-  `[TEXT_FILE]`   Text file to save message to (optional)
