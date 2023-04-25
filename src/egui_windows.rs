use egui_macroquad::egui;
use egui_macroquad::egui::{Context, pos2};
use crate::MainState;


/// Render/Handle UI windows
pub fn draw_windows(ctx: &Context, main_state: &mut MainState) {
    egui::Window::new("Image Settings")
        .fixed_pos(pos2(0., 0.))
        .fixed_size(egui::vec2(150., 100.))
        .show(
            ctx,
            |ui| {
                test_window(ui, main_state);
            }
        );
}


/// Render the test window used for debugging
pub fn test_window(ui: &mut egui::Ui, main_state: &mut MainState) {
    if ui.button("Open File").clicked() {
        let wrk_dir_option = std::env::current_dir().unwrap();
        let wrk_dir = match wrk_dir_option.to_str() {
            None => { return; }
            Some(path) => { path }
        };

        let path = match tinyfiledialogs::open_file_dialog(
            "Open File",
            wrk_dir,
            Some((&["*.png"], "Image file (.png)")),
        ) {
            None => { return; }
            Some(path) => { path }
        };

        main_state.path = path;
    }
}

