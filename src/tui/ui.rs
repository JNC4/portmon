use ratatui::layout::{Constraint, Layout};
use ratatui::Frame;

use crate::tui::app::{App, InputMode, PaneMode};
use crate::tui::components::{detail, dialog, filter_bar, help_bar, status_bar, table};

pub fn render(frame: &mut Frame, app: &mut App) {
    let has_filter = app.input_mode == InputMode::Filter || !app.filter_text.is_empty();
    let has_detail = app.pane_mode == PaneMode::TableWithDetail;

    let mut constraints = vec![Constraint::Length(1)]; // status bar

    if has_filter {
        constraints.push(Constraint::Length(1)); // filter bar
    }

    if has_detail {
        constraints.push(Constraint::Percentage(60)); // table
        constraints.push(Constraint::Percentage(40)); // detail
    } else {
        constraints.push(Constraint::Min(3)); // table
    }

    constraints.push(Constraint::Length(1)); // help bar

    let chunks = Layout::vertical(constraints).split(frame.area());

    let mut idx = 0;

    // Status bar
    status_bar::render(frame, app, chunks[idx]);
    idx += 1;

    // Filter bar (conditional)
    if has_filter {
        filter_bar::render(frame, app, chunks[idx]);
        idx += 1;
    }

    // Table
    table::render(frame, app, chunks[idx]);
    idx += 1;

    // Detail pane (conditional)
    if has_detail {
        app.load_detail_for_selected();
        detail::render(frame, app, chunks[idx]);
        idx += 1;
    }

    // Help bar
    help_bar::render(frame, app, chunks[idx]);

    // Kill confirmation dialog (overlay)
    if app.confirm_kill.is_some() {
        dialog::render_kill_confirm(frame, app, frame.area());
    }

    // Help overlay
    if app.show_help {
        let area = frame.area();
        let help_area = ratatui::layout::Layout::vertical([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(area);
        let help_area = ratatui::layout::Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(help_area[1]);

        frame.render_widget(ratatui::widgets::Clear, help_area[1]);
        help_bar::render_help_overlay(frame, app, help_area[1]);
    }
}
