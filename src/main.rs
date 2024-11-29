mod ui;

use ui::gui::AppCore;


fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Limiinal", native_options, Box::new(|cc| Ok(Box::new(AppCore::new(cc)))));
}

