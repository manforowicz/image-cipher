use image::io::Reader;
use image::RgbImage;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use bitvec::prelude::*;

const PADDING_LENGTH: usize = 8;

pub fn open_image(image_path: &Path) -> RgbImage {
    let reader = Reader::open(image_path).unwrap_or_else(|err| {
        eprintln!("Error: Image '{}' not found.", image_path.display());
        eprintln!("{err}");
        process::exit(1);
    });

    let img = reader.decode().unwrap_or_else(|err| {
        eprintln!(
            "Error: Image '{}' couldn't be opened.",
            image_path.display()
        );
        eprintln!("{err}");
        process::exit(1);
    });

    img.into_rgb8()
}

/// Returns vector of bytes from file
/// With 16 zero bytes added at the beginning and end
pub fn open_text(text_path: &Path) -> String {
    let mut txt = fs::read(text_path).unwrap_or_else(|err| {
        eprintln!("Text file '{}' not found.", text_path.display());
        eprintln!("{err}");
        process::exit(1);
    });

    let padding = [0; PADDING_LENGTH];
    txt.splice(0..0, padding);
    txt.extend(padding);
    String::from_utf8(txt).unwrap_or_else(|err| {
        eprintln!("Error: '{}' isn't valid utf-8 text.", text_path.display());
        eprintln!("{err}");
        process::exit(1);
    })
}

pub fn write_to_image(img: &mut RgbImage, txt: &str) {
    let mut txt = txt.as_bytes().view_bits::<Msb0>().iter();

    if txt.len() > (img.width() * img.width() * 6) as usize {
        let times = txt.len() as f32 / (img.width() * img.width() * 6) as f32;
        eprintln!("Error: Image must be {times:.2} times larger to fit the provided text.");
        eprintln!("Note: Text is padded with {PADDING_LENGTH} null bytes at front and back.");
        process::exit(1);
    }

    for ch in img.iter_mut() {
        let ch_bits = ch.view_bits_mut::<Lsb0>();
        if let Some(txt_bit) = txt.next() {
            ch_bits.set(1, *txt_bit);
        }
        if let Some(txt_bit) = txt.next() {
            ch_bits.set(0, *txt_bit);
        } else {
            break;
        }
    }
}

pub fn read_from_image(img: &RgbImage) -> String {
    let mut img_iter = img.iter();

    for _ in 0..(PADDING_LENGTH * 4) {
        let ch_bits = img_iter.next().unwrap().view_bits::<Lsb0>();
        if ch_bits[0] || ch_bits[1] {
            eprintln!("Error: Text doesn't start with {PADDING_LENGTH} null bytes.");
            eprintln!("Force try anyways? (y/N): ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.to_lowercase().starts_with('y') {
                img_iter = img.iter();
                break;
            } else {
                process::exit(1);
            }
        }
    }

    let mut bits = BitVec::<u8, Msb0>::new();
    let mut zeros = 0;

    for ch in img_iter {
        let ch_bits = ch.view_bits::<Lsb0>();
        bits.push(ch_bits[1]);
        bits.push(ch_bits[0]);
        if !ch_bits[1] && !ch_bits[0] {
            zeros += 2;
        } else {
            zeros = 0;
        }
        if zeros == PADDING_LENGTH * 8 {
            bits.truncate(bits.len() - PADDING_LENGTH * 8);
            break;
        }
    }

    String::from_utf8(bits.into_vec()).unwrap_or_else(|err| {
        eprintln!("Error: Decoded message isn't valid utf-8.");
        eprintln!("{err}");
        process::exit(1);
    })
}

pub fn save_image(img: &RgbImage, text_path: &Path) {
    let mut image_file = PathBuf::from(text_path);
    let old_name = image_file.file_name().unwrap();

    if !old_name.to_string_lossy().starts_with("encoded-") {
        let mut new_name = OsString::from("encoded-");
        new_name.push(old_name);
        image_file.set_file_name(new_name);
    }
    image_file.set_extension("png");

    img.save(&image_file).unwrap_or_else(|err| {
        eprintln!("Error: Couldn't save image to '{}'", image_file.display());
        eprintln!("{err}");
    });
}

pub fn save_text(text: &str, text_file: &Path) {
    fs::write(text_file, text).unwrap_or_else(|err| {
        eprintln!("Couldn't save text to '{}'", text_file.display());
        eprintln!("{err}");
        process::exit(1);
    })
}
