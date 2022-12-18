#[macro_use]
use lazy_static::lazy_static;

use arboard::Clipboard;
use serde_derive::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EcopyJson {
    pub name: String,
    pub data: Vec<CopyItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CopyItem {
    pub time: u64,
    pub content: String,
}

impl EcopyJson {
    fn create_default() -> Value {
        let now = SystemTime::now();
        let ms = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        json!({
            "name": "Ecopy2",
            "data": [{
                "content": "第一条复制的好好的地道",
                "time": ms + 10020
            },{
                "content": "g::builder().add_source(config",
                "time": ms + 2300
            },{
                "content": "ttings_path = std::path::",
                "time": ms + 44300
            },{
                "content": "board on  main [⇡!] is 📦 1.0.0 ",
                "time": ms + 1000
            },{
                "content": " 📦 1.0.0 via 🦀 1.65",
                "time": ms + 10120
            }],
        })
    }
    pub fn clear(ctx: &mut EcopyJson) {
        ctx.data = vec![];
        let serialized = serde_json::to_string::<Value>(&{
            json!({
                "name": "Ecopy2",
                "data": []
            })
        })
        .unwrap();
        std::fs::write(PATH, serialized).unwrap();
    }
}

static PATH: &str = "/Users/lsh/Desktop/t/packages/rust100/clipboard/src/fuck.json";

pub fn init_e_copy_json() {
    let exist_json = Path::new(PATH).exists();
    if !exist_json {
        let d = EcopyJson::create_default();
        let serialized = serde_json::to_string(&d).unwrap();
        std::fs::write(PATH, serialized).unwrap();
    }
}

pub fn get_e_copy_json() -> EcopyJson {
    let json_str = std::fs::read_to_string(PATH).unwrap();
    let mut c: EcopyJson = serde_json::from_str(&json_str).unwrap();
    println!("hello: {:?}", c.data);
    c.data.sort_by(|a, b| b.time.cmp(&a.time));
    return c;
}

pub fn set_e_copy_json(data: EcopyJson) {
    let o = serde_json::to_string::<EcopyJson>(&data).unwrap();
    std::fs::write(PATH, o).unwrap();
}

lazy_static! {
    // pub static ref CLIPBOARD: Mutex<Clipboard> = Mutex::new();
}
pub fn set_clip_board(str: &str) {
    if let Err(err) = Clipboard::new().unwrap().set_text(str) {
        panic!("Fail: parse text to clipboard {}", err);
    }
}
