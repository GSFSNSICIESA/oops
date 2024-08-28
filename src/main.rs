struct MyApp {
    buffer: String,
    cursor_position: i32,
    open_window_activated: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            buffer: "Karim Benzema".to_owned(),
            cursor_position: 1,
            open_window_activated: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            use egui::menu;

            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("open").clicked() {
                        self.open_window_activated = true;
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("settings").clicked() {
                        // …
                    }
                });
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
        egui::TopBottomPanel::bottom("status bar").show(ctx, |ui| {});
        egui::SidePanel::left("file navigation").show(ctx, |ui| {});
        egui::SidePanel::right("accessories").show(ctx, |ui| {});

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let text_edit = egui::TextEdit::multiline(&mut self.buffer)
                    .code_editor().frame(false);
                let available_size = ui.available_size();
                ui.add_sized(available_size, text_edit);
            });
        });
        let mut temp_act = self.open_window_activated;
            egui::Window::new("My Window")
                .open(&mut temp_act)
                .show(ctx, |ui| {
                    egui::TopBottomPanel::bottom("buttons").show_inside(ui, |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {

                            ui.button("open");
                            if ui.button("close").clicked(){
                                self.open_window_activated = false;
                            }
                            
                            
                        });
                    });
                    egui::SidePanel::left("navigation").show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {});
                    });
                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {});

                        ui.label("Hello World!");
                    });
                    
                });
                self.open_window_activated &= temp_act;
        }
    }


// main function
fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "OOPS",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    );
}
