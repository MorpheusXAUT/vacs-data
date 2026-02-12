mod cli;

use crate::cli::{Cli, Command, ImportCommand};
use clap::Parser;
use vacs_data_diagnostics::log;

pub fn main() {
    let cli = Cli::parse();
    vacs_data_diagnostics::init(cli.log_format);

    match cli.cmd {
        Command::Validate { input_pos, input } => {
            let Some(input) = input.or(input_pos) else {
                log::error("Missing input path. Either provide INPUT or use --i/--input.");
                std::process::exit(2);
            };

            if vacs_data_validator::validate(&input).is_err() {
                std::process::exit(1);
            }
        }
        Command::Import {
            cmd:
                ImportCommand::Vatglasses {
                    input_pos,
                    output_pos,
                    input,
                    output,
                    overwrite,
                    merge,
                    format,
                },
        } => {
            let Some(input) = input.or(input_pos) else {
                log::error("Missing input path. Either provide INPUT or use --i/--input.");
                std::process::exit(2);
            };
            let Some(output) = output.or(output_pos) else {
                log::error("Missing output path. Either provide OUTPUT or use --o/--output.");
                std::process::exit(2);
            };

            if vacs_data_importer::vatglasses::parse(&input, &output, overwrite, merge, format)
                .is_err()
            {
                std::process::exit(1);
            }
        }
        Command::Import {
            cmd:
                ImportCommand::Euroscope {
                    input_pos,
                    output_pos,
                    input,
                    output,
                    prefixes,
                    overwrite,
                    merge,
                    format,
                },
        } => {
            let Some(input) = input.or(input_pos) else {
                log::error("Missing input path. Either provide INPUT or use --i/--input.");
                std::process::exit(2);
            };
            let Some(output) = output.or(output_pos) else {
                log::error("Missing output path. Either provide OUTPUT or use --o/--output.");
                std::process::exit(2);
            };
            let prefixes = prefixes.unwrap_or_default();

            if vacs_data_importer::euroscope::parse(
                &input, &output, &prefixes, overwrite, merge, format,
            )
                .is_err()
            {
                std::process::exit(1);
            }
        }
    }
}
