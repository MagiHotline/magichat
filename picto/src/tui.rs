use std::{io, panic};

use color_eyre::Result;
use ratatui::crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::App, event::EventHandler, ui};

/// Representation of a Terminal User Interface,
/// it sets up the interface and draw events
pub struct Tui {
    pub terminal: CrosstermTerminal,
    pub events: EventHandler,
}
