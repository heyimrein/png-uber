mod egui_windows;

use std::time::SystemTime;
use egui_macroquad::egui;
use egui_macroquad::egui::{pos2};
use macroquad::prelude::*;


pub struct MainState {
    show_ui: bool,
    path: String,
    time: f32,
    dt_helper: SystemTime
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            show_ui: true,
            path: "".to_string(),
            time: 0.,
            dt_helper: SystemTime::now()
        }
    }
}

#[derive(Clone)]
pub struct Tuber {
    texture: Texture2D,
    params: DrawTextureParams
}

impl Tuber {
    pub fn new() -> Tuber {
        Tuber {
            texture: Texture2D::empty(),
            params: DrawTextureParams {
                dest_size: Some(vec2(0., 0.)),
                ..DrawTextureParams::default()
            }
        }
    }

    pub fn set_texture(&mut self, texture: &Texture2D) {
        let ratio = texture.width() / texture.height();
        self.params.dest_size = Some(vec2(200. * ratio, 200.));
        self.texture = texture.clone();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "⚡PNG Uber!⚡".to_string(),
        window_resizable: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    let mut main_state = MainState::new();
    let mut tuber = Tuber::new();

    loop {
        main_state.time += main_state.dt_helper
            .elapsed()
            .unwrap()
            .as_secs_f32();

        clear_background(BLACK);

        // Handle input
        if is_key_pressed(KeyCode::H) {
            main_state.show_ui = !main_state.show_ui;
        }

        // Draw main graphics
        let tuber_temp = tuber.clone();
        draw_texture_ex(
            tuber_temp.texture,
            window_conf().window_width as f32 / 2. - 100.,
            window_conf().window_height as f32 / 2. - 100. + ((&main_state.time * 10.).sin() * 25.),
            WHITE,
            tuber_temp.params
        );
        draw_text(
            main_state.path.as_str(),
            200.,
            50.,
            12.,
            WHITE
        );

        // Handle and draw UI
        let old_path = main_state.path.to_owned();
        if main_state.show_ui {
            egui_macroquad::ui(|ctx| {
                egui::Window::new("Image Settings")
                    .fixed_pos(pos2(0., 0.))
                    .fixed_size(egui::vec2(150., 100.))
                    .show(
                        ctx,
                        |ui| {
                            egui_windows::test_window(ui, &mut main_state)
                        }
                    );
            });
            egui_macroquad::draw();
        }
        if main_state.path != old_path {
            tuber.set_texture(&load_texture(main_state.path.as_str()).await.unwrap());
        }

        main_state.dt_helper = SystemTime::now();
        next_frame().await;
    }
}
