
use eframe::egui;
use egui::*;
pub fn run()  {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        always_on_top: true,
        initial_window_size: Some(egui::vec2(200 as f32, 300 as f32)),
        transparent: true,
        decorated: false,
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
}

impl Default for Content {
    fn default() -> Self {
        Self {
            count: 23
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // _frame.set_decorations(false)
        let fill = egui::Color32::from_rgba_premultiplied(0, 0, 0, 0);
        let frame = egui::Frame::none().fill(fill);
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
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
                        "Do you want to a fight?",
                        "Do you want to a fight?",
                        "Do you want to a fight?",
                        "Do you want to a fight?",
                        "Do you want to a fight?",
                        "Do you want to a fight?"
                    ].map(|words| {
                        ui.separator();
                        ui.button(words)
                        .on_hover_cursor(egui::CursorIcon::PointingHand);
                        // if btn_res.clicked() {
                        //     self.count += 1;
                        // }
                    });
                });
                ui.label(format!("hello, {}", self.count));
            });
    }
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }
}
