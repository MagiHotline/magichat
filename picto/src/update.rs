use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;

use crate::app::App;
use crate::app::InputMode;

// probably will need two states of update, once one in the CHAT and the other one
// while we inserting the input and connecting

pub fn update(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        crate::app::InputMode::Normal => match key.code {
            KeyCode::Char('e') => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('q') => app.should_quit = true,
            _ => {}
        },
        crate::app::InputMode::Editing => match key.code {
            KeyCode::Enter => app.submit(),
            KeyCode::Char(to_insert) => app.enter_char(to_insert),
            KeyCode::Backspace => app.delete_char(),
            KeyCode::Tab => app.next_field(),
            KeyCode::BackTab => app.previous_field(),
            KeyCode::Left => app.move_cursor_left(),
            KeyCode::Right => app.move_cursor_right(),
            _ => {}
        },
    }
}
