use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::theme;

pub fn render_kill_confirm(frame: &mut Frame, app: &App, area: Rect) {
    let Some(confirm) = &app.confirm_kill else {
        return;
    };

    let sig_name = if confirm.force { "SIGKILL" } else { "SIGTERM" };

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("  Kill "),
            Span::styled(
                &confirm.process_name,
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ),
            Span::raw(format!(" (PID {}) on port {}?", confirm.pid, confirm.port)),
        ]),
        Line::from(format!("  Signal: {}", sig_name)),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("y", theme::HEADER_STYLE),
            Span::raw("/Enter: confirm    any other key: cancel"),
        ]),
        Line::from(""),
    ];

    let block = Block::default()
        .title(" Confirm Kill ")
        .borders(Borders::ALL)
        .border_style(theme::DIALOG_BORDER_STYLE);

    // Center the dialog
    let dialog_area = centered_rect(50, 8, area);

    frame.render_widget(Clear, dialog_area);
    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, dialog_area);
}

fn centered_rect(percent_x: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .split(area);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)])
        .flex(Flex::Center)
        .split(vertical[0]);
    horizontal[0]
}
