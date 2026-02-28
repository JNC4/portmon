use ratatui::style::{Color, Modifier, Style};

use crate::data::types::PortEntry;

pub const HEADER_STYLE: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);

pub const SELECTED_STYLE: Style = Style::new()
    .bg(Color::DarkGray)
    .add_modifier(Modifier::BOLD);

pub const FILTER_STYLE: Style = Style::new().fg(Color::Yellow);

pub const HELP_STYLE: Style = Style::new().fg(Color::DarkGray);

pub const STATUS_STYLE: Style = Style::new().fg(Color::White).bg(Color::DarkGray);

pub const BORDER_STYLE: Style = Style::new().fg(Color::DarkGray);

pub const DETAIL_LABEL_STYLE: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);

pub const DIALOG_BORDER_STYLE: Style = Style::new().fg(Color::Red);

pub fn row_style(entry: &PortEntry) -> Style {
    if entry.uid == 0 {
        // Root processes in red
        Style::default().fg(Color::Red)
    } else if entry.port < 1024 {
        // Well-known ports in yellow
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    }
}

pub fn sort_indicator_style() -> Style {
    Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD)
}
