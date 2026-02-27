use std::net::IpAddr;

use crate::data::types::{Protocol, SocketState};

/// Raw socket data read directly from /proc/net/*
pub struct RawSocket {
    pub protocol: Protocol,
    pub local_addr: IpAddr,
    pub port: u16,
    pub remote_port: u16,
    pub state: SocketState,
    pub inode: u64,
    pub uid: u32,
}

fn tcp_state_to_socket_state(state: procfs::net::TcpState) -> SocketState {
    match state {
        procfs::net::TcpState::Established => SocketState::Established,
        procfs::net::TcpState::SynSent => SocketState::SynSent,
        procfs::net::TcpState::SynRecv => SocketState::SynRecv,
        procfs::net::TcpState::FinWait1 => SocketState::FinWait1,
        procfs::net::TcpState::FinWait2 => SocketState::FinWait2,
        procfs::net::TcpState::TimeWait => SocketState::TimeWait,
        procfs::net::TcpState::Close => SocketState::Close,
        procfs::net::TcpState::CloseWait => SocketState::CloseWait,
        procfs::net::TcpState::LastAck => SocketState::LastAck,
        procfs::net::TcpState::Listen => SocketState::Listen,
        procfs::net::TcpState::Closing => SocketState::Closing,
        procfs::net::TcpState::NewSynRecv => SocketState::SynRecv,
    }
}

/// Scan all listening sockets from /proc/net/{tcp,tcp6,udp,udp6}.
pub fn scan_listening_sockets() -> anyhow::Result<Vec<RawSocket>> {
    let mut sockets = Vec::new();

    // TCP
    if let Ok(entries) = procfs::net::tcp() {
        for entry in entries {
            if entry.state == procfs::net::TcpState::Listen {
                sockets.push(RawSocket {
                    protocol: Protocol::Tcp,
                    local_addr: entry.local_address.ip(),
                    port: entry.local_address.port(),
                    remote_port: entry.remote_address.port(),
                    state: tcp_state_to_socket_state(entry.state),
                    inode: entry.inode,
                    uid: entry.uid,
                });
            }
        }
    }

    // TCP6
    if let Ok(entries) = procfs::net::tcp6() {
        for entry in entries {
            if entry.state == procfs::net::TcpState::Listen {
                sockets.push(RawSocket {
                    protocol: Protocol::Tcp6,
                    local_addr: entry.local_address.ip(),
                    port: entry.local_address.port(),
                    remote_port: entry.remote_address.port(),
                    state: tcp_state_to_socket_state(entry.state),
                    inode: entry.inode,
                    uid: entry.uid,
                });
            }
        }
    }

    // UDP — all bound sockets are "listening"
    if let Ok(entries) = procfs::net::udp() {
        for entry in entries {
            sockets.push(RawSocket {
                protocol: Protocol::Udp,
                local_addr: entry.local_address.ip(),
                port: entry.local_address.port(),
                remote_port: entry.remote_address.port(),
                state: SocketState::Unconnected,
                inode: entry.inode,
                uid: entry.uid,
            });
        }
    }

    // UDP6
    if let Ok(entries) = procfs::net::udp6() {
        for entry in entries {
            sockets.push(RawSocket {
                protocol: Protocol::Udp6,
                local_addr: entry.local_address.ip(),
                port: entry.local_address.port(),
                remote_port: entry.remote_address.port(),
                state: SocketState::Unconnected,
                inode: entry.inode,
                uid: entry.uid,
            });
        }
    }

    Ok(sockets)
}
