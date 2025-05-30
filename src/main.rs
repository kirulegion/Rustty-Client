use color_eyre::eyre::Result;
use ratatui::{crossterm::event::{self, Event}, DefaultTerminal, Frame};

//Importing all the folder and file struct
mod state;
mod network;
mod ui;
mod utils;
mod App;
mod events;

fn main()->Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();

    result
}

fn run (mut terminal : DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|f| render(f))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

//Rednering all the ui elements.
fn render(f : &mut Frame ) {
    ui::layout::base(f);
    ui::layout::method(f);
    ui::layout::work_space(f);
    ui::layout::url(f);
    ui::layout::features(f);
    ui::layout::response(f);
}
