use std::time::Duration;
use eframe::{Frame, Storage};
use egui::{Context, RawInput, Ui, Visuals};

pub struct GuiApp{

}

impl GuiApp {
    pub fn new() -> GuiApp {
        todo!()
    }
}

impl eframe::App for GuiApp{
    fn logic(&mut self, ctx: &Context, frame: &mut Frame) {
        todo!()
    }

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        todo!()
    }

    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        todo!()
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        todo!()
    }

    fn on_exit(&mut self) {
        todo!()
    }

    fn auto_save_interval(&self) -> Duration {
        todo!()
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        todo!()
    }

    fn persist_egui_memory(&self) -> bool {
        todo!()
    }

    fn raw_input_hook(&mut self, _ctx: &Context, _raw_input: &mut RawInput) {
        todo!()
    }
}