use arboard::Clipboard;
use eframe::{
    egui::{self, style::Margin},
    epaint::text::{LayoutJob, TextWrapping},
};
use egui::*;

use crate::{font, utils};

pub fn run() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::vec2(150 as f32, 200 as f32)),
        transparent: true,
        decorated: false,
        ..Default::default()
    };
    eframe::run_native("ECopy", options, Box::new(|_cc| Box::new(Ecopy::new(_cc))));
}
struct Ecopy {
    count: i32,
    show_decorated: bool,
    clipboard: Clipboard,
    json: utils::EcopyJson,
}

impl Ecopy {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        font::install_fonts(&_cc.egui_ctx);
        Self {
            count: 23,
            show_decorated: false,
            clipboard: Clipboard::new().unwrap(),
            json: utils::get_e_copy_json(),
        }
    }
    fn set_clipboard_content(&mut self, str: &str) {
        if let Err(err) = self.clipboard.set_text(str) {
            panic!("Fail: parse text to clipboard {}", err);
        }
    }
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
                    // ui.add(egui::Button::new("ECopy"));
                    if ui.button("📎").clicked() {
                        self.show_decorated = !self.show_decorated;
                        _frame.set_decorations(self.show_decorated);
                    }
                    ui.separator();
                    if ui.button("clear").clicked() {
                        utils::EcopyJson::clear(&mut self.json);
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
                    // [
                    //     "Do you have any plan?",
                    //     "What is your name?",
                    //     "It's my first i join this game~",
                    //     "What the fuck are you saying?E tis t sfsd",
                    //     "what wrong?",
                    //     "Do you want to a fight?",
                    //     "随便一句话？",
                    //     "Do you want to a fight?",
                    //     "Do you want to a fight?",
                    //     "Do you want to a fight?",
                    //     "Do you want to a fight?",
                    //     "Do you want to a fight?",
                    // ]
                    // .into_iter()
                    self.json.clone().data.into_iter().for_each(|item| {
                        let words = &item.content;
                        // ui.separator();
                        let mut job =
                            LayoutJob::single_section(words.to_string(), TextFormat::default());
                        job.wrap = wrap.clone();

                        let btn_res = ui
                            .button(job)
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        if btn_res.clicked() {
                            self.set_clipboard_content(words);
                            self.count += 1;
                        }
                    });
                });
        });
    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }
}
