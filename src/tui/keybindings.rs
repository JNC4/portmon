use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::data::types::SortColumn;
use crate::tui::app::{App, InputMode};

#[derive(Debug, Clone)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    PageUp,
    PageDown,
    Home,
    End,
    ToggleDetail,
    ToggleHelp,
    EnterFilterMode,
    ExitFilterMode,
    FilterInput(char),
    FilterBackspace,
    SortByColumn(SortColumn),
    Kill,
    ForceKill,
    ConfirmKill,
    Cancel,
    CopyPort,
    CopyPid,
    CopyCmdline,
    OpenInBrowser,
    Refresh,
    Noop,
}

pub fn handle_key(app: &App, key: KeyEvent) -> Action {
    // Ctrl+C always quits
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Action::Quit;
    }

    // Kill confirmation dialog
    if app.confirm_kill.is_some() {
        return match key.code {
            KeyCode::Char('y') | KeyCode::Enter => Action::ConfirmKill,
            _ => Action::Cancel,
        };
    }

    // Filter mode
    if app.input_mode == InputMode::Filter {
        return match key.code {
            KeyCode::Esc => Action::ExitFilterMode,
            KeyCode::Enter => Action::ExitFilterMode,
            KeyCode::Backspace => Action::FilterBackspace,
            KeyCode::Char(c) => Action::FilterInput(c),
            KeyCode::Up => Action::MoveUp,
            KeyCode::Down => Action::MoveDown,
            _ => Action::Noop,
        };
    }

    // Help overlay
    if app.show_help {
        return match key.code {
            KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q') => Action::ToggleHelp,
            _ => Action::Noop,
        };
    }

    // Normal mode
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
        KeyCode::Char('/') => Action::EnterFilterMode,
        KeyCode::Up | KeyCode::Char('k') => Action::MoveUp,
        KeyCode::Down | KeyCode::Char('j') => Action::MoveDown,
        KeyCode::PageUp => Action::PageUp,
        KeyCode::PageDown => Action::PageDown,
        KeyCode::Char('g') => Action::Home,
        KeyCode::Char('G') => Action::End,
        KeyCode::Home => Action::Home,
        KeyCode::End => Action::End,
        KeyCode::Char('d') => Action::ToggleDetail,
        KeyCode::Char('?') => Action::ToggleHelp,
        KeyCode::Char('x') => Action::Kill,
        KeyCode::Char('X') => Action::ForceKill,
        KeyCode::Char('y') => Action::CopyPort,
        KeyCode::Char('Y') => Action::CopyPid,
        KeyCode::Char('c') => Action::CopyCmdline,
        KeyCode::Char('o') => Action::OpenInBrowser,
        KeyCode::Char('r') => Action::Refresh,
        KeyCode::Char('1') => Action::SortByColumn(SortColumn::Protocol),
        KeyCode::Char('2') => Action::SortByColumn(SortColumn::LocalAddr),
        KeyCode::Char('3') => Action::SortByColumn(SortColumn::Port),
        KeyCode::Char('4') => Action::SortByColumn(SortColumn::Pid),
        KeyCode::Char('5') => Action::SortByColumn(SortColumn::ProcessName),
        KeyCode::Char('6') => Action::SortByColumn(SortColumn::User),
        KeyCode::Char('7') => Action::SortByColumn(SortColumn::Project),
        KeyCode::Char('8') => Action::SortByColumn(SortColumn::State),
        KeyCode::Char('9') => Action::SortByColumn(SortColumn::Command),
        _ => Action::Noop,
    }
}
