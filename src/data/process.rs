use std::collections::HashMap;

use crate::data::types::{ChildProcess, ProcessDetail, ProcessInfo};

/// Build a map of socket inode -> ProcessInfo by scanning /proc/{pid}/fd/*.
pub fn build_inode_to_process_map() -> HashMap<u64, ProcessInfo> {
    let mut map = HashMap::new();

    let Ok(procs) = procfs::process::all_processes() else {
        return map;
    };

    for proc_result in procs {
        let Ok(process) = proc_result else {
            continue;
        };
        let Ok(fds) = process.fd() else {
            continue;
        };
        let Ok(stat) = process.stat() else {
            continue;
        };

        let cmdline = process
            .cmdline()
            .ok()
            .map(|args| args.join(" "))
            .unwrap_or_else(|| stat.comm.clone());

        let cwd = process.cwd().ok();

        let info = ProcessInfo {
            pid: stat.pid,
            name: stat.comm.clone(),
            cmdline,
            cwd,
        };

        for fd_result in fds {
            let Ok(fd_info) = fd_result else {
                continue;
            };
            if let procfs::process::FDTarget::Socket(inode) = fd_info.target {
                map.insert(inode, info.clone());
            }
        }
    }

    map
}

/// Load detailed process info for the detail pane. Called on-demand.
pub fn load_process_detail(pid: i32) -> Option<ProcessDetail> {
    let process = procfs::process::Process::new(pid).ok()?;
    let stat = process.stat().ok()?;

    let cmdline = process.cmdline().unwrap_or_default();
    let name = stat.comm.clone();

    let cwd = process.cwd().ok();

    // Environment variables (may fail due to permissions)
    let environ: Vec<(String, String)> = process
        .environ()
        .ok()
        .map(|env| {
            env.into_iter()
                .map(|(k, v)| {
                    (
                        k.to_string_lossy().into_owned(),
                        v.to_string_lossy().into_owned(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();

    // Count open FDs
    let open_fds = process.fd_count().unwrap_or(0);

    // Child processes
    let children = load_children(pid);

    // Container detection via cgroup
    let container_id = detect_container(&process);

    // Memory RSS from statm
    let memory_rss_bytes = process.statm().ok().map(|s| s.resident * page_size());

    let threads = Some(stat.num_threads as u32);

    Some(ProcessDetail {
        pid,
        name,
        cmdline,
        cwd,
        environ,
        open_fds,
        children,
        container_id,
        memory_rss_bytes,
        threads,
    })
}

fn load_children(pid: i32) -> Vec<ChildProcess> {
    let mut children = Vec::new();
    let Ok(procs) = procfs::process::all_processes() else {
        return children;
    };

    for proc_result in procs {
        let Ok(process) = proc_result else {
            continue;
        };
        let Ok(stat) = process.stat() else {
            continue;
        };
        if stat.ppid == pid {
            children.push(ChildProcess {
                pid: stat.pid,
                name: stat.comm,
            });
        }
    }

    children
}

fn detect_container(process: &procfs::process::Process) -> Option<String> {
    let cgroups = process.cgroups().ok()?;
    for cgroup in cgroups {
        let path = cgroup.pathname;
        // Docker: /docker/<hash> or /system.slice/docker-<hash>.scope
        if let Some(hash) = extract_container_hash(&path, "docker-") {
            return Some(hash);
        }
        if let Some(hash) = extract_container_hash(&path, "docker/") {
            return Some(hash);
        }
        // Podman: /libpod-<hash>.scope
        if let Some(hash) = extract_container_hash(&path, "libpod-") {
            return Some(hash);
        }
    }
    None
}

fn extract_container_hash(path: &str, prefix: &str) -> Option<String> {
    let idx = path.find(prefix)?;
    let after = &path[idx + prefix.len()..];
    // Take up to the next '.' or '/' or end
    let hash: String = after
        .chars()
        .take_while(|c| c.is_ascii_hexdigit())
        .collect();
    if hash.len() >= 12 {
        Some(hash[..12].to_string())
    } else {
        None
    }
}

fn page_size() -> u64 {
    // Safe: sysconf is always available on Linux
    unsafe { nix::libc::sysconf(nix::libc::_SC_PAGESIZE) as u64 }
}

/// Resolve a UID to a username.
pub fn resolve_username(uid: u32) -> String {
    uzers::get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| uid.to_string())
}
