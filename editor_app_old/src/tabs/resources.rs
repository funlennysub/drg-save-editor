use eframe::{
    egui::{Layout, Ui},
    emath::Align,
};
use egui_extras::Size;
use egui_grid::GridBuilder;

use crate::app::EditorApp;

impl EditorApp {
    pub(crate) fn resources_tab(&mut self, ui: &mut Ui) {
        let mut save = self.save.lock();

        if let Some(save) = &mut save.save_file {
            let minerals_grid = GridBuilder::new()
                .new_row(Size::exact(25.))
                .cell(Size::remainder())
                .new_row(Size::exact(60.))
                .cells(Size::exact(110.), 3)
                .new_row(Size::exact(60.))
                .cells(Size::exact(110.), 3);
            let brewing_grid = GridBuilder::new()
                .new_row(Size::exact(25.))
                .cell(Size::remainder())
                .new_row(Size::exact(60.))
                .cells(Size::exact(110.), 3)
                .new_row(Size::exact(60.))
                .cell(Size::exact(110.));
            let misc_grid = GridBuilder::new()
                .new_row(Size::exact(25.))
                .cell(Size::remainder())
                .new_row(Size::exact(60.))
                .cells(Size::exact(110.), 3)
                .new_row(Size::exact(60.))
                .cells(Size::exact(110.), 3);

            let main_grid = GridBuilder::new()
                .new_row(Size::exact(150.))
                .cell(Size::remainder())
                .nest(minerals_grid)
                .new_row(Size::exact(150.))
                .cell(Size::remainder())
                .nest(brewing_grid)
                .new_row(Size::exact(150.))
                .cell(Size::remainder())
                .nest(misc_grid);

            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 15.;

                main_grid.show(ui, |mut grid| {
                    grid.cell(|ui| {
                        ui.label("Minerals:");
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Magnite", &mut save.minerals.magnite, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Bismor", &mut save.minerals.bismor, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Croppa", &mut save.minerals.croppa, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Umanite", &mut save.minerals.umanite, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Jadiz", &mut save.minerals.jadiz, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Pearl", &mut save.minerals.enor_pearl, ui);
                    });

                    grid.cell(|ui| {
                        ui.label("Brewing:");
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Starch nut", &mut save.brewing.starch_nut, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Yeast cone", &mut save.brewing.yeast_cone, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Malt star", &mut save.brewing.malt_star, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Barley bulb", &mut save.brewing.barley_bulb, ui);
                    });

                    grid.cell(|ui| {
                        ui.label("Miscellaneous:");
                    });
                    grid.cell(|ui| {
                        self.value_widget_i32("Credits", &mut save.miscellaneous.credits, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_i32(
                            "Perk points",
                            &mut save.miscellaneous.perk_points,
                            ui,
                        );
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32(
                            "Error cubes",
                            &mut save.miscellaneous.error_cubes,
                            ui,
                        );
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Data cells", &mut save.miscellaneous.data_cells, ui);
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32(
                            "Blank cores",
                            &mut save.miscellaneous.blank_cores,
                            ui,
                        );
                    });
                    grid.cell(|ui| {
                        self.value_widget_f32("Phazyonite", &mut save.miscellaneous.phazyonite, ui);
                    });
                });
            });
        }
    }
}
