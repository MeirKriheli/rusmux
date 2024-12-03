//! CLI arguments parser.
use clap::command;

#[derive(Debug, Parser)]
#[command(version, about, arg_required_else_help = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the project's commands
    #[command(alias = "start", arg_required_else_help = true)]
    Run {
        /// Project name or filesystem path
        project: String,
    },
    /// Stop the project's session
    #[command(alias = "kill", arg_required_else_help = true)]
    Stop {
        /// Project name or filesystem path
        project: String,
    },
    /// Output shell commands for a project
    #[command(arg_required_else_help = true)]
    Debug {
        /// Project name or filesystem path
        project: String,
    },
    /// Edit an existing project
    #[command(arg_required_else_help = true)]
    Edit {
        /// Project name or filesystem path
        project: String,
    },
    /// Delete an existing project
    #[command(arg_required_else_help = true)]
    Delete {
        /// Project name or filesystem path
        project: String,
    },
    /// Create a new project
    #[command(arg_required_else_help = true)]
    New {
        /// Project name or filesystem path
        project: String,
        /// Don't use a template for the file
        #[arg(long)]
        blank: bool,
    },
    /// List all projects in the config directory
    List,
    /// Copy an existing project to a new one and edit it
    Copy {
        /// Existing Project name or filesystem path
        #[arg(required = true)]
        existing: String,
        /// New Project name or filesystem path
        #[arg(required = true)]
        new: String,
    },
    /// Check your environment's configuration
    Doctor,
}
