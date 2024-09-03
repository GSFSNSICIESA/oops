use rfd::FileDialog;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Default)]

struct OOPS {
    buffer: String,
    language: String,
    current_file: PathBuf,
    tmp_buffer: String,
    current_file_is_saved: bool,
    current_directory: PathBuf,
    left_side_panel_open: bool,
    right_side_panel_open: bool,
    bottom_panel_open: bool,
}

impl OOPS {
    fn new() -> Self {
        Self {
            current_file_is_saved: true,
            left_side_panel_open: true,
            right_side_panel_open: true,
            bottom_panel_open: true,
            language: "rs".to_owned(),
            ..Default::default()
        }
    }
    fn save_file(&mut self, path: String) {
        if path == self.current_file.to_str().unwrap() {
            if fs::write(self.current_file.clone(), self.buffer.clone()).is_ok() {
                self.tmp_buffer = self.buffer.clone();
                self.check_if_changed();
            };
        } else {
            // create new file
            self.current_file = PathBuf::from(path);
            self.language = self
                .current_file
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            if fs::write(self.current_file.clone(), self.buffer.clone()).is_ok() {
                self.tmp_buffer = self.buffer.clone();
                self.check_if_changed();
            };
        }
    }

    pub fn read_file(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        let _ = file.read_to_end(&mut bytes)?;

        let (result, _, _) = encoding_rs::UTF_8.decode(&bytes);
        Ok(result.into_owned())
    }
}
impl OOPS {
    fn check_if_changed(&mut self) {
        self.current_file_is_saved = self.tmp_buffer == self.buffer;
    }
}

impl eframe::App for OOPS {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            use egui::menu;

            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("open").clicked() {
                        let file = FileDialog::new().set_directory("/").pick_file();
                        if let Some(current_file) = file {
                            if let Some(extension) = current_file.extension() {
                                self.language = extension.to_str().unwrap().to_string();
                            }
                            let mut tmp_path = current_file.clone();
                            tmp_path.pop();
                            self.current_directory = tmp_path;
                            self.current_file = current_file;
                        }
                        let path_as_str = self.current_file.as_os_str().to_str().unwrap();
                        let contents = OOPS::read_file(path_as_str);
                        if let Ok(buffer) = contents {
                            self.buffer = buffer.clone();
                            self.tmp_buffer = buffer;
                        }
                        self.current_file_is_saved = true;
                        ui.close_menu();
                    }
                });
                ui.menu_button("Edit", |ui| if ui.button("settings").clicked() {});
                ui.menu_button("View", |ui| {
                    if ui.button("zen mode").clicked() {
                        self.left_side_panel_open = !self.left_side_panel_open;
                        self.right_side_panel_open = !self.right_side_panel_open;
                        self.bottom_panel_open = !self.bottom_panel_open;
                        ui.close_menu();
                    }
                    if ui.button("toggle left panel").clicked() {
                        self.left_side_panel_open = !self.left_side_panel_open;
                        ui.close_menu();
                    }
                    if ui.button("toggle right panel").clicked() {
                        self.right_side_panel_open = !self.right_side_panel_open;
                        ui.close_menu();
                    }
                    if ui.button("toggle bottom panel").clicked() {
                        self.bottom_panel_open = !self.bottom_panel_open;
                        ui.close_menu();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("open documentation").clicked() {
                        // …
                        ui.close_menu();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("status bar").show_animated(
            ctx,
            self.bottom_panel_open,
            |ui| {
                if ui.button("Save File").clicked() {
                    if self.current_file.exists() {
                        self.save_file(self.current_file.to_str().unwrap().to_string());
                    } else if let Some(file) = FileDialog::new().set_directory("/").save_file() {
                        let mut tmp_path = file.clone();
                        tmp_path.pop();
                        self.current_directory = tmp_path;
                        self.current_file = file;
                        self.save_file(self.current_file.to_str().unwrap().to_string());
                    }
                }
                if ui.button("check if the file is saved").clicked() {
                    self.check_if_changed();
                }
            },
        );
        egui::SidePanel::left("file navigation").show_animated(
            ctx,
            self.left_side_panel_open,
            |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if let Ok(files) = fs::read_dir(self.current_directory.clone()) {
                        let mut sorted_files: Vec<PathBuf> =
                            files.map(|file| file.unwrap().path()).collect();
                        sorted_files.sort_by(|a, b| {
                            if a.is_dir() != b.is_dir() {
                                // meaning, if
                                return b.is_dir().cmp(&a.is_dir());
                            }

                            a.as_os_str().cmp(b.as_os_str())
                        });
                        for file in sorted_files {
                            let path_as_str = file.as_os_str().to_str().unwrap();

                            let label: egui::Label = if file.is_dir() {
                                egui::Label::new(
                                    egui::RichText::new(
                                        file.file_name().unwrap().to_str().unwrap(),
                                    )
                                    .underline(),
                                )
                                .sense(egui::Sense::click())
                            } else {
                                egui::Label::new(file.file_name().unwrap().to_str().unwrap())
                                    .sense(egui::Sense::click())
                            };
                            if ui.add(label).clicked() {
                                self.current_file = file.clone();
                                self.language =
                                    file.extension().unwrap().to_str().unwrap().to_string();
                                let contents = OOPS::read_file(path_as_str);
                                match contents {
                                    Ok(buffer) => {
                                        self.buffer = buffer.clone();
                                        self.tmp_buffer = buffer;
                                    }
                                    Err(e) => {
                                        println!("error: {}", e);
                                    }
                                }
                            }
                        }
                    }
                });
            },
        );
        egui::SidePanel::right("accessories").show_animated(
            ctx,
            self.right_side_panel_open,
            |ui| {
                if !self.current_file_is_saved {
                    ui.label("⏳");
                }
            },
        );

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                ui.collapsing("Theme", |ui| {
                    ui.group(|ui| {
                        theme.ui(ui);
                        theme.clone().store_in_memory(ui.ctx());
                    });
                });

                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    let mut layout_job = egui_extras::syntax_highlighting::highlight(
                        ui.ctx(),
                        &theme,
                        string,
                        &self.language,
                    );
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };

                let text_edit = egui::TextEdit::multiline(&mut self.buffer)
                    .code_editor()
                    .frame(false)
                    .layouter(&mut layouter);
                let available_size = ui.available_size();
                ui.add_sized(available_size, text_edit);
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native("OOPS", options, Box::new(|_cc| Ok(Box::new(OOPS::new()))));
}
