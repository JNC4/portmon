use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortMonitorError {
    #[error("no process found listening on port {0}")]
    PortNotFound(u16),

    #[error("permission denied: cannot signal PID {0}")]
    PermissionDenied(i32),

    #[error("process {0} no longer exists")]
    ProcessGone(i32),

    #[error("clipboard unavailable: {0}")]
    Clipboard(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Procfs(#[from] procfs::ProcError),
}
