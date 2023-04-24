mod egui_windows;

use egui_macroquad::egui;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "PNG UBER".to_string(),
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("egui test :>").show(
                ctx,
                egui_windows::test_window
            );
        });
        egui_macroquad::draw();

        next_frame().await;
    }
}
