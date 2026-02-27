use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::tui::app::{App, InputMode};
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    if app.input_mode != InputMode::Filter && app.filter_text.is_empty() {
        return;
    }

    let cursor_char = if app.input_mode == InputMode::Filter {
        "█"
    } else {
        ""
    };

    let line = Line::from(vec![
        Span::styled(" /", theme::FILTER_STYLE),
        Span::raw(&app.filter_text),
        Span::styled(cursor_char, theme::FILTER_STYLE),
    ]);

    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, area);
}
