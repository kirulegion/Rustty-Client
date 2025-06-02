use ratatui::{
    layout::{Alignment, Rect},
    style::Color::Rgb,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::state::app_state::{App_state, Focused};

pub fn base(f: &mut Frame) {
    let area = Rect::new(3, 1, f.size().width - 5, f.size().height - 1);
    let base = Block::new()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    f.render_widget(base, area);
}

pub fn method(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(5, 2, 30, 5);
    let block = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("Method [M]");

    let text = format!("{:?}", state.method);

    // Total lines = 4 (height of area), 1 is content, so 3 left
    // Add 1 empty line before and 2 after (or 2 before and 1 after)
    let lines = vec![
        Line::raw(""),               // top padding
        Line::from(Span::raw(text)), // centered text
    ];

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

pub fn url(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(36, 2, f.size().width - 40, 4);
    let block = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("Url [U]");

    let widget = Paragraph::new(state.url_input.as_str());
    match state.focused {
        Focused::Url => {
            f.render_widget(
                widget.block(block.border_style(Style::default().fg(Rgb(144, 238, 144)))),
                area,
            );
        }
        _ => {
            f.render_widget(widget.block(block), area);
        }
    }
    //Rendering the cursor for the url block
    if let Focused::Url = state.focused {
        // Show cursor only if focused
        let x = area.x + state.url_input.len() as u16 + 1;
        let y = area.y + 1;
        f.set_cursor(x, y);
    }
}

pub fn work_space(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(5, 8, 30, f.size().height - 9);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("Workspace [W]");

    f.render_widget(widget, area);
}

pub fn features(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(36, 6, f.size().width - 40, 15);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("features");

    f.render_widget(widget, area);
}

pub fn response(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(36, 21, f.size().width - 40, f.size().height - 22);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("Response");

    f.render_widget(widget, area);
}
