use std::time::Duration;

use color_eyre::eyre::Result;
use events::handler::{handle_features, handle_method, handle_modes, handle_url};
use ratatui::{
    crossterm::event::{self, Event},
    DefaultTerminal, Frame,
};
use state::app_state::{App_state, Focused};

//Importing all the folder and file struct
mod events;
mod network;
mod state;
mod ui;
mod utils;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state = App_state::new();

    while app_state.is_running {
        if event::poll(Duration::from_millis(10))? {
            handler_key_events(&mut app_state)?;
        }

        // Then draw UI
        terminal.draw(|f| {
            render(f, &mut app_state);
        })?;
    }

    Ok(())
}

pub fn handler_key_events(app_state: &mut App_state) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        let mode_switch = handle_modes(key, app_state);

        if mode_switch {
            return Ok(());
        }

        match app_state.focused {
            Focused::Url => {
                handle_url(key, app_state);
            }
            Focused::Method => {
                handle_method(key, app_state);
            }
            Focused::Feature => {
                handle_features(key, app_state);
            }
            Focused::Workspace => {}
            _ => {}
        }
    }

    Ok(())
}

pub fn render(f: &mut Frame, app_state: &mut App_state) {
    ui::layout::base(f);
    ui::layout::method(f, app_state);
    ui::layout::work_space(f, app_state);
    ui::layout::url(f, app_state);
    ui::layout::features(f, app_state);
    ui::layout::response(f, app_state);
}
