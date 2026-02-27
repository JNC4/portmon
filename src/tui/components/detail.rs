use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::tui::app::App;
use crate::tui::theme;

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let Some(detail) = &app.detail_cache else {
        let msg = if app.selected_entry().is_some() {
            "No detail available (process info unavailable)"
        } else {
            "No entry selected"
        };
        let block = Block::default()
            .borders(Borders::TOP)
            .border_style(theme::BORDER_STYLE)
            .title(" Detail ");
        let paragraph = Paragraph::new(msg).block(block);
        frame.render_widget(paragraph, area);
        return;
    };

    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        Span::styled("PID:     ", theme::DETAIL_LABEL_STYLE),
        Span::raw(detail.pid.to_string()),
        Span::raw("  "),
        Span::styled("Name: ", theme::DETAIL_LABEL_STYLE),
        Span::raw(&detail.name),
    ]));

    lines.push(Line::from(vec![
        Span::styled("Command: ", theme::DETAIL_LABEL_STYLE),
        Span::raw(detail.cmdline.join(" ")),
    ]));

    if let Some(ref cwd) = detail.cwd {
        lines.push(Line::from(vec![
            Span::styled("CWD:     ", theme::DETAIL_LABEL_STYLE),
            Span::raw(cwd.display().to_string()),
        ]));
    }

    let mut info_parts = Vec::new();
    info_parts.push(Span::styled("FDs: ", theme::DETAIL_LABEL_STYLE));
    info_parts.push(Span::raw(format!("{}", detail.open_fds)));

    if let Some(threads) = detail.threads {
        info_parts.push(Span::raw("  "));
        info_parts.push(Span::styled("Threads: ", theme::DETAIL_LABEL_STYLE));
        info_parts.push(Span::raw(format!("{}", threads)));
    }

    if let Some(rss) = detail.memory_rss_bytes {
        info_parts.push(Span::raw("  "));
        info_parts.push(Span::styled("RSS: ", theme::DETAIL_LABEL_STYLE));
        info_parts.push(Span::raw(format!("{} MB", rss / (1024 * 1024))));
    }

    if !detail.children.is_empty() {
        info_parts.push(Span::raw("  "));
        info_parts.push(Span::styled("Children: ", theme::DETAIL_LABEL_STYLE));
        info_parts.push(Span::raw(format!("{}", detail.children.len())));
    }

    lines.push(Line::from(info_parts));

    if let Some(ref cid) = detail.container_id {
        lines.push(Line::from(vec![
            Span::styled("Container: ", theme::DETAIL_LABEL_STYLE),
            Span::raw(cid.as_str()),
        ]));
    }

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(theme::BORDER_STYLE)
        .title(" Detail ");

    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}
