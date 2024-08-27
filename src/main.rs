struct MyApp {
    buffer: String,
    cursor_position: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            buffer: "Arthur".to_owned(),
            cursor_position: 1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            use egui::menu;

        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                // …
                }
            });
        });

        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let text_edit = egui::TextEdit::multiline(&mut self.buffer).code_editor().desired_width(f32::INFINITY);
                let available_size = ui.available_size();
                ui.add_sized(available_size, text_edit);
            });
            
        });
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
        Box::new(|_cc| {
            Ok(Box::new(MyApp::default()))
        }),
    );
}
