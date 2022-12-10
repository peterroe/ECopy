extern crate arboard;
use arboard::{Clipboard, ImageData};
use image::{ImageBuffer, Rgba};
use std::thread;
use std::time::Duration;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.get_image() {
        Ok(data) => {
            let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
                data.width as u32,
                data.height as u32,
                data.bytes.to_vec(),
            )
            .expect("Fail To parse");
            img.save("clipboard.png").expect("parse error");
        }
        Err(_) => match clipboard.get_text() {
            Ok(c) => println!("output: {}", c),
            Err(_) => println!("No image or text found in clipboard."),
        },
    }
}
