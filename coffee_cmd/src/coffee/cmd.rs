//! Coffee command line arguments definition.
use clap::{Parser, Subcommand};

/// Coffee main command line definition for the command line tools.
#[derive(Debug, Parser)]
#[clap(name = "coffee")]
#[clap(about = "A plugin manager for core lightning", long_about = None)]
pub struct CoffeeArgs {
    #[clap(subcommand)]
    pub command: CoffeeCommand,
    #[clap(short, long, value_parser)]
    pub conf: Option<String>,
    #[clap(short, long, value_parser)]
    pub network: Option<String>,
    #[clap(short, long, value_parser, name = "data-dir")]
    pub data_dir: Option<String>,
}

/// Coffee subcommand of the command line daemon.
#[derive(Debug, Subcommand)]
pub enum CoffeeCommand {
    /// Install a single by name.
    #[clap(arg_required_else_help = true)]
    Install {
        plugin: String,
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        dynamic: bool,
    },
    /// upgrade a single or a list of plugins.
    #[clap(arg_required_else_help = true)]
    Upgrade,
    /// Print the list of plugins installed in cln.
    #[clap(arg_required_else_help = false)]
    List {
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        remotes: bool,
    },
    /// Remove a plugin installed in cln.
    #[clap(arg_required_else_help = true)]
    Remove,
    /// Manage Repository subcommand
    #[clap(arg_required_else_help = true)]
    Remote {
        #[clap(subcommand)]
        action: RemoteAction,
    },
    /// Configur coffee with the core lightning
    /// configuration
    #[clap(arg_required_else_help = true)]
    Setup { cln_conf: String },
}

#[derive(Debug, Subcommand)]
pub enum RemoteAction {
    Add { name: String, url: String },
    Remove { name: String },
}
