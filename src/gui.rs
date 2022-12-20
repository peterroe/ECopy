use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
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
    let [x, y] = utils::get_e_copy_json().pos;
    let options = eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::vec2(116 as f32, 200 as f32)),
        // initial_window_pos: Some(egui::pos2(x, y)),
        transparent: true,
        decorated: true,
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

    {
        let s = state.clone();
        thread::spawn(move || {
            loop {
                // 等待 1 分钟
                thread::sleep(Duration::from_secs(10));
                utils::set_e_copy_json(s.lock().unwrap().clone());
            }
        });
    }

    // {
    //     thread::spawn(move || {
    //         if let Err(err) = rdev::listen(move |event| {
    //             match event.event_type {
    // rdev::EventType::ButtonPress(button) => {
    //     if button == rdev::Button::Left {
    //         mouse_state.lock().unwrap().down();
    //     }
    // }
    // rdev::EventType::ButtonRelease(button) => {
    //     if button == rdev::Button::Left {
    //         mouse_state.lock().unwrap().release()
    //     }
    // }
    //                 rdev::EventType::MouseMove { x, y } => {
    //                     print!("x,y: {}, {}", x, y);
    //                     // mouse_state.lock().unwrap().moving()
    //                 }
    //                 _ => {}
    //             };
    //         }) {
    //             print!("error");
    //         }
    //     });
    // }
    Box::new(Ecopy::new(_cc, state.clone()))
}
struct Ecopy {
    count: i32,
    show_decorated: bool,
    // clipboard: Clipboard,
    show_scroll: bool,
    json: Arc<Mutex<EcopyJson>>,
}

impl Ecopy {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        state: Arc<Mutex<EcopyJson>>,
    ) -> Self {
        font::install_fonts(&_cc.egui_ctx);

        Self {
            count: 23,
            show_decorated: true,
            show_scroll: true,
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
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("❌").clicked() {
                        _frame.close();
                    }
                    //
                    // let f = self.show_scroll ? "\u{2B07}" : "\u{2B07}";
                    if ui.button("⬇").clicked() {
                        self.show_scroll = !self.show_scroll;
                    }
                    // let res = ui.add(egui::Button::new("move").sense(Sense::click_and_drag()));
                    // self.show_scroll = false;
                    // ui.separator();
                    // {}
                    // if res.dragged() {
                    //     let egui::Pos2 { x, y } = self.share_pos.lock().unwrap().clone();
                    //     if let Some(egui::Pos2 { x: x1, y: y1 }) = ctx.pointer_hover_pos() {
                    //         if x != 0.0 && y != 0.0 {
                    //             _frame.set_window_pos(egui::Pos2 {
                    //                 x: x * 2.0 - 10.0+ x1 * 4.0,
                    //                 y: y * 2.0 - 120.0 + y1 * 4.0,
                    //             });
                    //         }
                    //     }
                        // print!("drag, {}, {}", x, y);
                        //     if (x != 0.0 && y != 0.0) {
                        //     }
                    // }
                    if ui.button("📎").clicked() {
                        // self.json.lock().unwrap().clone().pos = 
                        // _frame.set_always_on_top(self.show_decorated);
                        self.show_decorated = !self.show_decorated;
                        _frame.set_decorations(self.show_decorated);
                    }
                    // ui.separator();
                    if ui.button("🗑").clicked() {
                        utils::EcopyJson::clear(&mut self.json.lock().unwrap());
                    }
                });
                ui.separator();
            });

            let wrap = TextWrapping {
                max_rows: 2,
                break_anywhere: true,
                overflow_character: Some('…'),
                ..Default::default()
            };
            if self.show_scroll {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let data = self.json.lock().unwrap();

                        let mut o = data.clone();
                        o.data.iter_mut().for_each(|item| {
                            let words = &item.content;
                            // ui.separator();
                            let mut job = LayoutJob::single_section(
                                words.trim().replace("\n", "").to_string(),
                                TextFormat::default(),
                            );
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
            }
            // ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
            //     ui.separator();
            // });
        });
    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }
    fn on_close_event(&mut self) -> bool {
        utils::set_e_copy_json(self.json.lock().unwrap().clone());
        true
    }
}
