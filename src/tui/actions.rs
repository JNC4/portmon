use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

use crate::data::collector::collect_port_entries;
use crate::tui::app::{App, InputMode, KillConfirmation, PaneMode};
use crate::tui::keybindings::Action;

pub fn handle_action(app: &mut App, action: Action) {
    match action {
        Action::Quit => app.should_quit = true,
        Action::MoveUp => app.move_selection(-1),
        Action::MoveDown => app.move_selection(1),
        Action::PageUp => app.move_selection(-20),
        Action::PageDown => app.move_selection(20),
        Action::Home => app.select_first(),
        Action::End => app.select_last(),
        Action::ToggleDetail => {
            app.pane_mode = match app.pane_mode {
                PaneMode::Table => {
                    app.load_detail_for_selected();
                    PaneMode::TableWithDetail
                }
                PaneMode::TableWithDetail => PaneMode::Table,
            };
        }
        Action::ToggleHelp => {
            app.show_help = !app.show_help;
        }
        Action::EnterFilterMode => {
            app.input_mode = InputMode::Filter;
        }
        Action::ExitFilterMode => {
            app.input_mode = InputMode::Normal;
        }
        Action::FilterInput(c) => {
            app.filter_text.push(c);
            app.recompute_view();
        }
        Action::FilterBackspace => {
            app.filter_text.pop();
            app.recompute_view();
        }
        Action::FilterClear => {
            app.filter_text.clear();
            app.recompute_view();
        }
        Action::SortByColumn(col) => {
            if app.sort_column == col {
                app.sort_order = app.sort_order.toggle();
            } else {
                app.sort_column = col;
                app.sort_order = crate::data::types::SortOrder::Ascending;
            }
            app.recompute_view();
        }
        Action::Kill => {
            if let Some(entry) = app.selected_entry() {
                if let Some(process) = &entry.process {
                    app.confirm_kill = Some(KillConfirmation {
                        pid: process.pid,
                        process_name: process.name.clone(),
                        port: entry.port,
                        force: false,
                    });
                } else {
                    app.set_status("No process info available".to_string());
                }
            }
        }
        Action::ForceKill => {
            if let Some(entry) = app.selected_entry() {
                if let Some(process) = &entry.process {
                    app.confirm_kill = Some(KillConfirmation {
                        pid: process.pid,
                        process_name: process.name.clone(),
                        port: entry.port,
                        force: true,
                    });
                } else {
                    app.set_status("No process info available".to_string());
                }
            }
        }
        Action::ConfirmKill => {
            if let Some(confirm) = app.confirm_kill.take() {
                let sig = if confirm.force {
                    Signal::SIGKILL
                } else {
                    Signal::SIGTERM
                };
                let sig_name = if confirm.force { "SIGKILL" } else { "SIGTERM" };

                match signal::kill(Pid::from_raw(confirm.pid), sig) {
                    Ok(()) => {
                        app.set_status(format!(
                            "Sent {} to {} (PID {}) on port {}",
                            sig_name, confirm.process_name, confirm.pid, confirm.port
                        ));
                        // Refresh data immediately
                        if let Ok(entries) = collect_port_entries() {
                            app.update_entries(entries);
                        }
                    }
                    Err(e) => {
                        app.set_status(format!("Kill failed: {}", e));
                    }
                }
            }
        }
        Action::CancelAction => {
            app.confirm_kill = None;
        }
        Action::CopyPort => {
            if let Some(entry) = app.selected_entry() {
                let text = entry.port.to_string();
                copy_to_clipboard(app, &text, "port");
            }
        }
        Action::CopyPid => {
            if let Some(entry) = app.selected_entry() {
                let text = entry.pid_str();
                copy_to_clipboard(app, &text, "PID");
            }
        }
        Action::CopyCmdline => {
            if let Some(entry) = app.selected_entry() {
                let text = entry.cmdline().to_string();
                copy_to_clipboard(app, &text, "command");
            }
        }
        Action::OpenInBrowser => {
            if let Some(entry) = app.selected_entry() {
                let url = format!("http://localhost:{}", entry.port);
                match open::that(&url) {
                    Ok(()) => app.set_status(format!("Opened {}", url)),
                    Err(e) => app.set_status(format!("Failed to open browser: {}", e)),
                }
            }
        }
        Action::Refresh => {
            if let Ok(entries) = collect_port_entries() {
                app.update_entries(entries);
                app.set_status("Refreshed".to_string());
            }
        }
        Action::Noop => {}
    }
}

fn copy_to_clipboard(app: &mut App, text: &str, label: &str) {
    match cli_clipboard::set_contents(text.to_string()) {
        Ok(()) => app.set_status(format!("Copied {} to clipboard: {}", label, text)),
        Err(e) => app.set_status(format!("Clipboard error: {}", e)),
    }
}
