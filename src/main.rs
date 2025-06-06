use std::{error::Error, time::Duration};

use color_eyre::eyre::Result;
use events::handler::{
    handle_features, handle_method, handle_modes, handle_request_processing, handle_url,
};
use ratatui::{
    crossterm::event::{self, Event},
    DefaultTerminal, Frame,
};
use state::app_state::{App_state, Focused, Response};

//Importing all the folder and file struct
mod events;
mod network;
mod state;
mod ui;
mod utils;

#[tokio::main]
async fn main() ->Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal).await;
    ratatui::restore();

    result
}

async fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app_state = App_state::new();
    let mut response_state = Response::new();

    while app_state.is_running {
        if event::poll(Duration::from_millis(10))? {
            handler_key_events(&mut app_state , &mut response_state).await;
        }

        // Then draw UI
        terminal.draw(|f| {
            render(f, &mut app_state , &mut response_state);
        }).unwrap();
    }

    Ok(())
}

async fn handler_key_events(app_state: &mut App_state ,  response :&mut Response)  -> Result<()> {
    if let Event::Key(key) = event::read()? {
        let mode_switch = handle_modes(key, app_state);

        if mode_switch {
            return Ok(());
        }

        match app_state.focused {
            Focused::Normal => {
                handle_request_processing(key, app_state , response).await;
            }
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

fn render (f: &mut Frame , app_state: &mut App_state , response :&mut Response) {
    ui::layout::base(f);
    ui::layout::method(f, app_state);
    ui::layout::work_space(f, app_state);
    ui::layout::url(f, app_state);
    ui::layout::features(f, app_state);
    ui::layout::response(f , app_state , response);
}
