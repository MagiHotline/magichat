use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::app::ActiveEditingArea::ClientName;
use client::client::Client;

/// Represents Input modes.
#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Enumerates the filling text areas.
#[derive(Debug)]
pub enum ActiveEditingArea {
    ClientName,
    Input,
}

/// Represents App's states
#[derive(Debug)]
pub enum AppState {
    Filling,
    Connection,
    Connected,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    EmptyFields,
}

/// App's state.
pub struct App {
    pub should_quit: bool,
    /// Client's configuration
    pub host: Client,
    /// Client's Socket
    pub host_ip_address: String,
    /// Socket you want to connect to
    pub dest_ip_address: String,
    /// Current cursor posistion
    pub curr_char_idx: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// Current editing area
    pub editing_area: ActiveEditingArea,
    /// App's current state
    pub state: AppState,
    /// Messages that are sent and received
    pub messages: Arc<Mutex<Vec<String>>>,
    /// Input chat message
    pub input: String,
    /// Last Error occured
    pub last_error_occured: Option<ErrorKind>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: Default::default(),
            host: Default::default(),
            dest_ip_address: Default::default(),
            editing_area: ClientName,
            input_mode: InputMode::Normal,
            curr_char_idx: 0,
            host_ip_address: Default::default(),
            state: AppState::Filling,
            messages: Arc::new(Mutex::new(Vec::new())),
            input: Default::default(),
            last_error_occured: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        // Based on which Text Field I am I need to clamp the cursor position in
        // that text field
        match self.editing_area {
            ActiveEditingArea::ClientName => {
                new_cursor_pos.clamp(0, self.host.name.chars().count())
            }
            ActiveEditingArea::Input => new_cursor_pos.clamp(0, self.input.chars().count()),
        }
    }

    pub fn next_field(&mut self) {
        todo!()
    }

    pub fn previous_field(&mut self) {
        todo!()
    }

    pub fn move_cursor_right(&mut self) {
        let right = self.curr_char_idx.saturating_add(1);
        self.curr_char_idx = self.clamp_cursor(right);
    }

    pub fn move_cursor_left(&mut self) {
        let left = self.curr_char_idx.saturating_sub(1);
        self.curr_char_idx = self.clamp_cursor(left);
    }

    pub fn enter_char(&mut self, ch: char) {
        let idx = self.byte_index();
        match self.editing_area {
            ClientName => self.host.name.insert(idx, ch),
            ActiveEditingArea::Input => self.input.insert(idx, ch),
        }
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.curr_char_idx != 0 {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.
            // WE DOING IT ANYWAY

            let current_idx = self.curr_char_idx;
            let from_left_curr_idx = current_idx - 1;
            match self.editing_area {
                ClientName => {
                    // Getting all characters before the selected character.
                    let before_char_to_delete = self.host.name.chars().take(from_left_curr_idx);
                    // Getting all characters after selected character.
                    let after_char_to_delete = self.host.name.chars().skip(current_idx);
                    // Put all characters together except the selected one.
                    // By leaving the selected one out, it is forgotten and therefore deleted.
                    self.host.name = before_char_to_delete.chain(after_char_to_delete).collect();
                }
                ActiveEditingArea::Input => {
                    let before_char_to_delete = self.input.chars().take(from_left_curr_idx);
                    let after_char_to_delete = self.input.chars().skip(current_idx);
                    self.input = before_char_to_delete.chain(after_char_to_delete).collect();
                }
            }
            self.move_cursor_left();
        }
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        match self.editing_area {
            ClientName => self
                .host
                .name
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.curr_char_idx)
                .unwrap_or(self.host.name.len()),
            ActiveEditingArea::Input => self
                .input
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.curr_char_idx)
                .unwrap_or(self.input.len()),
        }
    }

    const fn reset_cursor(&mut self) {
        self.curr_char_idx = 0;
    }

    /// Submits connection's data to the protocol
    pub fn submit(&mut self) -> Result<(), ErrorKind> {
        match self.state {
            AppState::Filling => {
                if self.host.name.is_empty()
                    || self.dest_ip_address.is_empty()
                    || self.host_ip_address.is_empty()
                {
                    self.last_error_occured = Some(ErrorKind::EmptyFields);
                    return Err(ErrorKind::EmptyFields);
                }

                self.state = AppState::Connection;
                let client = Client::connect(self.host.name.clone(), self.host_ip_address.clone())
                    .expect("Failed to connect client once submitted");

                self.host = client;
                Ok(())
            }
            AppState::Connected => {
                let full_message = format!("{}: {}", self.host.name.clone(), self.input.clone());
                self.messages.lock().unwrap().push(full_message.clone());
                self.host
                    .write(full_message.as_bytes())
                    .expect("Failed to transform string to bytes");
                self.input.clear();
                self.reset_cursor();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Once data is submitted, it connects to the destination
    /// If not, sends you back to the Menu with all the areas empty
    pub fn try_connection(&mut self) {
        let mut receiver_client = self.host.try_clone().expect("Failed to clone Client");
        // Once connected, spawn the listening thread
        let recv_messages = self.messages.clone();
        thread::spawn(move || {
            loop {
                let mut buf = [0; 8192];
                if let Ok(received) = receiver_client.read(&mut buf) {
                    if let Ok(output) = str::from_utf8(&buf[..received]) {
                        recv_messages.lock().unwrap().push(output.to_string());
                    }
                } else {
                    println!("Recv function failed");
                    break;
                }
            }
        });

        self.state = AppState::Connected;
        self.editing_area = ActiveEditingArea::Input;
        self.reset_cursor();
    }
}
