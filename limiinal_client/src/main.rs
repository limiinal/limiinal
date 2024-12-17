mod backend;
mod ui;

use backend::network::AppCore;
use iced::Theme;
use ui::gui::AppUI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    iced::application("Limiinal", AppUI::update, AppUI::view)
        .theme(|_| Theme::Dark)
        .run_with(AppUI::new)?;

    Ok(())
}
