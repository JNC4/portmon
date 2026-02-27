use tabled::{Table, Tabled};

use crate::data::types::PortEntry;

#[derive(Tabled)]
struct Row {
    #[tabled(rename = "PROTO")]
    proto: String,
    #[tabled(rename = "LOCAL ADDR")]
    local_addr: String,
    #[tabled(rename = "PORT")]
    port: u16,
    #[tabled(rename = "PID")]
    pid: String,
    #[tabled(rename = "PROCESS")]
    process: String,
    #[tabled(rename = "USER")]
    user: String,
    #[tabled(rename = "PROJECT")]
    project: String,
    #[tabled(rename = "STATE")]
    state: String,
    #[tabled(rename = "COMMAND")]
    command: String,
}

pub fn print(entries: &[PortEntry]) -> anyhow::Result<()> {
    let rows: Vec<Row> = entries
        .iter()
        .map(|e| {
            let cmd = e.cmdline();
            // Truncate long commands for table display
            let command = if cmd.len() > 60 {
                let mut end = 57;
                while !cmd.is_char_boundary(end) {
                    end -= 1;
                }
                format!("{}...", &cmd[..end])
            } else {
                cmd.to_string()
            };

            Row {
                proto: e.protocol.to_string(),
                local_addr: e.local_addr.to_string(),
                port: e.port,
                pid: e.pid_str(),
                process: e.process_name().to_string(),
                user: e.username.clone(),
                project: e.project().to_string(),
                state: e.state.to_string(),
                command,
            }
        })
        .collect();

    if rows.is_empty() {
        println!("No listening ports found.");
    } else {
        let table = Table::new(rows);
        println!("{table}");
    }

    Ok(())
}
