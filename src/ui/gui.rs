#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{column, text, Column};

#[derive(Default)]
pub struct AppCore { }

#[derive(Debug, Clone)]
pub struct Message {}

impl AppCore {
    pub fn update(&mut self, _message: Message) {
    }

    pub fn view(&self) -> Column<Message> {
        column! [
            text!("Welcome to Limiinal!"),
        ]

    }

}

