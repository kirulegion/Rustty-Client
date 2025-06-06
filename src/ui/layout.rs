use ratatui::text;
use ratatui::widgets::Wrap;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{
        Color::{self},
        Modifier, Style,
    },
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use reqwest::Result;

use crate::{
    network::client::handle_get,
    state::{
        self,
        app_state::{App_state, Focused, Response},
    },
};

pub fn base(f: &mut Frame) {
    let area = Rect::new(3, 1, f.size().width - 5, f.size().height - 1);
    let base = Block::new()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    f.render_widget(base, area);
}

pub fn method(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(5, 2, 30, 5);
    let mut block = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("  Method  ");

    let text = format!("{:?}", state.method);
    let is_focused = matches!(state.focused, Focused::Method);

    if is_focused {
        block = block.border_style(Style::default().fg(Color::Yellow));
    }
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
    let is_focused = matches!(state.focused, Focused::Workspace);

    f.render_widget(
        if is_focused {
            widget.border_style(Style::default().fg(Color::Yellow))
        } else {
            widget
        },
        area,
    );
}

pub fn features(f: &mut Frame, state: &mut App_state) {
    let area = Rect::new(36, 6, f.size().width - 40, 15);

    let is_focused = matches!(state.focused, Focused::Feature);
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
    f.render_widget(
        if is_focused {
            outer_block.border_style(Style::default().fg(Color::Yellow))
        } else {
            outer_block
        },
        area,
    );

    // 2. Render the tab bar
    let tabs = Tabs::new(vec![" Params    ", "    Body    ", "    Headers "])
        .select(state.selected_tab as usize)
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(Color::White));

    let mut inner_block = Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);

    f.render_widget(
        if is_focused {
            tabs.select(state.selected_tab as usize)
        } else {
            tabs.select(None)
        },
        chunks[0],
    );

    //creating the params list
    let items: Vec<ListItem> = state
        .params
        .iter()
        .map(|(k, v)| ListItem::new(format!("{}: {}", k, v)))
        .collect();

    // Create the List widget
    let list = List::new(items).highlight_symbol(">");

    //Rendering the tabs according to the selected tabs.
    match state.selected_tab {
        0 => {
            let content = Paragraph::new("").block(inner_block.title("Body"));
            f.render_widget(content, chunks[1])
        }
        1 => {
            inner_block = inner_block.title("Param");
            f.render_widget(list.block(inner_block), chunks[1]);
        }
        2 => {
            let content = Paragraph::new("").block(inner_block.title("Header"));

            f.render_widget(content, chunks[1]);
        }
        _ => {}
    }
}

pub fn response(f: &mut Frame, _state: &mut App_state, response: &mut Response) {
    let area = Rect::new(36, 21, f.size().width - 40, f.size().height - 22);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("  Response  ");

    let data = response.body.clone();
    let paragraph = Paragraph::new(data).block(widget);

    f.render_widget(paragraph, area);
}

pub fn help_modal(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled(
            "Help - Keybinds",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from("q        - Quit"),
        Line::from("?        - Toggle Help"),
        Line::from("Enter    - Submit"),
        Line::from("↑/↓      - Navigate"),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(Block::default().title("Help").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    let modal_area = centered_rect(60, 40, area);
    f.render_widget(paragraph, modal_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}
