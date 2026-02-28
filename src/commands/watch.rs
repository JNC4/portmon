use std::time::Duration;

use crate::data::collector::collect_port_entries;
use crate::data::types::PortEntry;

pub async fn run(port: u16, interval: Duration) -> anyhow::Result<()> {
    let mut tick = tokio::time::interval(interval);

    println!("Watching port {}... (Ctrl+C to stop)", port);

    // Initial scan
    let entries = tokio::task::spawn_blocking(collect_port_entries).await??;
    let mut last_state: Option<PortEntry> = entries.into_iter().find(|e| e.port == port);
    if let Some(ref entry) = last_state {
        println!(
            "[{}] Port {} is currently bound by {} (PID {})",
            timestamp(),
            port,
            entry.process_name(),
            entry.pid_str(),
        );
    } else {
        println!("[{}] Port {} is currently free", timestamp(), port);
    }

    loop {
        tick.tick().await;

        let entries = tokio::task::spawn_blocking(collect_port_entries).await??;
        let current = entries.into_iter().find(|e| e.port == port);

        match (&last_state, &current) {
            (None, Some(entry)) => {
                println!(
                    "[{}] BIND  port {} <- {} (PID {})",
                    timestamp(),
                    port,
                    entry.process_name(),
                    entry.pid_str(),
                );
            }
            (Some(_), None) => {
                println!("[{}] FREE  port {}", timestamp(), port);
            }
            (Some(prev), Some(curr)) => {
                let prev_pid = prev.pid();
                let curr_pid = curr.pid();
                if prev_pid != curr_pid {
                    println!(
                        "[{}] CHANGE port {}: {} (PID {}) -> {} (PID {})",
                        timestamp(),
                        port,
                        prev.process_name(),
                        prev.pid_str(),
                        curr.process_name(),
                        curr.pid_str(),
                    );
                }
            }
            (None, None) => {}
        }

        last_state = current;
    }
}

fn timestamp() -> String {
    unsafe {
        let mut t: nix::libc::time_t = 0;
        nix::libc::time(&mut t);
        let tm = nix::libc::localtime(&t);
        if tm.is_null() {
            return "??:??:??".to_string();
        }
        format!(
            "{:02}:{:02}:{:02}",
            (*tm).tm_hour,
            (*tm).tm_min,
            (*tm).tm_sec
        )
    }
}
