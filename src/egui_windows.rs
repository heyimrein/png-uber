use egui_macroquad::egui;
use crate::MainState;

pub fn test_window(ui: &mut egui::Ui, state: &mut MainState) {
    if ui.button("Open File").clicked() {
        let wrk_dir_option = std::env::current_dir().unwrap();
        let wrk_dir = match wrk_dir_option.to_str() {
            None => { return; }
            Some(path) => { path }
        };

        let path = match tinyfiledialogs::open_file_dialog(
            "Open File",
            wrk_dir,
            None,
        ) {
            None => { return; }
            Some(path) => { path }
        };

        state.debug_path = path;
    }
}

