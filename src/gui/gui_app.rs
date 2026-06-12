use eframe::{Frame, Storage};
use egui::{Context, RawInput, Ui};

pub struct GuiApp{

}

impl GuiApp {
    pub fn new() -> GuiApp {
        Self{}
    }
}

impl eframe::App for GuiApp{
    fn logic(&mut self, ctx: &Context, frame: &mut Frame) {
    }

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            });
        });

        egui::Panel::left("my_left_panel").show_inside(ui, |ui| {
            ui.label("Hello World!");
        });
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
    }

    fn on_exit(&mut self) {
    }
    fn raw_input_hook(&mut self, _ctx: &Context, _raw_input: &mut RawInput) {
    }
}