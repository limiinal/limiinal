mod backend;
mod ui;

use backend::network::AppCore;
use ui::gui::AppUI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // set up backend
    AppCore::new().run().await;

    // set up frontend
    AppUI::run()?;

    Ok(())
}
