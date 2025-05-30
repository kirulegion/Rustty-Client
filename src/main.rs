use color_eyre::eyre::Result;
use ratatui::{crossterm::event::{self, Event, KeyCode}, layout::Rect, widgets::{block::title, Block}, DefaultTerminal, Frame};

//Importing all the folder and file struct
mod state;
mod network;
mod ui;
mod utils;
mod App;


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

fn render(frame : &mut Frame) {

    //creating costume area for the methods and request history block
    let meth_req_area = Rect::new(1 , 1, 30 , frame.size().height);
    let (meth_block , request_history) = ui::layout::layout_one(meth_req_area);
    frame.render_widget(Block::bordered().title("Method"), meth_block);
    frame.render_widget(Block::bordered().title("Requests"), request_history);

    
}
