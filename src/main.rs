use crate::gui::gui_app::GuiApp;

mod gui;
mod route_finder_manager;
mod solution;

include!(concat!(env!("OUT_DIR"), "/test.rs"));

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Route finder",
        options,
        Box::new(|_cc| Ok(Box::new(GuiApp::new()))),
    )
}
