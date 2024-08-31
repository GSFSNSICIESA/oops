use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

#[derive(Default)]
struct OOPS {
    buffer: String,
    current_file: PathBuf,
    tmp_buffer: String,
    current_file_is_saved: bool,
    current_path: PathBuf,
}
impl OOPS {
    fn new() -> Self {
        Self {
            current_file_is_saved: true,
            ..Default::default()
        }
    }
}
impl OOPS {
    fn check_if_changed(&mut self) {
        if self.tmp_buffer != self.buffer {
            self.current_file_is_saved = false;
        } else {
            self.current_file_is_saved = true;
        }
    }
}

impl eframe::App for OOPS {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            use egui::menu;

            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("open").clicked() {
                        let file = FileDialog::new()
                            .set_directory("/")
                            .pick_file();
                        match file {
                            Some(current_file) => {
                                let mut tmp_path = current_file.clone();
                                tmp_path.pop();
                                self.current_path = tmp_path;
                                self.current_file = current_file;
                            }
                            None => {}
                        }
                        let contents = fs::read_to_string(self.current_file.clone());
                        match contents {
                            Ok(buffer) => {
                                self.buffer = buffer.clone();
                                self.tmp_buffer = buffer;
                            }
                            Err(_) => {}
                        }
                        self.current_file_is_saved = true;
                    }
                });
                ui.menu_button("Edit", |ui| if ui.button("settings").clicked() {});
                ui.menu_button("View", |ui| {
                    if ui.button("hide side panel").clicked() {
                        // …
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("open documentation").clicked() {
                        // …
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("status bar").show(ctx, |ui| {
            // ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::BottomUp), |ui| {

            if ui
                .button("Save file to your computer to get your work not wasted")
                .clicked()
            {
                fs::write(self.current_file.clone(), self.buffer.clone())
                    .expect("Unable to write file");
                self.tmp_buffer = self.buffer.clone();
                self.check_if_changed();
            }
            if ui.button("check if the file is saved").clicked() {
                // compare buffer with tmp_buffer
                self.check_if_changed();
            }
        });
        // });
        egui::SidePanel::left("file navigation").show(ctx, |ui| {
            if let Ok(files) = fs::read_dir(self.current_path.clone()) {
                for file in files {
                    // if file is directory
                    
                    match file {
                        Ok(file) => {
                            let label = egui::Label::new(file.file_name().to_str().unwrap())
                                .sense(egui::Sense::click());
                            if ui.add(label).clicked() {
                                self.current_file = file.path();
                                let contents = fs::read_to_string(self.current_file.clone());
                                match contents {
                                    Ok(buffer) => {
                                        self.buffer = buffer.clone();
                                        self.tmp_buffer = buffer;
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                        Err(_) => {}
                    }
                }
            }
        });
        egui::SidePanel::right("accessories").show(ctx, |ui| {
            if !self.current_file_is_saved {
                ui.label("⏳");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let text_edit = egui::TextEdit::multiline(&mut self.buffer)
                    .code_editor()
                    .frame(false);
                let available_size = ui.available_size();
                ui.add_sized(available_size, text_edit);
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let _ = eframe::run_native("OOPS", options, Box::new(|_cc| Ok(Box::new(OOPS::new()))));
}
