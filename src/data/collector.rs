use crate::data::process::{build_inode_to_process_map, resolve_username};
use crate::data::scanner::scan_listening_sockets;
use crate::data::types::PortEntry;

/// Collect all listening port entries with associated process info.
/// This is the main entry point used by both TUI and CLI.
pub fn collect_port_entries() -> anyhow::Result<Vec<PortEntry>> {
    let sockets = scan_listening_sockets()?;
    let inode_map = build_inode_to_process_map();

    let entries: Vec<PortEntry> = sockets
        .into_iter()
        .map(|raw| {
            let process = inode_map.get(&raw.inode).cloned();
            let username = resolve_username(raw.uid);

            PortEntry {
                protocol: raw.protocol,
                local_addr: raw.local_addr,
                port: raw.port,
                remote_port: raw.remote_port,
                state: raw.state,
                inode: raw.inode,
                uid: raw.uid,
                username,
                process,
            }
        })
        .collect();

    Ok(entries)
}
