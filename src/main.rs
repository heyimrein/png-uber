mod egui_windows;

use std::time::SystemTime;
use macroquad::prelude::*;


/// Main state object containing app-wide data
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


/// PNG-Tuber avatar object
#[derive(Clone)]
pub struct Tuber {
    texture: Texture2D,
    scale: f32,
    offset: Vec2,
    params: DrawTextureParams
}

impl Tuber {
    pub fn new() -> Tuber {
        Tuber {
            texture: Texture2D::empty(),
            scale: 1.,
            offset: vec2(0., 0.),
            params: DrawTextureParams {
                dest_size: Some(vec2(0., 0.)),
                ..DrawTextureParams::default()
            }
        }
    }

    pub fn set_texture(&mut self, texture: &Texture2D) {
        let res = vec2(texture.width(), texture.height());
        let ratio = res.x / res.y;
        let height = 200. * self.scale;
        let width = height * ratio;

        self.params.dest_size = Some(vec2(width, height));
        self.offset = vec2(width / 2., height / 2.);
        self.texture = texture.clone();
    }
}


/// Init window config
fn window_conf() -> Conf {
    Conf {
        window_title: "⚡PNG Uber!⚡".to_string(),
        window_resizable: false,
        ..Default::default()
    }
}


/// App entry-point
#[macroquad::main(window_conf)]
async fn main() {
    let mut main_state = MainState::new();
    let mut tuber = Tuber::new();

    // App loop
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
            (window_conf().window_width as f32 / 2.) - tuber.offset.x,
            (window_conf().window_height as f32 / 2.) - tuber.offset.y
                + ((&main_state.time * 10.).sin() * 25.),
            WHITE,
            tuber_temp.params
        );
        draw_text(
            main_state.path.as_str(),
            200.,
            50.,
            15.,
            WHITE
        );

        // Handle and draw UI
        let old_path = main_state.path.to_owned();
        if main_state.show_ui {
            egui_macroquad::ui(|ctx| {
                egui_windows::draw_windows(ctx, &mut main_state);
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
