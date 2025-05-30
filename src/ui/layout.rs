use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn layout_one (area :Rect)->(Rect , Rect) {

    let method_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(8) , Constraint::Percentage(92)])
        .split(area);
    (method_block[0] , method_block[1])
}

