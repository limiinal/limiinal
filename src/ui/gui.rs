#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use iced::widget::{column, container, row, image, button, text, svg,};
use iced::widget::{Column, Space, button::Status};
use iced::{Element, Length, Color, Theme, Padding, Border, Background};
use iced::border::Radius;
use iced::widget::image::Handle;
use log::info;
use iced::widget::TextInput;
use iced::widget::text_input;
use iced::Alignment;


#[derive(Default)]
pub struct AppUI {
    window_width: f32,
    window_height: f32,
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
}

impl AppUI {
    pub fn run() -> iced::Result {
        iced::application("Limiinal", AppUI::update, AppUI::view)
            .theme(|_| { Theme::Dark })
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
                self.message_list_float_view.search_query = new_content;
                info!("Content changed to...");
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
}

impl LogoFloatView {
    fn container_view(&self) -> Element<'_, Message> {
        container(svg::Svg::from_path("./assets/icons/logo.svg"))
            .width(self.width)
            .height(self.height)
            .style(LogoFloatView::style()).into()
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
        container(
            column![
                button(
                    svg::Svg::from_path("./assets/icons/home.svg")
                )
                .padding(15)
                .on_press(Message::NavToHome)
                .style(self.button_style(NavFloatViewButton::Home)),
                Space::with_height(Length::Fill),
                button(
                    svg::Svg::from_path("./assets/icons/chat.svg")
                )
                .padding(15)
                .on_press(Message::NavToChat)
                .style(self.button_style(NavFloatViewButton::Chat)),
                Space::with_height(Length::Fill),
                button(
                    svg::Svg::from_path("./assets/icons/setting.svg")
                )
                .padding(15)
                .on_press(Message::NavToSettings)
                .style(self.button_style(NavFloatViewButton::Settings)),
            ]
        )
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
            .style(NavFloatView::style()).into()
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

    fn button_style(&self, active: NavFloatViewButton) -> impl Fn(&Theme, Status) -> button::Style + '_ {
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
    pub text_input_state: State,
}

impl MessageListFloatView {
    pub fn container_view(&mut self) -> Element<'_, Message> {
        println!("Why does this not show?");
    
        let is_focused = self.text_input_state.is_focused();
    
        let status_message = if is_focused {
            ""
        } else {
            "Search messages"
        };
    
        let input: TextInput<'_, Message> = TextInput::new(
            &mut self.text_input_state, 
            status_message,
            &self.search_query,
            Message::ContentChanged,
        )
        .align_x(iced::alignment::Horizontal::Center)
        .style(MyTextInput) 
        .width(200.0);
    
        let input_element: Element<'_, Message> = input.into();
    
        let content_column = Column::new()
            .align_x(iced::Alignment::End)
            .padding(10)
            .push(input_element);
    
        container(content_column)
            .width(self.width)
            .height(self.height)
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
            text_input_state: State::new(),
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
        }
    }
}
