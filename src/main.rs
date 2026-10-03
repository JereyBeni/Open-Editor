#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console on Windows release

mod plugins;

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
    // plugin_manager: plugins::PluginManager,
}

impl eframe::App for OpenEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Open Editor");
            ui.label("Lightweight VEGAS-style editor");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Status:");
                ui.colored_label(egui::Color32::LIGHT_GREEN, "UI + Plugin system skeleton ready 🔥");
            });

            ui.add_space(12.0);
            ui.label("Current priorities:");
            ui.label("1. DirectFX effects support (legacy Vegas)");
            ui.label("2. OFX host → BCC + SapphireFX first");

            ui.add_space(20.0);
            ui.label("Next steps:");
            ui.label("- Timeline");
            ui.label("- Media import");
            ui.label("- Actual DirectFX / OFX loading");
        });
    }
}