use crate::data::collector::collect_port_entries;
use crate::data::types::Protocol;
use crate::output;

pub async fn run(json: bool, protocol: Option<String>, port: Option<u16>) -> anyhow::Result<()> {
    let mut entries = tokio::task::spawn_blocking(collect_port_entries).await??;

    if let Some(proto) = protocol {
        let proto = proto.to_lowercase();
        entries.retain(|e| match proto.as_str() {
            "tcp" => matches!(e.protocol, Protocol::Tcp | Protocol::Tcp6),
            "udp" => matches!(e.protocol, Protocol::Udp | Protocol::Udp6),
            "tcp4" => e.protocol == Protocol::Tcp,
            "tcp6" => e.protocol == Protocol::Tcp6,
            "udp4" => e.protocol == Protocol::Udp,
            "udp6" => e.protocol == Protocol::Udp6,
            _ => true,
        });
    }

    if let Some(p) = port {
        entries.retain(|e| e.port == p);
    }

    // Sort by port by default for CLI output
    entries.sort_by_key(|e| e.port);

    if json {
        output::json::print(&entries)
    } else {
        output::table::print(&entries)
    }
}
