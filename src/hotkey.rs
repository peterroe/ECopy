use arboard::Clipboard;
use rdev::{listen, simulate, Event, EventType, Key};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::utils::{CopyItem, EcopyJson};

pub fn listen_copy(state: Arc<Mutex<EcopyJson>>) {
    let mut last_text = "".to_string();
    let mut clipboard = Clipboard::new().unwrap();
    loop {
        thread::sleep(Duration::from_millis(200));
        let new_c = clipboard.get_text().unwrap();
        if last_text != new_c {
            thread::sleep(Duration::from_millis(100));
            last_text = new_c;
            state.lock().unwrap().data.insert(
                0,
                CopyItem {
                    time: 234,
                    content: last_text.clone(),
                },
            );
            println!("{}", last_text);
        }
    }
}
