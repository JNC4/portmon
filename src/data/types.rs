use serde::Serialize;
use std::cmp::Ordering;
use std::net::IpAddr;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Protocol {
    Tcp,
    Tcp6,
    Udp,
    Udp6,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Tcp => write!(f, "TCP"),
            Protocol::Tcp6 => write!(f, "TCP6"),
            Protocol::Udp => write!(f, "UDP"),
            Protocol::Udp6 => write!(f, "UDP6"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SocketState {
    Listen,
    Established,
    SynSent,
    SynRecv,
    FinWait1,
    FinWait2,
    TimeWait,
    Close,
    CloseWait,
    LastAck,
    Closing,
    /// UDP bound socket
    Unconnected,
}

impl std::fmt::Display for SocketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketState::Listen => write!(f, "LISTEN"),
            SocketState::Established => write!(f, "ESTAB"),
            SocketState::SynSent => write!(f, "SYN-SENT"),
            SocketState::SynRecv => write!(f, "SYN-RECV"),
            SocketState::FinWait1 => write!(f, "FIN-WAIT1"),
            SocketState::FinWait2 => write!(f, "FIN-WAIT2"),
            SocketState::TimeWait => write!(f, "TIME-WAIT"),
            SocketState::Close => write!(f, "CLOSE"),
            SocketState::CloseWait => write!(f, "CLOSE-WAIT"),
            SocketState::LastAck => write!(f, "LAST-ACK"),
            SocketState::Closing => write!(f, "CLOSING"),
            SocketState::Unconnected => write!(f, "UNCONN"),
        }
    }
}

impl SocketState {
    pub fn short(&self) -> &'static str {
        match self {
            SocketState::Listen => "L",
            SocketState::Established => "E",
            SocketState::SynSent => "SS",
            SocketState::SynRecv => "SR",
            SocketState::FinWait1 => "F1",
            SocketState::FinWait2 => "F2",
            SocketState::TimeWait => "TW",
            SocketState::Close => "CL",
            SocketState::CloseWait => "CW",
            SocketState::LastAck => "LA",
            SocketState::Closing => "CG",
            SocketState::Unconnected => "UC",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cmdline: String,
    pub cwd: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PortEntry {
    pub protocol: Protocol,
    pub local_addr: IpAddr,
    pub port: u16,
    pub remote_port: u16,
    pub state: SocketState,
    pub inode: u64,
    pub uid: u32,
    pub username: String,
    pub process: Option<ProcessInfo>,
}

impl PortEntry {
    pub fn process_name(&self) -> &str {
        self.process
            .as_ref()
            .map(|p| p.name.as_str())
            .unwrap_or("-")
    }

    pub fn pid(&self) -> Option<i32> {
        self.process.as_ref().map(|p| p.pid)
    }

    pub fn pid_str(&self) -> String {
        self.process
            .as_ref()
            .map(|p| p.pid.to_string())
            .unwrap_or_else(|| "-".to_string())
    }

    pub fn cmdline(&self) -> &str {
        self.process
            .as_ref()
            .map(|p| p.cmdline.as_str())
            .unwrap_or("-")
    }

    /// Extract project name from the process CWD (last directory component).
    pub fn project(&self) -> &str {
        self.process
            .as_ref()
            .and_then(|p| p.cwd.as_ref())
            .and_then(|cwd| cwd.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("-")
    }

    /// Concatenated search string for fuzzy matching across all fields.
    pub fn search_string(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {}",
            self.protocol,
            self.local_addr,
            self.port,
            self.pid_str(),
            self.process_name(),
            self.username,
            self.project(),
            self.cmdline(),
        )
    }

    pub fn cmp_by_column(&self, other: &Self, col: SortColumn) -> Ordering {
        match col {
            SortColumn::Protocol => self.protocol.cmp(&other.protocol),
            SortColumn::LocalAddr => self
                .local_addr
                .to_string()
                .cmp(&other.local_addr.to_string()),
            SortColumn::Port => self.port.cmp(&other.port),
            SortColumn::Pid => {
                let a = self.pid().unwrap_or(0);
                let b = other.pid().unwrap_or(0);
                a.cmp(&b)
            }
            SortColumn::ProcessName => self
                .process_name()
                .to_lowercase()
                .cmp(&other.process_name().to_lowercase()),
            SortColumn::User => self.username.cmp(&other.username),
            SortColumn::Project => self
                .project()
                .to_lowercase()
                .cmp(&other.project().to_lowercase()),
            SortColumn::State => self.state.short().cmp(other.state.short()),
            SortColumn::Command => self.cmdline().cmp(other.cmdline()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessDetail {
    pub pid: i32,
    pub name: String,
    pub cmdline: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub environ: Vec<(String, String)>,
    pub open_fds: usize,
    pub children: Vec<ChildProcess>,
    pub container_id: Option<String>,
    pub memory_rss_bytes: Option<u64>,
    pub threads: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChildProcess {
    pub pid: i32,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Protocol,
    LocalAddr,
    Port,
    Pid,
    ProcessName,
    User,
    Project,
    State,
    Command,
}

impl SortColumn {
    pub fn label(&self) -> &'static str {
        match self {
            SortColumn::Protocol => "Proto",
            SortColumn::LocalAddr => "Local Addr",
            SortColumn::Port => "Port",
            SortColumn::Pid => "PID",
            SortColumn::ProcessName => "Name",
            SortColumn::User => "User",
            SortColumn::Project => "Project",
            SortColumn::State => "St",
            SortColumn::Command => "Command",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn toggle(&self) -> Self {
        match self {
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Ascending,
        }
    }

    pub fn arrow(&self) -> &'static str {
        match self {
            SortOrder::Ascending => "▲",
            SortOrder::Descending => "▼",
        }
    }
}
