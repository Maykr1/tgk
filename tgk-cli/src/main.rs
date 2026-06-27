//! TGK CLI & Orchestrator
//!
//! This module serves as the entry point for the The Greatest Knight. It
//! handles CLI argument parsing using [`clap`].

use clap::{Parser, Subcommand};

/// Top-level CLI argument structure
///
/// This struct represents the CLI command tree. It is parsed using
/// [`clap::Parser`] and contains the selected subcommands.
#[derive(Parser)]
#[command(name = "tgk", version, about = "The Greatest Knight")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available CLI subcommands.
///
/// Each variant corresponds to a distinct execution path within The Greatest Knight.
#[derive(Subcommand)]
enum Commands {
    /// Initialize an ecrypted identity profile
    Init,

    /// Scan public sources for your exposed data
    Scan {
        /// Only go over findings at or above this severity
        #[arg(long, default_value = "low")]
        severity: String,
    },

    /// List current findings
    Findings {
        /// Filter by severity
        #[arg(long)]
        severity: Option<String>,
    },

    /// Generate and send removal requests
    Remediate {
        /// Target a specific source (e.g. whitepages, spokeo)
        #[arg(long)]
        source: Option<String>,

        /// Send removal requests for all findings
        #[arg(long)]
        all: bool,
    },

    /// Check status of sent removal requests
    Status,

    /// Schedule periodic audits
    Schedule {
        /// Audit interval (e.g. 7d, 24h)
        #[arg(long, default_value = "7d")]
        interval: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Initializing profile...");
        }
        Commands::Scan { severity } => {
            println!("Scanning with severity: {severity}");
        }
        Commands::Findings { severity } => {
            println!("Listing findings (filter: {severity:?})");
        }
        Commands::Remediate { source, all } => {
            println!("Remediating — source: {source:?}, all: {all}");
        }
        Commands::Status => {
            println!("Checking removal request status...");
        }
        Commands::Schedule { interval } => {
            println!("Scheduling audits every {interval}");
        }
    }
}
