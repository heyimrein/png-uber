mod egui_windows;

use egui_macroquad::egui;
use macroquad::prelude::*;


pub struct MainState {
    path: String
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            path: "".to_string()
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "PNG UBER".to_string(),
        window_resizable: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut state = MainState::new();

    loop {
        clear_background(BLACK);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("egui test :>").show(
                ctx,
                |ui| { egui_windows::test_window(ui, &mut state) }
            );
        });
        egui_macroquad::draw();

        draw_text(state.path.as_str(), 50., 50., 12., WHITE);

        next_frame().await;
    }
}
