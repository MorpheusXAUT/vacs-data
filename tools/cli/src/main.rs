mod cli;

use crate::cli::{Cli, Command, ImportCommand};
use clap::Parser;

pub fn main() {
    let cli = Cli::parse();
    vacs_data_diagnostics::init(cli.log_format);

    match cli.cmd {
        Command::Validate { input_pos, input } => {
            let input = input.or(input_pos).unwrap();

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
            let input = input.or(input_pos).unwrap();
            let output = output.or(output_pos).unwrap();

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
            let input = input.or(input_pos).unwrap();
            let output = output.or(output_pos).unwrap();
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
