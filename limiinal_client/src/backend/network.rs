use tokio::task;

pub struct AppCore {
    pub backend_thread: Option<task::JoinHandle<()>>,
}

impl AppCore {
    pub fn new() -> Self {
        AppCore {
            backend_thread: None,
        }
    }

    pub async fn run(&mut self) {
        //self.backend_thread = Some(task::spawn(async {
        //    AppCore::start().await;
        //}));
    }

    async fn start() {
        loop {
            println!("AppCore::start()");
        }
    }
}
