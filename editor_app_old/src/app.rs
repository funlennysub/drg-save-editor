// TODO: handle errors

use std::{
    num::{IntErrorKind, ParseFloatError, ParseIntError},
    ops::RangeInclusive,
    path::PathBuf,
    sync::Arc,
};

use editor_core::{registry, save_file::SaveFile};
use eframe::{
    egui::{self, CentralPanel, Layout, RichText, SidePanel, TextEdit, TopBottomPanel, Ui},
    emath::Align,
};
use egui_extras::Size;
use egui_grid::GridBuilder;
use parking_lot::Mutex;

use crate::{error::Error, tabs::Tab};

macro_rules! full_btn {
    ($ui:expr, $text:expr) => {
        $ui.add_sized(
            [$ui.available_width(), $ui.available_height()],
            egui::Button::new($text),
        )
    };
}

#[derive(Default)]
pub(crate) struct Idk {
    pub(crate) gvas_promise: Option<poll_promise::Promise<Result<SaveFile, Error>>>,
    _error: Option<&'static Error>,
}

#[derive(Default)]
pub(crate) struct Save {
    pub(crate) save_file: Option<SaveFile>,
    pub(crate) save_path: Option<PathBuf>,
}

#[derive(Default)]
pub struct EditorApp {
    pub(crate) save: Arc<Mutex<Save>>,
    pub(crate) current_tab: Tab,
    pub(crate) thing: Idk,
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
            self.input_i32(value, 0..=registry::MAX_I32, ui);
        });
    }

    pub fn value_widget_f32(&self, title: impl Into<RichText>, value: &mut f32, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.monospace(title.into());
            self.input_f32(value, 0.0..=registry::MAX_F32, ui);
        });
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
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
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
