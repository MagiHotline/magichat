use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use crate::app::{App, AppState, InputMode};

// probably will need two states of update, once one in the CHAT and the other one
// while we inserting the input and connecting

pub fn update(app: &mut App, key: KeyEvent) {
    match app.state {
        AppState::Filling => match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => app.should_quit = true,
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    app.submit();
                    app.try_connection()
                }
                KeyCode::Char(to_insert) => app.enter_char(to_insert),
                KeyCode::Backspace => app.delete_char(),
                KeyCode::Tab => app.next_field(),
                KeyCode::BackTab => app.previous_field(),
                KeyCode::Left => app.move_cursor_left(),
                KeyCode::Right => app.move_cursor_right(),
                KeyCode::Esc => app.input_mode = InputMode::Normal,
                _ => {}
            },
        },
        AppState::Connection => {}
        AppState::Connected => match key.code {
            KeyCode::Enter => {
                app.submit();
            }
            KeyCode::Char(to_insert) => app.enter_char(to_insert),
            KeyCode::Backspace => app.delete_char(),
            KeyCode::Left => app.move_cursor_left(),
            KeyCode::Right => app.move_cursor_right(),
            KeyCode::Esc => app.should_quit = true,
            _ => {}
        },
    }
    /*

    */
}
