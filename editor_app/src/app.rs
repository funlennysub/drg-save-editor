// TODO: handle errors

use std::{
    num::{IntErrorKind, ParseFloatError, ParseIntError},
    ops::RangeInclusive,
    path::PathBuf,
    sync::Arc,
};

use editor_core::save_file::SaveFile;
use eframe::{
    egui::{self, CentralPanel, Layout, RichText, SidePanel, TextEdit, TopBottomPanel, Ui},
    emath::Align,
};
use egui_extras::Size;
use egui_grid::GridBuilder;
use parking_lot::Mutex;

use crate::error::Error;

macro_rules! full_btn {
    ($ui:expr, $text:expr) => {
        $ui.add_sized($ui.available_size(), egui::Button::new($text))
    };
}

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

#[derive(Default)]
struct Idk {
    gvas_promise: Option<poll_promise::Promise<Result<SaveFile, Error>>>,
    _error: Option<&'static Error>,
}

#[derive(Default)]
struct Save {
    save_file: Option<SaveFile>,
    save_path: Option<PathBuf>,
}

#[derive(Default)]
pub struct EditorApp {
    save: Arc<Mutex<Save>>,
    current_tab: Tab,
    thing: Idk,
}

macro_rules! input {
    ($ty:ty, $val:expr, $range:expr, $ui:expr) => {
        let mut value_str = $val.to_string();
        let input = $ui.add_sized(
            [110., 25.],
            TextEdit::singleline(&mut value_str).vertical_align(Align::Center),
        );

        if input.changed() {
            *$val = value_str
                .parse::<$ty>()
                .map(|v| v.clamp(*$range.start(), *$range.end()))
                .unwrap_or_default();
        }
    };
}

impl EditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .font_data
            .iter_mut()
            .for_each(|font| font.1.tweak.scale = 1.3);

        cc.egui_ctx.set_fonts(fonts);

        Self {
            save: Arc::new(Mutex::new(Save::default())),
            current_tab: Tab::Resources,
            thing: Idk::default(),
        }
    }

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

    fn input_i32(&self, value: &mut i32, range: RangeInclusive<i32>, ui: &mut Ui) {
        let mut value_str = value.to_string();
        let input = ui.add_sized(
            [110., 25.],
            TextEdit::singleline(&mut value_str).vertical_align(Align::Center),
        );

        if input.changed() {
            *value = value_str
                .parse::<i32>()
                .map(|v| v.clamp(*range.start(), *range.end()))
                .unwrap_or_else(|n| match n.kind() {
                    IntErrorKind::InvalidDigit => *value,
                    IntErrorKind::PosOverflow => *range.end(),
                    _ => 0,
                });
        }
    }

    fn input_f32(&self, value: &mut f32, range: RangeInclusive<f32>, ui: &mut Ui) {
        let mut value_str = format!("{value:.0}");
        let input = ui.add_sized(
            [110., 25.],
            TextEdit::singleline(&mut value_str).vertical_align(Align::Center),
        );

        if input.changed() {
            *value = value_str
                .parse::<f32>()
                .map(|v| v.clamp(*range.start(), *range.end()))
                .unwrap_or_default();
        }
    }

    pub fn value_widget_i32(&self, title: impl Into<RichText>, value: &mut i32, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.monospace(title.into());
            self.input_i32(value, 0..=268_435_456, ui);
        });
    }

    pub fn value_widget_f32(&self, title: impl Into<RichText>, value: &mut f32, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.monospace(title.into());
            self.input_f32(value, 0.0..=268_435_456.0, ui);
        });
    }

    fn resources_tab(&mut self, ui: &mut Ui) {
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
    fn engineer_tab(&mut self, ui: &mut Ui) {
        ui.label("eng");
    }
    fn gunner_tab(&mut self, ui: &mut Ui) {
        ui.label("gun");
    }
    fn driller_tab(&mut self, ui: &mut Ui) {
        ui.label("dril");
    }
    fn scout_tab(&mut self, ui: &mut Ui) {
        ui.label("sc");
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);

        if let Some(promise) = &self.thing.gvas_promise {
            if let Some(gvas) = promise.ready() {
                match gvas {
                    Ok(save_file) => {
                        let mut save = self.save.lock();
                        save.save_file = Some(save_file.clone());
                    }
                    Err(_err) => {}
                }
                self.thing.gvas_promise = None;
            }
        }

        TopBottomPanel::top("top-menu")
            .exact_height(40.)
            .resizable(false)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.x = 5.;
                GridBuilder::new()
                    .new_row(Size::relative(1.))
                    .cells(Size::exact(50.), 3)
                    .with_layout(Layout::left_to_right(Align::Center))
                    .show(ui, |mut grid| {
                        grid.cell(|ui| {
                            if ui
                                .add_sized(
                                    [ui.available_width(), ui.available_height() - 10.],
                                    egui::Button::new("Load"),
                                )
                                .clicked()
                            {
                                let file = rfd::FileDialog::new()
                                    .add_filter("Save file", &["sav"])
                                    .pick_file();
                                if let Some(save_path) = file {
                                    let mut save = self.save.lock();
                                    save.save_path = Some(save_path.clone());

                                    self.thing.gvas_promise =
                                        Some(poll_promise::Promise::spawn_thread(
                                            "parse gvas",
                                            move || -> Result<SaveFile, Error> {
                                                Ok(editor_core::save_file::SaveFile::from_gvas(
                                                    &editor_core::read_gvas(&save_path)?,
                                                )?)
                                            },
                                        ));
                                }
                            }
                        });
                        grid.cell(|ui| {
                            if ui
                                .add_sized(
                                    [ui.available_width(), ui.available_height() - 10.],
                                    egui::Button::new("Save"),
                                )
                                .clicked()
                            {}
                        });
                        grid.cell(|ui| {
                            if ui
                                .add_sized(
                                    [ui.available_width(), ui.available_height() - 10.],
                                    egui::Button::new("Reset"),
                                )
                                .clicked()
                            {}
                        });
                    });
            });

        SidePanel::left("selector")
            .resizable(false)
            .show(ctx, |ui| {
                GridBuilder::new()
                    .rows_as_columns(true)
                    .new_row(Size::remainder())
                    .cells(Size::exact(60.), 5)
                    .show(ui, |mut grid| {
                        grid.cell(|ui| {
                            if full_btn!(ui, "Resources").clicked() {
                                self.set_tab(Tab::Resources)
                            }
                        });
                        grid.cell(|ui| {
                            if full_btn!(ui, "Engineer").clicked() {
                                self.set_tab(Tab::Engineer)
                            }
                        });
                        grid.cell(|ui| {
                            if full_btn!(ui, "Gunner").clicked() {
                                self.set_tab(Tab::Gunner)
                            }
                        });
                        grid.cell(|ui| {
                            if full_btn!(ui, "Driller").clicked() {
                                self.set_tab(Tab::Driller)
                            }
                        });
                        grid.cell(|ui| {
                            if full_btn!(ui, "Scout").clicked() {
                                self.set_tab(Tab::Scout)
                            }
                        });
                    })
            });
        CentralPanel::default().show(ctx, |ui| self.handle_tab(ui));
    }
}
