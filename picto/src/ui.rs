use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render(_app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("Press 'q' to exit"))
            .block(
                Block::default()
                    .title("Pictochat")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White)),
        frame.area(),
    );
}
