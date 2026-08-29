use client::client::Client;

pub struct App {
    pub should_quit: bool,
    pub host: Client,
    pub dest_ip_address: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: Default::default(),
            host: Default::default(),
            dest_ip_address: Default::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            host: Client::default(),
            dest_ip_address: String::new(),
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
