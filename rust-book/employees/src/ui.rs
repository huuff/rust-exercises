use std::{io::{self, Stdout}, collections::HashMap};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand as _,
};
use ratatui::{
    layout::SegmentSize,
    prelude::*,
    widgets::{Block, Borders, Row, Table, TableState, Cell, List},
};

use crate::{Department, scene::{Scene, DepartmentList, DepartmentView}, Employee, App};

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

pub fn render<'a, T: Iterator<Item=&'a Employee>>(f: &mut Frame, scene: &mut Scene<'a, T>, app: &App) {
    let outer_block = Block::default().title("Employees").borders(Borders::ALL);
    f.render_widget(outer_block, f.size());

    let vertical_layout = Layout::new(Direction::Vertical, Constraint::from_mins([0, 20, 0]))
        .segment_size(SegmentSize::EvenDistribution);

    let horizontal_layout = Layout::new(Direction::Horizontal, Constraint::from_mins([0, 20, 0]))
        .segment_size(SegmentSize::EvenDistribution);

    let bottom_right = horizontal_layout.split(vertical_layout.split(f.size())[2])[2];
    render_keybindings(f, app, bottom_right);

    let center_rect = horizontal_layout.split(vertical_layout.split(f.size())[1])[1];
    match scene {
        Scene::List(DepartmentList { departments_to_employees, state, .. }) => {
	    render_department_table(f, departments_to_employees, state, center_rect);
	}
        Scene::View(DepartmentView { department, state, employees }) => {
	    render_department_view(f, department, employees, state,  center_rect)
	},
    }
}

pub fn render_department_table(
    f: &mut Frame,
    department_to_employees: &HashMap<Department, u32>,
    table_state: &mut TableState,
    target_area: Rect,
) {
    let widths = [Constraint::default(), Constraint::Length(10)];
    let rows = department_to_employees
        .iter()
        .map(|(department, num_employees): (&'static str, String)| Row::new([Cell::new(department), Cell::new(num_employees)]));

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

pub fn render_department_view<'a, T: Iterator<Item=&'a Employee>>(
    f: &mut Frame,
    department: &Department, 
    employees: T,
    state: &mut TableState,
    target_area: Rect
) {
    let widths = [Constraint::default(), Constraint::Length(10)];
    let rows = employees.iter()
	// TODO: Maybe add id
        .map(|Employee { name, salary, .. }| [name.clone(), salary.to_string()])
        .map(|t| Row::new(t));

    let table = Table::new(rows, widths)
        .header(
            Row::new(["Employee", "Salary"])
                .style(Style::new().bold())
                .add_modifier(Modifier::UNDERLINED),
        )
        .block(Block::default()
	       .borders(Borders::ALL)
	       .title::<&'static str>(department.into())
	)
        .highlight_style(Style::new().on_dark_gray())
        .segment_size(SegmentSize::EvenDistribution);

    f.render_stateful_widget(table, target_area, state);
}

pub fn render_keybindings(f: &mut Frame, app: &App, target_area: Rect) {
    let list_items = vec![
	"Use arrow keys to move around.".to_string(),
	"Press q to exit.".to_string(),
	match &app.selected_employee {
		Some(employee) => format!("Press enter to move {} here.", employee.name),
		None => "Press enter to select.".to_string(),
	}
    ];
    
    let list = List::new(list_items)
        .block(Block::new()
	       .title("Keybindings")
	       .borders(Borders::ALL)
	)
	;


    f.render_widget(list, target_area);
} 
