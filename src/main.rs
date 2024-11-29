mod ui;

use ui::gui::AppCore;

use iced::Theme;

fn main() -> iced::Result {
    iced::application("Limiinal", AppCore::update, AppCore::view)
        .theme(|_| { Theme::Dark })
        .run()
}

