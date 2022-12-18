use std::thread;

// use arboard::Clipboard;
use image::{ImageBuffer, Rgba};
use rdev::{listen, Event, EventType, Key};
mod font;
mod gui;
mod hotkey;
mod utils;
use crate::gui::run;

fn main() {
    // hotkey::listen_copy();
    // let o = hotkey::ctrl_c().unwrap();
    // print!("o: {}", o);
    // return;
    utils::init_e_copy_json();
    run();
}
