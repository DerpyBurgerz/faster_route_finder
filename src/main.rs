use eframe::UserEvent;
use winit::event_loop::{EventLoop};
use crate::gui::gui_app::GuiApp;

mod gui;

fn main() -> eframe::Result<()> {
    // GUI stuff
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    let eventloop = EventLoop::<UserEvent>::with_user_event().build()?;

    let gui_app = GuiApp::new();

    let mut gui_app = eframe::create_native(
        "Route finder",
        options,
        Box::new(|_cc| Ok(Box::new(gui_app))),
        &eventloop,
    );

    eventloop.run_app(&mut gui_app)?;
    Ok(())
}
