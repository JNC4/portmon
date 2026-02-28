use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "portmon",
    about = "Monitor listening ports with rich process info",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Refresh interval in seconds (TUI mode)
    #[arg(short, long, default_value = "2.0")]
    pub interval: f64,
}

#[derive(Subcommand)]
pub enum Command {
    /// List all listening ports
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,

        /// Filter by protocol (tcp, udp)
        #[arg(short, long)]
        protocol: Option<String>,

        /// Show only this port number
        #[arg(short = 'P', long)]
        port: Option<u16>,
    },

    /// Kill the process listening on a port
    Kill {
        /// Port number
        port: u16,

        /// Use SIGKILL instead of SIGTERM
        #[arg(long)]
        force: bool,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Show detailed info about what's on a port
    Info {
        /// Port number
        port: u16,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Watch a port for bind/unbind events
    Watch {
        /// Port number
        port: u16,

        /// Poll interval in seconds
        #[arg(short, long, default_value = "1.0")]
        interval: f64,
    },
}
