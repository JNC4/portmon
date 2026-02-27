use std::io::{self, Write};

use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;

use crate::data::collector::collect_port_entries;
use crate::error::PortMonitorError;

pub async fn run(port: u16, force: bool, yes: bool) -> anyhow::Result<()> {
    let entries = tokio::task::spawn_blocking(collect_port_entries).await??;

    let entry = entries
        .iter()
        .find(|e| e.port == port)
        .ok_or(PortMonitorError::PortNotFound(port))?;

    let process = entry
        .process
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("process info unavailable for port {} (try running as root)", port))?;

    let sig = if force { Signal::SIGKILL } else { Signal::SIGTERM };
    let sig_name = if force { "SIGKILL" } else { "SIGTERM" };

    if !yes {
        print!(
            "Kill {} (PID {}) on port {} with {}? [y/N] ",
            process.name, process.pid, port, sig_name
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    match signal::kill(Pid::from_raw(process.pid), sig) {
        Ok(()) => {
            println!(
                "Sent {} to {} (PID {}) on port {}",
                sig_name, process.name, process.pid, port
            );
        }
        Err(nix::errno::Errno::EPERM) => {
            return Err(PortMonitorError::PermissionDenied(process.pid).into());
        }
        Err(nix::errno::Errno::ESRCH) => {
            return Err(PortMonitorError::ProcessGone(process.pid).into());
        }
        Err(e) => {
            return Err(anyhow::anyhow!("kill failed: {}", e));
        }
    }

    Ok(())
}
