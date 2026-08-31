use ratatui::{Frame, widgets::Block};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(Block::bordered().title("Pictochat"), frame.area());
}
