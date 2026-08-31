use crate::app::ActiveEditingArea::{ClientName, ClientSocket, DestinationSocket};
use client::client::Client;
use color_eyre::Result;
use std::slice::Iter;

/// Represents Input modes.
#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Enumerates the filling text areas.
#[repr(C)]
#[derive(Debug)]
pub enum ActiveEditingArea {
    ClientName,
    ClientSocket,
    DestinationSocket,
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
            ActiveEditingArea::ClientSocket => {
                new_cursor_pos.clamp(0, self.host_ip_address.chars().count())
            }
            ActiveEditingArea::DestinationSocket => {
                new_cursor_pos.clamp(0, self.dest_ip_address.chars().count())
            }
        }
    }

    pub fn next_field(&mut self) {
        match self.editing_area {
            ClientName => self.editing_area = ClientSocket,
            ClientSocket => self.editing_area = DestinationSocket,
            DestinationSocket => self.editing_area = ClientName,
        }
        self.reset_cursor();
    }

    pub fn previous_field(&mut self) {
        match self.editing_area {
            ClientName => self.editing_area = DestinationSocket,
            ClientSocket => self.editing_area = ClientName,
            DestinationSocket => self.editing_area = ClientSocket,
        }
        self.reset_cursor();
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
            ActiveEditingArea::ClientSocket => self.host_ip_address.insert(idx, ch),
            ActiveEditingArea::DestinationSocket => self.dest_ip_address.insert(idx, ch),
        }
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        if self.curr_char_idx != 0 {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.
            // WE DOING IT ANYWAY
            match self.editing_area {
                ClientName => {
                    self.host.name.remove(self.curr_char_idx);
                }
                ClientSocket => {
                    self.host_ip_address.remove(self.curr_char_idx);
                }
                DestinationSocket => {
                    self.dest_ip_address.remove(self.curr_char_idx);
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
            ActiveEditingArea::ClientSocket => self
                .host_ip_address
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.curr_char_idx)
                .unwrap_or(self.host_ip_address.len()),
            ActiveEditingArea::DestinationSocket => self
                .dest_ip_address
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.curr_char_idx)
                .unwrap_or(self.dest_ip_address.len()),
        }
    }

    const fn reset_cursor(&mut self) {
        self.curr_char_idx = 0;
    }

    /// Submits connection's data to the protocol
    pub fn submit(&mut self) {
        todo!()
    }

    /// Once data is submitted, it connects to the destination
    /// If not, sends you back to the Menu with all the areas empty
    pub fn try_connection() -> Result<()> {
        todo!()
    }
}
