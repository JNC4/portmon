use ratatui::layout::Constraint;
use ratatui::style::Modifier;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Row, Table};
use ratatui::Frame;

use crate::data::types::SortColumn;
use crate::tui::app::App;
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let header_cells = [
        SortColumn::Protocol,
        SortColumn::LocalAddr,
        SortColumn::Port,
        SortColumn::Pid,
        SortColumn::ProcessName,
        SortColumn::User,
        SortColumn::Project,
        SortColumn::State,
        SortColumn::Command,
    ]
    .iter()
    .map(|col| {
        let label = if *col == app.sort_column {
            format!("{} {}", col.label(), app.sort_order.arrow())
        } else {
            col.label().to_string()
        };
        Cell::from(Line::from(Span::styled(label, theme::HEADER_STYLE)))
    });

    let header = Row::new(header_cells).height(1);

    let rows = app.filtered_indices.iter().map(|&idx| {
        let entry = &app.entries[idx];
        let style = theme::row_style(entry);

        let cells = vec![
            Cell::from(entry.protocol.to_string()),
            Cell::from(entry.local_addr.to_string()),
            Cell::from(entry.port.to_string()),
            Cell::from(entry.pid_str()),
            Cell::from(entry.process_name().to_string()),
            Cell::from(entry.username.clone()),
            Cell::from(truncate(entry.project(), 20)),
            Cell::from(entry.state.short().to_string()),
            Cell::from(truncate(entry.cmdline(), 40)),
        ];

        Row::new(cells).style(style)
    });

    let widths = [
        Constraint::Length(5),  // Proto
        Constraint::Length(16), // Local Addr
        Constraint::Length(6),  // Port
        Constraint::Length(7),  // PID
        Constraint::Length(15), // Name
        Constraint::Length(10), // User
        Constraint::Length(20), // Project
        Constraint::Length(4),  // State
        Constraint::Min(20),    // Command (fills remaining)
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::NONE))
        .row_highlight_style(theme::SELECTED_STYLE.add_modifier(Modifier::BOLD));

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}
