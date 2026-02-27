use crate::data::collector::collect_port_entries;
use crate::data::process::load_process_detail;
use crate::error::PortMonitorError;

pub async fn run(port: u16, json: bool) -> anyhow::Result<()> {
    let entries = tokio::task::spawn_blocking(collect_port_entries).await??;

    let entry = entries
        .iter()
        .find(|e| e.port == port)
        .ok_or(PortMonitorError::PortNotFound(port))?;

    let pid = entry
        .pid()
        .ok_or_else(|| anyhow::anyhow!("process info unavailable for port {} (try running as root)", port))?;

    let detail = load_process_detail(pid)
        .ok_or_else(|| anyhow::anyhow!("could not read process details for PID {}", pid))?;

    if json {
        let json = serde_json::to_string_pretty(&detail)?;
        println!("{json}");
    } else {
        println!("Port:       {}", port);
        println!("Protocol:   {}", entry.protocol);
        println!("Address:    {}", entry.local_addr);
        println!("State:      {}", entry.state);
        println!("PID:        {}", detail.pid);
        println!("Process:    {}", detail.name);
        println!("Command:    {}", detail.cmdline.join(" "));
        println!("User:       {} (uid {})", entry.username, entry.uid);
        if let Some(ref cwd) = detail.cwd {
            println!("CWD:        {}", cwd.display());
        }
        println!("Open FDs:   {}", detail.open_fds);
        if let Some(rss) = detail.memory_rss_bytes {
            println!("RSS:        {} MB", rss / (1024 * 1024));
        }
        if let Some(threads) = detail.threads {
            println!("Threads:    {}", threads);
        }
        if let Some(ref cid) = detail.container_id {
            println!("Container:  {}", cid);
        }
        if !detail.children.is_empty() {
            println!("Children:   {}", detail.children.len());
            for child in &detail.children {
                println!("  PID {} ({})", child.pid, child.name);
            }
        }
        if !detail.environ.is_empty() {
            println!("Environment ({} vars):", detail.environ.len());
            for (k, v) in detail.environ.iter().take(20) {
                let display_v = if v.len() > 80 {
                    format!("{}...", &v[..77])
                } else {
                    v.clone()
                };
                println!("  {}={}", k, display_v);
            }
            if detail.environ.len() > 20 {
                println!("  ... and {} more", detail.environ.len() - 20);
            }
        }
    }

    Ok(())
}
