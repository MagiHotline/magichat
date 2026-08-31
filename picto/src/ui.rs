use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Position},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::{ActiveEditingArea, App};
use crate::app::{AppState, InputMode};

pub fn render_chat(app: &mut App, frame: &mut Frame) {
    let layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(3),
        Constraint::Length(1),
    ]);

    let [messages_area, input_area, _] = frame.area().layout(&layout);

    // Render the three text fields
    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::White))
        .block(Block::bordered().title("Input"));
    frame.render_widget(input, input_area);

    match app.input_mode {
        // Hide the cursor.
        InputMode::Normal => {}
        #[expect(clippy::cast_possible_truncation)]
        InputMode::Editing => frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            input_area.x + app.curr_char_idx as u16 + 1,
            input_area.y + 1,
        )),
    }

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .map(|m| {
            let content = Line::from(Span::raw(format!("{}: {m}", app.host.name)));
            ListItem::new(content)
        })
        .collect();

    frame.render_widget(
        List::new(messages).block(Block::bordered().title("Messages")),
        messages_area,
    );
}

pub fn render_text_area(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Percentage(30),
        Constraint::Fill(1),
    ]);

    let [_, center_area, _] = frame.area().layout(&outer_layout);

    let layout = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(13),
        Constraint::Length(2),
        Constraint::Min(1),
    ]);

    let [_, main_area, message_area, _] = center_area.layout(&layout);

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

    let (msg, style) = match app.state {
        AppState::Filling => match app.input_mode {
            InputMode::Normal => (
                format!("[q] Exit | [e] Edit"),
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                format!("[Esc] Stop editing, [Enter] Submit, [Tab/Untab] Move"),
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
        },
        AppState::Connection => (
            format!("Connecting to client..."),
            Style::default()
                .add_modifier(Modifier::RAPID_BLINK)
                .fg(Color::Yellow),
        ),
        AppState::Connected => (
            format!("Connection successful!"),
            Style::default()
                .add_modifier(Modifier::RAPID_BLINK)
                .fg(Color::Green),
        ),
    };

    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
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
            host_name_input_area.y + 1,
        ),
        ActiveEditingArea::ClientSocket => (
            host_socket_input_area.x + app.curr_char_idx as u16 + 1,
            host_socket_input_area.y + 1,
        ),
        ActiveEditingArea::DestinationSocket => (
            dest_socket_input_area.x + app.curr_char_idx as u16 + 1,
            dest_socket_input_area.y + 1,
        ),
        _ => unreachable!(),
    };

    match app.input_mode {
        // Hide the cursor. `Frame` does this by default.
        InputMode::Normal => {}
        // Make the cursor visible
        #[expect(clippy::cast_possible_truncation)]
        InputMode::Editing => {
            frame.set_cursor_position(Position::new(editing_position.0, editing_position.1))
        }
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
