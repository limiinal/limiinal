#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{column, container, row, Text, text, Column};
use iced::{Element, Length, Color, Theme};
use iced::widget::image;
use iced::widget::image::Handle;


#[derive(Default)]
pub struct AppCore {
    window_width: f32,
    window_height: f32,
    nav_float_views: NavFloatView,
    logo_float_view: LogoFloatView,
    message_list_float_view: MessageListFloatView,
    message_float_view: MessageFloatView,
 }

#[derive(Debug, Clone)]
pub enum Message {
    Resize(f32, f32),
}

impl AppCore {

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
            //text!("Welcome to Limiinal!"),
            self.containers(),
        ]
        .padding(10)
    }

    pub fn containers(&self) -> Element<Message> {
        row![
            column![
                self.logo_float_view.container_view(),
                self.nav_float_views.container_view(),
            ]
            .spacing(10),
            self.message_list_float_view.container_view(),
            self.message_float_view.container_view(),
        ]
        .width(Length::Fill) 
        .spacing(10)
        .into()
    }

    pub fn subscribtion(&self) -> iced::Subscription<Message> {
         todo!()
    }
} 

//====== Logo Float View ======//
struct LogoFloatView {
    pub id: i32,
    pub name: String,
    pub width: Length,
    pub height: Length,
    pub image_path: String,
}

impl LogoFloatView {
    fn container_view(&self) -> Element<'_, Message> {
        let logo_image = image(Handle::from_path(&self.image_path))
            .width(self.width)
            .height(self.height);
        
        container(logo_image)
            .width(self.width)
            .height(self.height)
            .style(LogoFloatView::style()).into()
    }

    fn style() -> impl Fn(&Theme) -> container::Style {
        move |_| container::Style {
            background: Some(Color::from_rgb(0.5, 0.5, 0.5).into()),
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

impl Default for LogoFloatView {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from("Logo"),
            width: Length::Fixed(100.0),
            height: Length::Fixed(100.0),
            image_path: "./assets/images/LimiinalTransparent.png".to_string(),
        }
    }
}
//====== Nav Float View ======//
struct NavFloatView {
    pub id: i32,
    pub name: String,
    pub home: String,
    pub width: Length,
    pub height: Length,
    pub active: bool,
}

impl NavFloatView {
    fn container_view(&self) -> Element<'_, Message> {
        container(self.name.as_str())
        //container("asdjoad")
            .width(self.width)
            .height(self.height)
            .style(NavFloatView::style()).into()
    }

    fn style() -> impl Fn(&Theme) -> container::Style {
        move |_| container::Style {
            background: Some(Color::from_rgb(0.4, 0.4, 0.4).into()),
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

impl Default for NavFloatView {
    fn default() -> Self {
        Self {
            id: 1,
            name: String::from("Nav"),
            home: String::from(""),
            width: Length::Fixed(100.0),
            height: Length::Fixed(300.0),
            active: false,
        }
    }
}

//====== Message List Float View ======//
struct MessageListFloatView {
    pub id: i32,
    pub name: String,
    pub width: Length,
    pub height: Length,
}

impl MessageListFloatView {
    fn container_view(&self) -> Element<'_, Message> {
        container(self.name.as_str())
            .width(self.width)
            .height(self.height)
            .style(MessageListFloatView::style()).into()
    }

    fn style() -> impl Fn(&Theme) -> container::Style {
        move |_| container::Style {
            background: Some(Color::from_rgb(0.3, 0.3, 0.3).into()),
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

impl Default for MessageListFloatView {
    fn default() -> Self {
        Self {
            id: 2,
            name: String::from("MessageList"),
            width: Length::FillPortion(3),
            height: Length::Fill,
        }
    }
}
//====== Message Float View ======//
struct MessageFloatView {
    pub id: i32,
    pub name: String,
    pub width: Length,
    pub height: Length,
}

impl MessageFloatView {
    fn container_view(&self) -> Element<'_, Message> {
        container(self.name.as_str())
            .width(self.width)
            .height(self.height)
            .style(MessageFloatView::style()).into()
        
    }

    fn style() -> impl Fn(&Theme) -> container::Style {
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

impl Default for MessageFloatView {
    fn default() -> Self {
        Self {
            id: 3,
            name: String::from("Message"),
            width: Length::FillPortion(8),
            height: Length::Fill,
        }
    }
}
