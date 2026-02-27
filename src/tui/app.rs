use std::time::{Duration, Instant};

use ratatui::widgets::TableState;

use crate::data::process::load_process_detail;
use crate::data::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Filter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneMode {
    Table,
    TableWithDetail,
}

pub struct KillConfirmation {
    pub pid: i32,
    pub process_name: String,
    pub port: u16,
    pub force: bool,
}

pub struct App {
    pub entries: Vec<PortEntry>,
    pub filtered_indices: Vec<usize>,
    pub table_state: TableState,

    pub input_mode: InputMode,
    pub pane_mode: PaneMode,

    pub filter_text: String,
    pub sort_column: SortColumn,
    pub sort_order: SortOrder,

    pub detail_cache: Option<ProcessDetail>,
    pub detail_pid: Option<i32>,

    pub refresh_interval: Duration,
    pub show_help: bool,
    pub confirm_kill: Option<KillConfirmation>,

    pub should_quit: bool,
    pub status_message: Option<(String, Instant)>,
    pub is_root: bool,
}

impl App {
    pub fn new(entries: Vec<PortEntry>, refresh_interval: Duration) -> Self {
        let is_root = nix::unistd::geteuid().is_root();
        let mut app = Self {
            entries,
            filtered_indices: Vec::new(),
            table_state: TableState::default(),
            input_mode: InputMode::Normal,
            pane_mode: PaneMode::Table,
            filter_text: String::new(),
            sort_column: SortColumn::Port,
            sort_order: SortOrder::Ascending,
            detail_cache: None,
            detail_pid: None,
            refresh_interval,
            show_help: false,
            confirm_kill: None,
            should_quit: false,
            status_message: None,
            is_root,
        };
        app.recompute_view();
        if !app.filtered_indices.is_empty() {
            app.table_state.select(Some(0));
        }
        app
    }

    pub fn update_entries(&mut self, entries: Vec<PortEntry>) {
        self.entries = entries;
        self.recompute_view();
    }

    pub fn recompute_view(&mut self) {
        let mut indices: Vec<usize> = if self.filter_text.is_empty() {
            (0..self.entries.len()).collect()
        } else {
            self.entries
                .iter()
                .enumerate()
                .filter(|(_, entry)| fuzzy_match(&self.filter_text, &entry.search_string()))
                .map(|(i, _)| i)
                .collect()
        };

        indices.sort_by(|&a, &b| {
            let ord = self.entries[a].cmp_by_column(&self.entries[b], self.sort_column);
            match self.sort_order {
                SortOrder::Ascending => ord,
                SortOrder::Descending => ord.reverse(),
            }
        });

        self.filtered_indices = indices;

        // Clamp selection
        if let Some(selected) = self.table_state.selected() {
            if selected >= self.filtered_indices.len() {
                let new = self.filtered_indices.len().checked_sub(1);
                self.table_state.select(new);
            }
        } else if !self.filtered_indices.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    pub fn selected_entry(&self) -> Option<&PortEntry> {
        self.table_state
            .selected()
            .and_then(|i| self.filtered_indices.get(i))
            .and_then(|&idx| self.entries.get(idx))
    }

    pub fn move_selection(&mut self, delta: i32) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let current = self.table_state.selected().unwrap_or(0) as i32;
        let max = self.filtered_indices.len() as i32 - 1;
        let new = (current + delta).clamp(0, max) as usize;
        self.table_state.select(Some(new));
        self.invalidate_detail_if_changed();
    }

    pub fn select_first(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.table_state.select(Some(0));
            self.invalidate_detail_if_changed();
        }
    }

    pub fn select_last(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.table_state
                .select(Some(self.filtered_indices.len() - 1));
            self.invalidate_detail_if_changed();
        }
    }

    fn invalidate_detail_if_changed(&mut self) {
        let current_pid = self.selected_entry().and_then(|e| e.pid());
        if current_pid != self.detail_pid {
            self.detail_cache = None;
            self.detail_pid = None;
        }
    }

    pub fn load_detail_for_selected(&mut self) {
        if let Some(entry) = self.selected_entry() {
            if let Some(pid) = entry.pid() {
                if self.detail_pid != Some(pid) {
                    self.detail_cache = load_process_detail(pid);
                    self.detail_pid = Some(pid);
                }
            }
        }
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_message = Some((msg, Instant::now()));
    }

    pub fn status_text(&self) -> Option<&str> {
        self.status_message.as_ref().and_then(|(msg, when)| {
            if when.elapsed() < Duration::from_secs(5) {
                Some(msg.as_str())
            } else {
                None
            }
        })
    }

    pub fn entry_count(&self) -> usize {
        self.filtered_indices.len()
    }

    pub fn total_count(&self) -> usize {
        self.entries.len()
    }
}

/// Simple fuzzy match: all characters in the pattern must appear in order in the haystack.
/// Case-insensitive.
fn fuzzy_match(pattern: &str, haystack: &str) -> bool {
    let pattern = pattern.to_lowercase();
    let haystack = haystack.to_lowercase();
    let mut haystack_chars = haystack.chars();
    for pc in pattern.chars() {
        loop {
            match haystack_chars.next() {
                Some(hc) if hc == pc => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}
