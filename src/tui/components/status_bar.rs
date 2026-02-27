use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let mut spans = vec![Span::styled(" portmonitor", theme::HEADER_STYLE)];

    // Status message or default info
    if let Some(msg) = app.status_text() {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(msg, theme::FILTER_STYLE));
    }

    // Right-aligned info
    let count_text = if app.filter_text.is_empty() {
        format!("{} ports", app.total_count())
    } else {
        format!("{}/{} ports", app.entry_count(), app.total_count())
    };

    let interval_text = format!(" [{}s]", app.refresh_interval.as_secs_f64());

    let right_info = if !app.is_root {
        format!("  {}  {}  [unprivileged]", count_text, interval_text)
    } else {
        format!("  {}  {}", count_text, interval_text)
    };

    // Calculate padding
    let left_len: usize = spans.iter().map(|s| s.content.len()).sum();
    let right_len = right_info.len();
    let padding = area
        .width
        .saturating_sub(left_len as u16 + right_len as u16);

    spans.push(Span::raw(" ".repeat(padding as usize)));
    spans.push(Span::raw(right_info));

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(theme::STATUS_STYLE);
    frame.render_widget(paragraph, area);
}
