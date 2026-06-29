use crate::gui::top_bar::TopBar;
use eframe::{Frame, Storage};
use egui::{Context, RawInput, Ui};

pub struct GuiApp {
    top_bar: TopBar,
}

impl GuiApp {
    pub fn new() -> GuiApp {
        Self {
            top_bar: TopBar::default(),
        }
    }
}

impl eframe::App for GuiApp {
    fn logic(&mut self, ctx: &Context, frame: &mut Frame) {}

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        self.top_bar.show(ui);
        egui::Panel::left("my_left_panel").show_inside(ui, |ui| {
            ui.label("Hello World!");
        });
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {}

    fn save(&mut self, _storage: &mut dyn Storage) {}

    fn on_exit(&mut self) {}
    fn raw_input_hook(&mut self, _ctx: &Context, _raw_input: &mut RawInput) {}
}
