use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::configure_project;

#[derive(Parser)]
#[command(name = "hulma", version, about = "Project-aware Claude Code scaffolder")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate review agents, slash commands, and hook templates for a project's .claude/ directory.
    ///
    /// By default, hulma launches Claude Code to inspect the project and write
    /// project-specific templates. Pass --static to install generic templates
    /// without invoking Claude.
    Configure {
        /// Project directory to configure (defaults to current directory)
        #[arg(default_value = ".")]
        dir: PathBuf,

        /// Install generic templates without invoking Claude Code
        #[arg(long)]
        r#static: bool,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Configure { dir, r#static } => configure_project::run_configure(&dir, r#static),
    }
}
