mod ui;

use ui::gui::AppCore;

use iced::Theme;

fn main() -> iced::Result {
    iced::application("Limiinal", AppCore::update, AppCore::view)
        //.subscription(AppCore::subscribtion)
        .theme(|_| { Theme::Dark })
        .run()
}

