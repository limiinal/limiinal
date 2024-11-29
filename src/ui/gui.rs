#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{column, text, Column, container, row};
use iced::{Element, Length, Color, Theme};


#[derive(Default)]
pub struct AppCore { }

#[derive(Debug, Clone)]
pub struct Message {}

impl AppCore {
    pub fn update(&mut self, _message: Message) {
    }

    pub fn view(&self) -> Column<Message> {
        column![
            text!("Welcome to Limiinal!"),
            self.containers(),
        ]
    }

    pub fn containers(&self) -> Element<Message> {
        row![
            container("First container")
                .padding(10)
                .width(Length::Fixed(300.0))
                .height(Length::Fixed(100.0)) 
                .style(ContainerStyle::float_view()),
            container("Second container")
                .padding(10)
                .width(Length::Fixed(500.0))
                .height(Length::Fixed(100.0)) 
                .style(container::rounded_box),
        ]
        .width(Length::Fill) 
        .into()
    }
} 

struct ContainerStyle;

impl ContainerStyle {
    fn float_view() -> impl Fn(&Theme) -> container::Style {
        move |_| container::Style {
            background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
            text_color: Some(Color::WHITE),
            border: iced::Border {
                radius: iced::border::Radius {
                    top_left: 20.0,
                    top_right: 20.0,
                    bottom_left: 20.0,
                    bottom_right: 20.0,
                },
                ..iced::Border::default()
            },
            ..container::Style::default()
        }
    }
}
