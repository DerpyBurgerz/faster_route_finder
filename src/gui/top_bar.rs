use std::path::PathBuf;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use egui::Ui;
use rfd::FileDialog;
#[derive(Default)]
pub struct TopBar {
    rx: Option<Receiver<Option<PathBuf>>>,
    selected_file: Option<PathBuf>
}


impl TopBar {
    pub fn show(&mut self, ui: &mut Ui) {
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("New simulation", |ui| {
                    if ui.button("Exit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    self.file_button(ui);
                });
                ui.add_space(16.0);
            });
        });
    }

    fn file_button(&mut self, ui: &mut Ui) {
        if ui.button("From file").clicked() {
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let file = FileDialog::new().pick_file();
                let _ = tx.send(file);
            });
            self.rx = Some(rx);
        }
    }

    fn take_selected_file(&mut self) -> Result<Option<PathBuf>, TryRecvError> {
        if let Some(rx) = self.rx.take() {
            rx.try_recv()
        } else {
            Ok(None)
        }
    }
}
