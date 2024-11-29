mod ui;

use ui::gui::AppCore;


fn main() -> iced::Result {
    /*iced::run("Limiinal", AppCore::update, AppCore::view)*/
    iced::run("Limiinal", AppCore::update, AppCore::containers)
}

