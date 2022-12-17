use arboard::Clipboard;
use image::{ImageBuffer, Rgba};
use rdev::{listen, Event, EventType, Key};
mod gui;
use crate::gui::run;

fn main() {
    run();
    return;
    let mut is_meta = false;
    let mut clipboard = Clipboard::new().unwrap();
    let mut parse_clipboard = move || match clipboard.get_image() {
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
    };

    let callback = move |event: Event| {
        // println!("My callback {:?}", event);
        match event.event_type {
            EventType::KeyPress(key) => match key {
                Key::KeyC | Key::KeyX => {
                    if is_meta {
                        println!("copy!!");
                        parse_clipboard();
                    }
                }
                Key::MetaLeft | Key::MetaRight => is_meta = true,
                _ => {
                    if is_meta {
                        is_meta = false;
                    }
                }
            },
            _ => (),
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
