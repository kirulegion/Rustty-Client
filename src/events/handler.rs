use ratatui::crossterm::event::{self, KeyCode, KeyEvent};

use crate::state::app_state::{App_state, Focused};

pub fn handle_modes(key: KeyEvent, state: &mut App_state) {
    match (state.focused, key.code) {
        //Exit the application when q is pressed in normal.
        (Focused::Normal, KeyCode::Char('q')) => {
            state.is_running = false;
        }
        //Enter the url block to edit or enter the url.
        (Focused::Normal, event::KeyCode::Char('U')) => {
            state.focused = Focused::Url;
        }
        //Enter the methods selection block.
        (Focused::Normal, event::KeyCode::Char('M')) => {
            state.focused = Focused::Method;
        }
        //Entry normal mode when ESC is pressed.
        (_, event::KeyCode::Esc) => {
            state.focused = Focused::Normal;
        }
        _ => {}
    }
}

pub fn handle_url(key: KeyEvent, state: &mut App_state) {
    match key.code {
        event::KeyCode::Char(c) => {
            state.url_input.push(c);
        }
        event::KeyCode::Backspace => {
            state.url_input.pop();
        }
        _ => {}
    }
}

pub fn handle_method(key: KeyEvent, state: &mut App_state) {
    match (key.code, state.focused) {
        (KeyCode::Down, Focused::Method) => {
            state.method = state.method.next();
        }
        (KeyCode::Up, Focused::Method) => {
            state.method = state.method.prev();
        }
        _ => {}
    }
}
