#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

#[derive(Default)]
pub struct AppCore { }

impl AppCore {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for AppCore {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Welcome to Limiinal");
           ui.label("A secure and private messaging app.");
       });
   }
}
