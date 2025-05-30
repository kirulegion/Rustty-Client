use ratatui::{ layout::{Alignment, Constraint, Direction, Layout, Rect}, text::Line, widgets::{self, block::Position, Block, BorderType, Borders, Paragraph}, Frame};

pub fn base(f : &mut Frame){
    let area = Rect::new(3, 1, f.size().width - 5, f.size().height-1);
    let base = Block::new()
        .borders(Borders::all())
        .border_type(BorderType::Rounded);

    f.render_widget(base, area);
}

pub fn method(f : &mut Frame) {
    let area = Rect::new(5,2,30,4);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("Method [M]");

    f.render_widget(widget, area);
}


pub fn url(f : &mut Frame) {
    let area = Rect::new(36,2,f.size().width - 40,4);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("Url [U]");

    f.render_widget(widget, area);
}

pub fn work_space(f : &mut Frame) {
    let area = Rect::new(5,6,30,f.size().height-7);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("Workspace [W]");

    f.render_widget(widget, area);
}


pub fn features(f : &mut Frame) {
    let area = Rect::new(36,6,f.size().width-40, 15);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("features");

    f.render_widget(widget, area);
}


pub fn response(f : &mut Frame) {
    let area = Rect::new(36,21,f.size().width-40, f.size().height-7);
    let widget = Block::new()
        .borders(Borders::all())
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .title("Response");

    f.render_widget(widget, area);
}

