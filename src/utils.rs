use eframe::egui::TextBuffer;
#[macro_use]
use lazy_static::lazy_static;

use arboard::Clipboard;
use home::home_dir;
use serde_derive::{Deserialize, Serialize};
use serde_json::{self, json, Value};
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use eframe::egui;
// use users::os::unix::UserExt;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EcopyJson {
    pub name: String,
    pub pos: [f32; 2],
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
            "pos": [300.0, 300.0],
            "data": [{
                "content": "复制的字符串内容",
                "time": ms + 1000
            },{
                "content": "都会被检测到",
                "time": ms + 800
            },{
                "content": "======",
                "time": ms + 650
            },{
                "content": "点击当前行",
                "time": ms + 400
            },{
                "content": "可以复制当前行的内容",
                "time": ms + 300
            }],
        })
    }
    pub fn clear(ctx: &mut EcopyJson) {
        ctx.data = vec![];
        let [x, y] = ctx.pos;
        let serialized = serde_json::to_string::<Value>(&{
            json!({
                "name": "Ecopy2",
                "egio": [x, y],
                "data": []
            })
        })
        .unwrap();
        std::fs::write(get_config_dir(), serialized).unwrap();
    }
}

// static mut get_config_dir().to_str().unwrap(): &str = "";
fn get_config_dir() -> PathBuf {
    home_dir().unwrap().join(Path::new(".ecopy.json"))
}

pub fn init_e_copy_json() {
    print!("get_config_dir(), {:?}", get_config_dir());
    let exist_json = Path::new::<PathBuf>(&get_config_dir()).exists();
    if !exist_json {
        let d = EcopyJson::create_default();
        let serialized = serde_json::to_string(&d).unwrap();
        std::fs::write(get_config_dir(), serialized).unwrap();
    }
}

pub fn get_e_copy_json() -> EcopyJson {
    let json_str = std::fs::read_to_string(get_config_dir()).unwrap();
    let mut c: EcopyJson = serde_json::from_str(&json_str).unwrap();
    println!("hello: {:?}", c.data);
    c.data.sort_by(|a, b| b.time.cmp(&a.time));
    return c;
}

pub fn set_e_copy_json(data: EcopyJson) {
    let o = serde_json::to_string::<EcopyJson>(&data).unwrap();
    std::fs::write(get_config_dir(), o).unwrap();
}

lazy_static! {
    // pub static ref CLIPBOARD: Mutex<Clipboard> = Mutex::new();
}
pub fn set_clip_board(str: &str) {
    if let Err(err) = Clipboard::new().unwrap().set_text(str) {
        panic!("Fail: parse text to clipboard {}", err);
    }
}
