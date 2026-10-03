#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on Windows release

use eframe::egui;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Open Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Open Editor",
        options,
        Box::new(|_cc| Ok(Box::new(OpenEditorApp::default()))),
    )
}

#[derive(Default)]
struct OpenEditorApp {
    // TODO: timeline, media bin, preview, effects panel
}

impl eframe::App for OpenEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Open Editor");
            ui.label("Lightweight VEGAS-style editor");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.label("UI skeleton ready 🔥");
            });

            ui.add_space(20.0);
            ui.label("Next steps:");
            ui.label("- Timeline");
            ui.label("- Media import");
            ui.label("- OFX host");
            ui.label("- VEGAS-style effects panel");
        });
    }
}