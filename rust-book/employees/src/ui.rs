use std::{io::{self, Stdout}, collections::HashMap};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand as _,
};
use ratatui::{
    layout::SegmentSize,
    prelude::*,
    widgets::{Block, Borders, Row, Table, TableState},
};

use crate::{Department, scene::{Scene, EmployeeSet}, Employee};

pub fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Fullscreen,
        },
    )?;
    Ok(terminal)
}

pub fn close_terminal() -> anyhow::Result<()> {
    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn render(f: &mut Frame, scene: &mut Scene) {
    let outer_block = Block::default().title("Employees").borders(Borders::ALL);
    f.render_widget(outer_block, f.size());

    let vertical_layout = Layout::new(Direction::Vertical, Constraint::from_mins([0, 20, 0]))
        .segment_size(SegmentSize::EvenDistribution);

    let horizontal_layout = Layout::new(Direction::Horizontal, Constraint::from_mins([0, 20, 0]))
        .segment_size(SegmentSize::EvenDistribution);

    let center_rect = horizontal_layout.split(vertical_layout.split(f.size())[1])[1];
    match scene {
        Scene::DepartmentList { state, department_to_employees, .. } => {
	    render_department_table(f, department_to_employees, state, center_rect);
	}
        Scene::DepartmentView { department, employees, state } => {
	    render_department_view(f, *department, employees, state,  center_rect)
	},
    }
}

pub fn render_department_table(
    f: &mut Frame,
    department_to_employees: &HashMap<Department, EmployeeSet>,
    table_state: &mut TableState,
    target_area: Rect,
) {
    let widths = [Constraint::default(), Constraint::Length(10)];
    let rows = Department::all().iter()
        .map(|department| {
            [
                department.to_string(),
                department_to_employees
                    .get(&department)
                    .map(|employees| employees.len().to_string())
                    .unwrap_or("0".to_string()),
            ]
        })
        .map(|t| Row::new(t));

    let table = Table::new(rows, widths)
        .header(
            Row::new(["Department", "Employees"])
                .style(Style::new().bold())
                .add_modifier(Modifier::UNDERLINED),
        )
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::new().on_dark_gray())
        .segment_size(SegmentSize::EvenDistribution);

    f.render_stateful_widget(table, target_area, table_state);
}

pub fn render_department_view(
    f: &mut Frame,
    department: Department,
    employees: &EmployeeSet,
    state: &mut TableState,
    target_area: Rect
) {
    let widths = [Constraint::default(), Constraint::Length(10)];
    let rows = employees.iter()
        .map(|Employee { name, salary }| [name.clone(), salary.to_string()])
        .map(|t| Row::new(t));

    let table = Table::new(rows, widths)
        .header(
            Row::new(["Employee", "Salary"])
                .style(Style::new().bold())
                .add_modifier(Modifier::UNDERLINED),
        )
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::new().on_dark_gray())
        .segment_size(SegmentSize::EvenDistribution);

    f.render_stateful_widget(table, target_area, state);
}
