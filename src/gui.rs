use std::{
    sync::{Arc, Mutex},
    thread,
};

// use arboard::Clipboard;
use eframe::{
    egui::{self, style::Margin},
    epaint::text::{LayoutJob, TextWrapping},
};
use egui::*;

use crate::{
    font, hotkey,
    utils::{self, CopyItem, EcopyJson},
};

pub fn run() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::vec2(150 as f32, 200 as f32)),
        transparent: true,
        decorated: false,
        ..Default::default()
    };
    eframe::run_native("ECopy", options, Box::new(set_task));
}

fn set_task(_cc: &eframe::CreationContext) -> Box<dyn eframe::App> {
    let state = Arc::new(Mutex::new(utils::get_e_copy_json()));
    {
        let s = state.clone();
        thread::spawn(move || {
            hotkey::listen_copy(s);
        });
    }
    Box::new(Ecopy::new(_cc, state.clone()))
}
struct Ecopy {
    count: i32,
    show_decorated: bool,
    // clipboard: Clipboard,
    json: Arc<Mutex<EcopyJson>>,
}

impl Ecopy {
    fn new(_cc: &eframe::CreationContext<'_>, state: Arc<Mutex<EcopyJson>>) -> Self {
        font::install_fonts(&_cc.egui_ctx);

        Self {
            count: 23,
            show_decorated: false,
            // clipboard: Clipboard::new().unwrap(),
            json: state,
        }
    }
    // fn set_clipboard_content(&mut self, str: &str) {
    //     if let Err(err) = self.clipboard.set_text(str) {
    //         panic!("Fail: parse text to clipboard {}", err);
    //     }
    // }
}

impl eframe::App for Ecopy {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // _frame.set_decorations(false)
        let fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 0);
        let frame = egui::Frame::none()
            .fill(fill)
            .inner_margin(Margin::same(5.0));
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::Button::new(self.count.to_string()));
                    if ui.button("📎").clicked() {
                        self.show_decorated = !self.show_decorated;
                        _frame.set_decorations(self.show_decorated);
                    }
                    ui.separator();
                    if ui.button("clear").clicked() {
                        utils::EcopyJson::clear(&mut self.json.lock().unwrap());
                    }
                });
                ui.separator();
            });

            let wrap = TextWrapping {
                max_rows: 1,
                break_anywhere: true,
                overflow_character: Some('…'),
                ..Default::default()
            };
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let data = self.json.lock().unwrap();

                    // println!("{:?}", data.clone().data);
                    // self.json
                    //     .lock()
                    //     .unwrap()
                    //     .clone()
                    //     .data
                    let mut o = data.clone();
                    o.data.iter_mut().for_each(|item| {
                        let words = &item.content;
                        // ui.separator();
                        let mut job =
                            LayoutJob::single_section(words.to_string(), TextFormat::default());
                        job.wrap = wrap.clone();

                        let btn_res = ui
                            .button(job)
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        if btn_res.clicked() {
                            // self.clipboard
                            // self.set_clipboard_content(words);
                            self.count += 1;
                            utils::set_clip_board(words);
                        }
                    });
                });
        });
    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }
}
