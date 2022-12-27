use eframe::{egui::{self, FontDefinitions, FontFamily, TextStyle}, epaint::{Shadow, Stroke, Color32}};
use egui::{FontData, FontId};
use FontFamily::{Monospace, Proportional};

pub fn install_fonts(egui_ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "LXGWWenKai-Regular".to_owned(),
        FontData::from_static(include_bytes!("../res/LXGWWenKai-Regular.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());

    egui_ctx.set_fonts(fonts);

    let mut style = (*egui_ctx.style()).clone();
    // style.text_styles = [
    //     (TextStyle::Heading, FontId::new(20.0, Proportional)),
    //     (TextStyle::Body, FontId::new(20.0, Proportional)),
    //     (TextStyle::Monospace, FontId::new(18.0, Monospace)),
    //     (TextStyle::Button, FontId::new(20.0, Proportional)),
    //     (TextStyle::Small, FontId::new(18.0, Proportional)),
    // ]
    // .into();
    // style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    // style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    // style.visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    // style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    // egui_ctx.set_style(style);
}
