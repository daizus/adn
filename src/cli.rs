use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "adn", version, about = "Network orchestrator MVP")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Preview {
        #[arg(value_name = "CONFIG")]
        config_path: Option<std::path::PathBuf>,
    },
    Apply {
        #[arg(value_name = "CONFIG")]
        config_path: Option<std::path::PathBuf>,
    },
    List {
        #[arg(value_name = "CONFIG")]
        config_path: Option<std::path::PathBuf>,
    },
    Status {
        #[arg(value_name = "CONFIG")]
        config_path: Option<std::path::PathBuf>,
    },
    Reset {
        #[arg(value_name = "CONFIG")]
        config_path: Option<std::path::PathBuf>,
    },
}
