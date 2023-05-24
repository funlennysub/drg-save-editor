use eframe::egui::Ui;

use crate::app::EditorApp;

pub(crate) mod driller;
pub(crate) mod engineer;
pub(crate) mod gunner;
pub(crate) mod resources;
pub(crate) mod scout;

#[derive(Debug)]
pub enum Tab {
    Resources,
    Engineer,
    Gunner,
    Driller,
    Scout,
}

impl Default for Tab {
    fn default() -> Self {
        Self::Resources
    }
}

impl EditorApp {
    pub fn set_tab(&mut self, new_tab: Tab) {
        self.current_tab = new_tab;
    }

    pub fn handle_tab(&mut self, ui: &mut Ui) {
        let save = self.save.lock();

        if save.save_file.is_some() {
            drop(save);
            match self.current_tab {
                Tab::Resources => self.resources_tab(ui),
                Tab::Engineer => self.engineer_tab(ui),
                Tab::Gunner => self.gunner_tab(ui),
                Tab::Driller => self.driller_tab(ui),
                Tab::Scout => self.scout_tab(ui),
            }
        } else {
            ui.label("Please load a save file!");
        }
    }
}
