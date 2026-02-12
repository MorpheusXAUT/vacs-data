pub mod euroscope;
pub mod format;
pub mod vatglasses;

pub use format::OutputFormat;

use std::path::{Path, PathBuf};
use vacs_data_diagnostics::log;

pub fn check_input_exists(input: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !input.exists() {
        log::error(format_args!("Input file {input:?} does not exist"));
        return Err("Input file does not exist".into());
    }
    Ok(())
}

pub fn ensure_output_directory(output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if output.exists() {
        if !output.is_dir() {
            log::error(format_args!("Output {output:?} is not a directory"));
            return Err("Output is not a directory".into());
        }
    } else if let Err(err) = std::fs::create_dir_all(output) {
        log::error(format_args!(
            "Failed to create output directory {output:?}: {err:?}",
        ));
        return Err(err.into());
    }
    Ok(())
}

pub fn check_output_file(
    output_dir: &Path,
    filename: &str,
    label: &str,
    overwrite: bool,
    merge: bool,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_path = output_dir.join(filename);
    if output_path.exists() {
        if overwrite {
            log::warn(format_args!(
                "Overwriting existing {label} output file {output_path:?}"
            ));
        } else if merge {
            log::info(format_args!(
                "Merging with existing {label} output file {output_path:?}"
            ));
        } else {
            log::error(format_args!(
                "{label} output file {output_path:?} already exists. Either --merge or --overwrite must be specified."
            ));
            return Err(format!("{label} output file already exists").into());
        }
    }
    Ok(output_path)
}

pub fn write_output_file(
    path: &Path,
    content: &str,
    label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Err(err) = std::fs::write(path, content) {
        log::error(format_args!(
            "Failed to write {label} output file {path:?}: {err:?}"
        ));
        return Err(err.into());
    }
    Ok(())
}
