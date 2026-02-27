use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::tui::app::{App, InputMode};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let line = if app.input_mode == InputMode::Filter {
        Line::from(vec![
            key_span("Esc"),
            desc_span(" close  "),
            key_span("↑↓"),
            desc_span(" navigate  "),
            key_span("Enter"),
            desc_span(" confirm"),
        ])
    } else {
        Line::from(vec![
            key_span("q"),
            desc_span(":quit "),
            key_span("/"),
            desc_span(":filter "),
            key_span("x"),
            desc_span(":kill "),
            key_span("X"),
            desc_span(":KILL "),
            key_span("d"),
            desc_span(":detail "),
            key_span("y"),
            desc_span(":cp-port "),
            key_span("Y"),
            desc_span(":cp-pid "),
            key_span("c"),
            desc_span(":cp-cmd "),
            key_span("o"),
            desc_span(":browser "),
            key_span("1-9"),
            desc_span(":sort "),
            key_span("?"),
            desc_span(":help"),
        ])
    };

    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}

pub fn render_help_overlay(frame: &mut Frame, _app: &App, area: Rect) {
    let lines = vec![
        Line::from(""),
        Line::from(vec![key_span("  Navigation")]),
        Line::from(vec![
            key_span("  j/↓  "),
            desc_span("Move down"),
        ]),
        Line::from(vec![
            key_span("  k/↑  "),
            desc_span("Move up"),
        ]),
        Line::from(vec![
            key_span("  g    "),
            desc_span("Go to top"),
        ]),
        Line::from(vec![
            key_span("  G    "),
            desc_span("Go to bottom"),
        ]),
        Line::from(vec![
            key_span("  PgUp/PgDn "),
            desc_span("Page up/down"),
        ]),
        Line::from(""),
        Line::from(vec![key_span("  Actions")]),
        Line::from(vec![
            key_span("  /    "),
            desc_span("Filter (fuzzy search)"),
        ]),
        Line::from(vec![
            key_span("  x    "),
            desc_span("Kill process (SIGTERM)"),
        ]),
        Line::from(vec![
            key_span("  X    "),
            desc_span("Force kill (SIGKILL)"),
        ]),
        Line::from(vec![
            key_span("  d    "),
            desc_span("Toggle detail pane"),
        ]),
        Line::from(vec![
            key_span("  o    "),
            desc_span("Open in browser"),
        ]),
        Line::from(vec![
            key_span("  r    "),
            desc_span("Manual refresh"),
        ]),
        Line::from(""),
        Line::from(vec![key_span("  Clipboard")]),
        Line::from(vec![
            key_span("  y    "),
            desc_span("Copy port"),
        ]),
        Line::from(vec![
            key_span("  Y    "),
            desc_span("Copy PID"),
        ]),
        Line::from(vec![
            key_span("  c    "),
            desc_span("Copy command"),
        ]),
        Line::from(""),
        Line::from(vec![key_span("  Sorting")]),
        Line::from(vec![
            key_span("  1-8  "),
            desc_span("Sort by column (press again to reverse)"),
        ]),
        Line::from(""),
        Line::from(vec![
            key_span("  q/Esc"),
            desc_span(" Close help"),
        ]),
    ];

    let block = ratatui::widgets::Block::default()
        .title(" Help ")
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(Style::default().fg(ratatui::style::Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn key_span(s: &str) -> Span<'_> {
    Span::styled(
        s,
        Style::default()
            .fg(ratatui::style::Color::Cyan)
            .add_modifier(ratatui::style::Modifier::BOLD),
    )
}

fn desc_span(s: &str) -> Span<'_> {
    Span::styled(s, Style::default().fg(ratatui::style::Color::DarkGray))
}

