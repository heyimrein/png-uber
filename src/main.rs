mod egui_windows;

use egui_macroquad::egui;
use egui_macroquad::egui::{pos2};
use macroquad::prelude::*;


pub struct MainState {
    show_ui: bool,
    debug_path: String,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            show_ui: true,
            debug_path: "".to_string()
        }
    }
}

pub struct Tuber {

}

fn window_conf() -> Conf {
    Conf {
        window_title: "PNG uber".to_string(),
        window_resizable: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut state = MainState::new();

    loop {
        clear_background(BLACK);

        // Handle input
        if is_key_pressed(KeyCode::H) {
            state.show_ui = !state.show_ui;
        }

        // Draw main graphics
        draw_text(
            state.debug_path.as_str(),
            200.,
            50.,
            12.,
            WHITE
        );

        // Handle and draw UI
        if state.show_ui {
            egui_macroquad::ui(|ctx| {
                egui::Window::new("egui test :>")
                    .fixed_pos(pos2(0., 0.))
                    .fixed_size(egui::vec2(150., 100.))
                    .show(
                        ctx,
                        |ui| { egui_windows::test_window(ui, &mut state) }
                    );
            });
            egui_macroquad::draw();
        }

        next_frame().await;
    }
}
