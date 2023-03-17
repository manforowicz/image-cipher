#![warn(clippy::all)]

use clap::{Args, Parser, Subcommand};
use image_cipher::*;
use std::path::PathBuf;

/// Steganographically encode and decode plaintext utf-8 messages from images.
/// Replaces the last two bits of each pixel color with text bits.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    operation: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Write a message to an image.
    /// Saved as encoded-{old_name}.png
    Encode(EncodeArgs),

    /// Read a message from an image
    Decode(DecodeArgs),
}

#[derive(Args, Debug)]
struct EncodeArgs {
    /// Image to write message on.
    image_file: PathBuf,

    /// The message to write.
    text_file: PathBuf,
}

#[derive(Args, Debug)]
struct DecodeArgs {
    /// Image to read message from.
    image_file: PathBuf,

    /// Text file to save message to.
    text_file: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    match cli.operation {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
    }
}

fn encode(args: EncodeArgs) {
    let txt = open_text(&args.text_file);
    let mut img = open_image(&args.image_file);
    write_to_image(&mut img, &txt);
    save_image(&img, &args.image_file);
}

fn decode(args: DecodeArgs) {
    let img = open_image(&args.image_file);
    let message = read_from_image(&img);
    if let Some(text_path) = &args.text_file {
        save_text(&message, text_path);
    } else {
        println!("{message}");
    }
}
