use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
use crate::app::InputMode;

pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Percentage(50),
        Constraint::Fill(1),
    ]);

    let [_, center_area, _] = frame.area().layout(&outer_layout);

    let layout = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);

    // let [[block => 3 x input_area], messages_area] = frame.area().layout(&layout);
    let [_, main_area, message_area] = center_area.layout(&layout);

    let inner_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
    ]);

    let [
        host_name_input_area,
        host_socket_input_area,
        dest_socket_input_area,
    ] = main_area.layout(&inner_layout);

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            format!("Press 'q' to exit, 'e' to start editing"),
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            format!(
                "Press 'Esc' to stop editing, 'Enter' to record the messages, 'Tab/Untab' to move to the next TextFields"
            ),
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
    };

    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, message_area);

    frame.render_widget(
        Paragraph::new(format!(""))
            .block(
                Block::default()
                    .title("Pictochat")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White)),
        main_area,
    );
}
