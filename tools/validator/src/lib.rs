use std::path::PathBuf;

use vacs_data_diagnostics::log;
use vacs_vatsim::coverage::CoverageError;
use vacs_vatsim::coverage::network::Network;

pub fn validate(input: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    log::info(format_args!("Validating dataset: {input:?}"));

    if !input.exists() {
        log::error(format_args!("Input {input:?} does not exist"));
        return Err("Input does not exist".into());
    } else if !input.is_dir() {
        log::error(format_args!("Input {input:?} is not a directory"));
        return Err("Input is not a directory".into());
    }

    match Network::load_from_dir(input) {
        Ok(_) => {
            log::info("Dataset validation successful");
        }
        Err(errors) => {
            for err in errors {
                let (context, inner_error) = unwind_error(&err);

                if log::is_human() {
                    use console::style;
                    use vacs_vatsim::coverage::{StructureError, ValidationError};

                    let msg = match inner_error {
                        CoverageError::Validation(ValidationError::MissingReference {
                            field,
                            ref_id,
                        }) => {
                            format!(
                                "referenced {} {} does not exist",
                                field,
                                style(format!("`{ref_id}`")).cyan()
                            )
                        }
                        CoverageError::Structure(StructureError::Duplicate { entity, id }) => {
                            format!("duplicate {} {}", entity, style(format!("`{id}`")).cyan())
                        }
                        _ => inner_error.to_string(),
                    };
                    log::error_with_context(&context, msg);
                } else {
                    log::error_with_context(&context, inner_error);
                }
            }

            return Err("Dataset validation error".into());
        }
    }

    Ok(())
}

fn unwind_error(error: &CoverageError) -> (Vec<String>, &CoverageError) {
    let mut context = Vec::new();
    let mut current_error = error;

    while let CoverageError::Context(ctx) = current_error {
        context.push(ctx.location.clone());
        current_error = &ctx.error;
    }

    (context, current_error)
}
