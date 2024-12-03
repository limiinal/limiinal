mod ui;
mod backend;

use ui::gui::AppUI;
use backend::network::AppCore;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    // set up backend
    AppCore::new().run().await;

    // set up frontend
    AppUI::run()?;


    Ok(())
}

