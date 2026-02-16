use clap::{Parser, Subcommand};
use std::path::PathBuf;
use vacs_data_diagnostics::LogFormat;

#[derive(Debug, Parser)]
#[command(name = "vacs-data", version, about = "vacs dataset tools")]
pub struct Cli {
    /// Increase verbosity, can be specified multiple times (-v, -vv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Quiet mode (errors only)
    #[arg(short, long)]
    pub quiet: bool,

    /// Logging output format. Supported: human, github
    #[arg(long, default_value_t = LogFormat::Human)]
    pub log_format: LogFormat,

    /// Disable interactive prompts
    #[arg(long)]
    pub non_interactive: bool,

    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run validations on the whole dataset
    #[command(arg_required_else_help = true)]
    Validate {
        /// Dataset root to validate (positional).
        #[arg(value_name = "INPUT", required_unless_present = "input")]
        input_pos: Option<PathBuf>,

        /// Dataset root to validate
        #[arg(short, long)]
        input: Option<PathBuf>,
    },

    /// Import data from external sources, converting them to vacs dataset format
    Import {
        #[command(subcommand)]
        cmd: ImportCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum ImportCommand {
    /// Import data from the VATglasses project, converting it to vacs dataset format
    #[command(arg_required_else_help = true)]
    Vatglasses {
        /// Input JSON file (positional)
        #[arg(value_name = "INPUT", required_unless_present = "input")]
        input_pos: Option<PathBuf>,

        /// Output directory (positional)
        #[arg(value_name = "OUTPUT", required_unless_present = "output")]
        output_pos: Option<PathBuf>,

        /// Input JSON file
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Format to use for output files. Supported: toml, json
        #[arg(short, long, default_value_t = vacs_data_importer::OutputFormat::Toml)]
        format: vacs_data_importer::OutputFormat,

        /// Overwrite existing files
        #[arg(long, conflicts_with = "merge")]
        overwrite: bool,

        /// Merge with existing files
        #[arg(long, conflicts_with = "overwrite")]
        merge: bool,
    },

    /// Import data from an EuroScope sectorfile, converting it to vacs dataset format
    #[command(arg_required_else_help = true)]
    Euroscope {
        /// Input JSON file (positional)
        #[arg(value_name = "INPUT", required_unless_present = "input")]
        input_pos: Option<PathBuf>,

        /// Output directory (positional)
        #[arg(value_name = "OUTPUT", required_unless_present = "output")]
        output_pos: Option<PathBuf>,

        /// Input JSON file
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Format to use for output files. Supported: toml, json
        #[arg(short, long, default_value_t = vacs_data_importer::OutputFormat::Toml)]
        format: vacs_data_importer::OutputFormat,

        /// Prefixes to filter positions by
        #[arg(short, long, value_name = "PREFIX")]
        prefixes: Option<Vec<String>>,

        /// Overwrite existing files
        #[arg(long, conflicts_with = "merge")]
        overwrite: bool,

        /// Merge with existing files
        #[arg(long, conflicts_with = "overwrite")]
        merge: bool,
    },
}
