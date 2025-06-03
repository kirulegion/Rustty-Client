use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{
        Color::{self, Rgb},
        Modifier, Style, Stylize,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Tabs},
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
        .title("  Method  ");

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
    //checking wether the current block is selected or not.
    let is_focused = matches!(state.focused, Focused::Url);
    let area = Rect::new(36, 2, f.size().width - 40, 4);
    let block = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("  Url  ")
        //If the current block is selected the bg will be green instead of white.
        .border_style(if is_focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let widget = Paragraph::new(state.url_input.as_str()).block(block);

    f.render_widget(widget, area);

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
        .title("  Workspace  ");

    f.render_widget(widget, area);
}

pub fn features(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(36, 6, f.size().width - 40, 15);

    let outer_block = Block::new()
        .borders(Borders::ALL)
        .title("  features  ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    // Split the area into tab header and content area
    let inner_area = outer_block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(inner_area);

    // 1. Render the block
    f.render_widget(outer_block, area);

    // 2. Render the tab bar
    let tabs = Tabs::new(vec![" Params    ", "    Body    ", "    Headers "])
        .select(state.selected_tab as usize)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(tabs, chunks[0]);

    let inner_block = Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    match state.selected_tab {
        0 => {
            let content = Paragraph::new("").block(inner_block.title("Param"));
            f.render_widget(content, chunks[1])
        }
        1 => {
            let content = Paragraph::new("").block(inner_block.title("Body"));
            f.render_widget(content, chunks[1]);
        }
        2 => {
            let content = Paragraph::new("").block(inner_block.title("Header"));

            f.render_widget(content, chunks[1]);
        }
        _ => {}
    }
}

pub fn response(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(36, 21, f.size().width - 40, f.size().height - 22);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("  Response  ");

    f.render_widget(widget, area);
}
