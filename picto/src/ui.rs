use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Position},
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::InputMode;
use crate::app::{ActiveEditingArea, App};

pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Percentage(40),
        Constraint::Fill(1),
    ]);

    let [_, center_area, _] = frame.area().layout(&outer_layout);

    let layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Percentage(50),
        Constraint::Min(1),
    ]);

    // let [[block => 3 x input_area], messages_area] = frame.area().layout(&layout);
    let [_, main_area, message_area] = center_area.layout(&layout);

    let inner_layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(1),
    ]);

    let [
        _,
        host_name_input_area,
        host_socket_input_area,
        dest_socket_input_area,
        _,
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
    let help_message = Paragraph::new(text).alignment(Alignment::Center);
    frame.render_widget(help_message, message_area);

    // Render the three text fields
    let input = Paragraph::new(app.host.name.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Host Name"));
    frame.render_widget(input, host_name_input_area);

    let input = Paragraph::new(app.host_ip_address.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Host Socket"));
    frame.render_widget(input, host_socket_input_area);

    let input = Paragraph::new(app.dest_ip_address.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::bordered()
                .title("Destination Socket")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        );
    frame.render_widget(input, dest_socket_input_area);

    // Handle the cursor based of where we are
    let editing_position: (u16, u16) = match app.editing_area {
        ActiveEditingArea::ClientName => (
            host_name_input_area.x + app.curr_char_idx as u16 + 1,
            // Move one line down, from the border to the input line
            host_name_input_area.y + 1,
        ),
        ActiveEditingArea::ClientSocket => (
            host_socket_input_area.x + app.curr_char_idx as u16 + 1,
            // Move one line down, from the border to the input line
            host_socket_input_area.y + 1,
        ),
        ActiveEditingArea::DestinationSocket => (
            dest_socket_input_area.x + app.curr_char_idx as u16 + 1,
            // Move one line down, from the border to the input line
            dest_socket_input_area.y + 1,
        ),
    };

    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        InputMode::Normal => {}

        // Make the cursor visible and ask ratatui to put it at the specified coordinates after
        // rendering
        #[expect(clippy::cast_possible_truncation)]
        InputMode::Editing => frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            editing_position.0,
            editing_position.1,
        )),
    }

    frame.render_widget(
        Block::default()
            .title("Pictochat")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().fg(Color::White)),
        main_area,
    );
}
