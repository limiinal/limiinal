#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::border::Radius;
use iced::widget::image::Handle;
use iced::widget::scrollable;
use iced::widget::TextInput;
use iced::widget::{button, column, container, image, row, svg, text, center, text_input};
use iced::widget::{button::Status, Column, Space};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Theme};
use iced::keyboard;
use log::info;

macro_rules! asset_path {
    ($path:expr) => {
        format!("{}/{}", env!("CARGO_MANIFEST_DIR"), $path)
    };
}

#[derive(Default)]
pub struct AppUI {
    window_width: f32,
    window_height: f32,

    // float views
    logo_float_view: LogoFloatView,
    nav_float_views: NavFloatView,
    message_list_float_view: MessageListFloatView,
    message_float_view: MessageFloatView,
    search_query: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Resize(f32, f32),
    ContentChanged(String),

    // Navigation events
    NavToHome,
    NavToChat,
    NavToSettings,

    ChatInputChanged(String),
    SendMessage,
}

impl AppUI {
    pub fn run() -> iced::Result {
        iced::application("Limiinal", AppUI::update, AppUI::view)
            .theme(|_| Theme::Dark)
            .run()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Resize(width, height) => {
                self.window_width = width;
                self.window_height = height;
                println!("Resized! New width: {}", self.window_width);
            }
            Message::NavToHome => {
                info!("Navigating to Home");
                self.nav_float_views.current_active = NavFloatViewButton::Home;
            }
            Message::NavToChat => {
                info!("Navigating to Chat");
                self.nav_float_views.current_active = NavFloatViewButton::Chat;
            }
            Message::NavToSettings => {
                info!("Navigating to Settings");
                self.nav_float_views.current_active = NavFloatViewButton::Settings;
            }
            // This needs to be updated with functionality when a search is entered.
            Message::ContentChanged(new_content) => {
                self.message_list_float_view.search_query = new_content.to_string();
                info!("{}", format!("Content changed to {}", new_content));
            }
            Message::ChatInputChanged(new_content) => {
                self.message_float_view.input_message = new_content.to_string();
                info!("{}", format!("Chat input changed to {}", self.message_float_view.input_message));
            }
            Message::SendMessage => {
                if self.message_float_view.input_message.is_empty() {
                    return;
                }
                self.message_float_view.chat_message.push(ChatMessage {
                    time: String::from(""),
                    sender: String::from(""),
                    body: self.message_float_view.input_message.to_string(),
                    is_read: false,
                });
                info!("{}", format!("Message sent: {}", self.message_float_view.input_message));

                self.message_float_view.input_message = String::new();
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
}

//====== Logo Float View ======//
struct LogoFloatView {
    pub id: i32,
    pub name: String,
    pub width: Length,
    pub height: Length,
}

impl LogoFloatView {
    fn container_view(&self) -> Element<'_, Message> {
        container(svg::Svg::from_path(asset_path!("./assets/icons/logo.svg")))
            .width(self.width)
            .height(self.height)
            .style(LogoFloatView::style())
            .into()
    }

    fn style() -> impl Fn(&Theme) -> container::Style {
        move |_| container::Style {
            background: Some(Color::from_rgb(0.5, 0.5, 0.5).into()),
            text_color: Some(Color::WHITE),
            border: Border {
                radius: Radius {
                    top_left: 20.0,
                    top_right: 20.0,
                    bottom_left: 20.0,
                    bottom_right: 20.0,
                },
                ..Border::default()
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
        }
    }
}
//====== Nav Float View ======//
#[derive(Debug, PartialEq)]
enum NavFloatViewButton {
    Home,
    Chat,
    Settings,
}

struct NavFloatView {
    pub id: i32,
    pub name: String,
    pub home: String,
    pub width: Length,
    pub height: Length,
    pub current_active: NavFloatViewButton,
}

impl NavFloatView {
    fn container_view(&self) -> Element<'_, Message> {
        container(column![
            button(svg::Svg::from_path(asset_path!("assets/icons/home.svg")))
                .padding(15)
                .on_press(Message::NavToHome)
                .style(self.button_style(NavFloatViewButton::Home)),
            Space::with_height(Length::Fill),
            button(svg::Svg::from_path(asset_path!("./assets/icons/chat.svg")))
                .padding(15)
                .on_press(Message::NavToChat)
                .style(self.button_style(NavFloatViewButton::Chat)),
            Space::with_height(Length::Fill),
            button(svg::Svg::from_path(asset_path!(
                "./assets/icons/setting.svg"
            )))
            .padding(15)
            .on_press(Message::NavToSettings)
            .style(self.button_style(NavFloatViewButton::Settings)),
        ])
        .center(Length::Fill)
        .padding(Padding {
            top: 25.0,
            left: 10.0,
            right: 10.0,
            bottom: 25.0,
        })
        //container("asdjoad")
        .width(self.width)
        .height(self.height)
        .style(NavFloatView::style())
        .into()
    }

    fn style() -> impl Fn(&Theme) -> container::Style {
        move |_| container::Style {
            background: Some(Color::from_rgb(0.4, 0.4, 0.4).into()),
            text_color: Some(Color::WHITE),
            border: Border {
                radius: Radius {
                    top_left: 20.0,
                    top_right: 20.0,
                    bottom_left: 20.0,
                    bottom_right: 20.0,
                },
                ..Border::default()
            },
            ..container::Style::default()
        }
    }

    fn button_style(
        &self,
        active: NavFloatViewButton,
    ) -> impl Fn(&Theme, Status) -> button::Style + '_ {
        move |_, status| {
            let mut background: Option<Background>;
            let mut border: Border;
            match status {
                Status::Hovered => {
                    background = Some(Color::from_rgb(0.45, 0.45, 0.45).into());
                    border = Border {
                        radius: Radius {
                            top_left: 20.0,
                            top_right: 20.0,
                            bottom_left: 20.0,
                            bottom_right: 20.0,
                        },
                        ..Border::default()
                    }
                }
                _ => {
                    background = None;
                    border = Border::default();
                }
            };

            // check the active the is current active
            if active == self.current_active {
                background = Some(Color::from_rgb(0.5, 0.5, 0.5).into());
                border = Border {
                    radius: Radius {
                        top_left: 20.0,
                        top_right: 20.0,
                        bottom_left: 20.0,
                        bottom_right: 20.0,
                    },
                    ..Border::default()
                }
            }

            button::Style {
                background,
                border,
                ..button::Style::default()
            }
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
            current_active: NavFloatViewButton::Chat,
        }
    }
}

//====== Message List Float View ======//
struct MessageListFloatView {
    pub id: i32,
    pub name: String,
    pub width: Length,
    pub height: Length,
    pub content: String,
    pub search_query: String,
}

impl MessageListFloatView {
    fn container_view(&self) -> Element<'_, Message> {
        let input: TextInput<'_, Message> =
            text_input::<Message, iced::theme::Theme, iced::Renderer>(
                "Search messages",
                &self.search_query,
            )
            .on_input(Message::ContentChanged)
            // Aligns text central
            .align_x(iced::Alignment::Center)
            .width(200.0);

        let input_element: Element<'_, Message> = input.into();

        let content_column = Column::new()
            .align_x(iced::Alignment::End)
            .padding(10)
            .push(input_element);

        container(content_column)
            .width(self.width)
            .height(self.height)
            // This is causing search bar to be aligned correctly.
            // However it may cause everything in container to be central.
            // Cant seem to align it any other way.
            .align_x(iced::Alignment::Center)
            .style(MessageListFloatView::style())
            .into()
    }

    fn style() -> impl Fn(&Theme) -> container::Style {
        move |_| container::Style {
            background: Some(Color::from_rgb(0.3, 0.3, 0.3).into()),
            text_color: Some(Color::WHITE),
            border: Border {
                radius: Radius {
                    top_left: 20.0,
                    top_right: 20.0,
                    bottom_left: 20.0,
                    bottom_right: 20.0,
                },
                ..Border::default()
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
            content: String::from("Search Messages"),
            search_query: String::new(),
        }
    }
}
//====== Message Float View ======//
struct ChatMessage {
    time: String,
    sender: String,
    body: String,
    is_read: bool,
}

struct MessageFloatView {
    pub id: i32,
    pub name: String,
    pub width: Length,
    pub height: Length,
    pub input_message: String,
    pub chat_message: Box<Vec<ChatMessage>>,
}

impl MessageFloatView {
    fn container_view(&self) -> Element<Message> {
        // chat view
        let chat_view: Element<_> = if self.chat_message.is_empty() {
            center(
                text("Start a Conversation")
            ).into()
        } else {
            scrollable(
                column(self.chat_message.iter().map(|msg| {
                    row![
                        text(&msg.body).width(Length::Fill),
                    ].into()
                }))
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        };

        // message input
        let message_input = {
            // input field
            let input = text_input("Message", &self.input_message)
                .on_input(|content| Message::ChatInputChanged(content))
                .on_submit(Message::SendMessage);

            // send button
            let mut send_button = button("Send")
                .style(|_, _| button::Style {
                    background: Some(Color::from_rgb(0.4, 0.4, 0.4).into()),
                    border: Border {
                        radius: Radius {
                            top_left: 20.0,
                            top_right: 20.0,
                            bottom_left: 20.0,
                            bottom_right: 20.0,
                        },
                        ..Border::default()
                    },
                    ..button::Style::default()
                });

            if !self.input_message.is_empty() {
                send_button = send_button.on_press(Message::SendMessage);
            }

 
            row![input, send_button].spacing(10)
        };

        // message view
        let message_view = column![chat_view, message_input];
        container(message_view)
            .padding(20)
            .width(self.width)
            .height(self.height)
            .style(MessageFloatView::style())
            .into()
    }

    fn style() -> impl Fn(&Theme) -> container::Style {
        move |_| container::Style {
            background: Some(Color::from_rgb(0.2, 0.2, 0.2).into()),
            text_color: Some(Color::WHITE),
            border: Border {
                radius: Radius {
                    top_left: 20.0,
                    top_right: 20.0,
                    bottom_left: 20.0,
                    bottom_right: 20.0,
                },
                ..Border::default()
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
            chat_message: Box::new(vec![
                ChatMessage {
                    time: String::from("12:00"),
                    sender: String::from("John"),
                    body: String::from("Hello!"),
                    is_read: false,
                },
                ChatMessage {
                    time: String::from("12:01"),
                    sender: String::from("Jane"),
                    body: String::from("Hi!"),
                    is_read: false,
                },
            ]),
            input_message: String::new(),
        }
    }
}

fn load_svg(name: &str) -> Result<svg::Handle, std::io::Error> {
    let path = format!("{}/resources/{}.svg", env!("CARGO_MANIFEST_DIR"), name);

    // Attempt to create the SVG handle
    let handle = svg::Handle::from_path(path);

    Ok(handle)
}
