use std::path::PathBuf;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use egui::Ui;
use rfd::FileDialog;

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
        match &self.rx {
            None => {Ok(None)}
            Some(rx) => {
                match rx.try_recv() {
                    file@Ok(_) => {
                        self.rx = None;
                        file
                    }
                    Err(uhh) => {
                        match uhh {
                            TryRecvError::Empty => {Ok(None)}
                            error@TryRecvError::Disconnected => {
                                Err(error)
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Default for TopBar{
    fn default() -> Self {
        Self {
            rx: None,
            selected_file: None,
        }
    }
}