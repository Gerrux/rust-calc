#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod calculator;
mod icon;

use app::CalculatorApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let icon = egui::IconData {
        rgba: icon::ICON_RGBA.to_vec(),
        width: icon::ICON_WIDTH,
        height: icon::ICON_HEIGHT,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([360.0, 560.0])
            .with_min_inner_size([350.0, 540.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_title("Rust Calculator")
            .with_icon(icon),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Rust Calculator",
        options,
        Box::new(|cc| Ok(Box::new(CalculatorApp::new(cc)))),
    )
}
