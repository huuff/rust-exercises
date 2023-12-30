use ratatui::layout::{Layout, Direction, Constraint, Rect};


/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let vertical_layout = Layout::new(Direction::Vertical, [
	Constraint::Percentage((100 - percent_y) / 2),
	Constraint::Percentage(percent_y),
	Constraint::Percentage((100 - percent_y) / 2),
    ]);

    // Cut the middle vertical piece into three width-wise pieces
    let horizontal_layout = Layout::new(Direction::Horizontal, [
	Constraint::Percentage((100 - percent_x) / 2),
	Constraint::Percentage(percent_x),
	Constraint::Percentage((100 - percent_x) / 2),
    ]);

   horizontal_layout.split(vertical_layout.split(r)[1])[1]
}
