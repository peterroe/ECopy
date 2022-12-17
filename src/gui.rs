use arboard::Clipboard;
use eframe::{
    egui::{self, style::Margin},
    epaint::text::{LayoutJob, TextWrapping},
};
use egui::*;

pub fn run() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::vec2(100 as f32, 200 as f32)),
        transparent: true,
        decorated: false,
        initial_window_pos: Some(egui::Pos2 { x: -10.0, y: 10.0 }),
        ..Default::default()
    };
    eframe::run_native(
        "ECopy",
        options,
        Box::new(|_cc| Box::new(Content::default())),
    );
}
struct Content {
    count: i32,
    show_decorated: bool,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            count: 23,
            show_decorated: false,
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // _frame.set_decorations(false)
        let fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 0);
        let frame = egui::Frame::none()
            .fill(fill)
            .inner_margin(Margin::same(5.0));
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    // ui.add(egui::Button::new("ECopy"));
                    if ui.button("📎").clicked() {
                        self.show_decorated = !self.show_decorated;
                        _frame.set_decorations(self.show_decorated);
                    }
                    ui.add(egui::Button::new(self.count.to_string()));
                    ui.add(egui::Button::new("clear"));
                });
                ui.separator();
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
                    [
                        "Do you have any plan?",
                        "What is your name?",
                        "It's my first i join this game~",
                        "What the fuck are you saying?E tis t sfsd",
                        "what wrong?",
                        "Do you want to a fight?",
                        "随便一句话？",
                        "Do you want to a fight?",
                        "Do you want to a fight?",
                        "Do you want to a fight?",
                        "Do you want to a fight?",
                        "Do you want to a fight?",
                    ]
                    .into_iter()
                    .for_each(|words| {
                        // ui.separator();
                        let mut job =
                            LayoutJob::single_section(words.to_string(), TextFormat::default());
                        job.wrap = wrap.clone();

                        let btn_res = ui
                            .button(job)
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        if btn_res.clicked() {
                            let mut clipboard = Clipboard::new().unwrap();
                            clipboard.set_text(words).unwrap();
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
