mod cli;
mod commands;
mod data;
mod error;
mod output;
mod tui;

use std::time::Duration;

use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        None => {
            // TUI mode
            let interval = Duration::from_secs_f64(cli.interval);
            tui::run_tui(interval).await
        }
        Some(cli::Command::List {
            json,
            protocol,
            port,
        }) => commands::list::run(json, protocol, port).await,
        Some(cli::Command::Kill { port, force, yes }) => {
            commands::kill::run(port, force, yes).await
        }
        Some(cli::Command::Info { port, json }) => commands::info::run(port, json).await,
        Some(cli::Command::Watch { port, interval }) => {
            commands::watch::run(port, Duration::from_secs_f64(interval)).await
        }
    }
}
