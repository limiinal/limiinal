#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{column, text, Column, container, row};
use iced::{Element, Length};


#[derive(Default)]
pub struct AppCore {
    window_width: f32,
    window_height: f32,
 }

#[derive(Debug, Clone)]
pub enum Message {
    Resize(f32, f32),
}

impl AppCore {

    pub fn new(initial_width: f32, initial_height: f32) -> Self {
        Self {
            window_width: initial_width,
            window_height: initial_height,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Resize(width, height) => {
                self.window_width = width;
                self.window_height = height;
                println!("Resized! New width: {}", self.window_width);
            }
        }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            text!("Welcome to Limiinal!"),
            self.containers(),
        ]
    }

    pub fn containers(&self) -> Element<Message> {
        println!("Window Width: {}", self.window_width);
        row![
            container("Taskbar, Home GUI")
                .padding(10)
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(900.0)) 
                .style(container::rounded_box),
            container("All Message Chats")
                .padding(10)
                .width(Length::Fixed(300.0))
                .height(Length::Fixed(900.0)) 
                .style(container::rounded_box),
            container("Active message UI")
                .padding(10)
                .width(Length::Fixed(900.0))
                .height(Length::Fixed(900.0)) 
                .style(container::rounded_box),
        ]
        .spacing(50)
        .into()
    }
}