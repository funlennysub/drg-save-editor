use std::path::PathBuf;

use app::EditorApp;

use editor_core::{read_gvas, save_file::SaveFile};
use eframe::{egui, NativeOptions};

mod app;
pub mod error;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 600.0)),
        resizable: false,
        ..Default::default()
    };

    // let mut gvas = read_gvas(&PathBuf::from(
    //     r"C:\Users\funlennysub\Documents\cockding\drg-save-editor\00000000000000000_Player.sav",
    // ))
    // .unwrap();
    // let mut save = SaveFile::from_gvas(&gvas).unwrap();
    // // save.miscellaneous.credits = i32::MAX;
    // save.save(
    //     &mut gvas,
    //     &PathBuf::from(
    //         r"C:\Program Files (x86)\Steam\steamapps\common\Deep Rock Galactic\FSD\Saved\SaveGames\76561198282694357_Player.sav",
    //     ),
    // );

    eframe::run_native(
        "DRG Save Editor",
        options,
        Box::new(|cc| Box::new(EditorApp::new(cc))),
    )
    // Ok(())
}
