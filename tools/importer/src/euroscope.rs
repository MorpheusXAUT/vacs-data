use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use vacs_data_diagnostics::log;
use vacs_protocol::vatsim::PositionId;
use vacs_vatsim::FacilityType;
use vacs_vatsim::coverage::position;
use vacs_vatsim::coverage::position::{PositionConfigFile, PositionRaw};

pub fn parse(
    input: &PathBuf,
    output: &PathBuf,
    prefixes: &[String],
    overwrite: bool,
    merge: bool,
    format: crate::OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info(format_args!(
        "Parsing EuroScope sectorfile data from {input:?} to {output:?}"
    ));

    crate::check_input_exists(input)?;
    crate::ensure_output_directory(output)?;

    let ext = format.ext();
    let output_positions = crate::check_output_file(
        output,
        &format!("positions.{ext}"),
        "Positions",
        overwrite,
        merge,
    )?;

    let file = match std::fs::File::open(input) {
        Ok(f) => f,
        Err(err) => {
            log::error(format_args!("Failed to open input file {input:?}: {err:?}"));
            return Err(err.into());
        }
    };

    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let reader = BufReader::new(decoder);

    let mut positions = Vec::new();
    let mut in_positions_section = false;

    for line in reader.lines() {
        let Ok(line) = line else {
            break;
        };
        let trimmed = line.trim();

        // Empty line or comment
        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }
        // Start of positions section
        if trimmed == "[POSITIONS]" {
            in_positions_section = true;
            continue;
        }
        // Start of next section after leaving positions section
        if in_positions_section && trimmed.starts_with('[') && trimmed.ends_with(']') {
            break;
        }
        // Ignore positions outside specified prefixes
        if !prefixes.is_empty() && prefixes.iter().all(|p| !trimmed.starts_with(p)) {
            continue;
        }

        if let Ok(position) = PositionRaw::from_ese_line(trimmed) {
            if position.facility_type == FacilityType::Unknown {
                continue;
            }
            positions.push(position);
        }
    }

    if merge && output_positions.exists() {
        log::info(format_args!(
            "Reading existing positions from {output_positions:?}"
        ));
        let content = std::fs::read_to_string(&output_positions)?;
        let mut existing_config: PositionConfigFile = toml::from_str(&content)?;
        let existing_ids: HashSet<_> = existing_config
            .positions
            .iter()
            .map(|p| p.id.clone())
            .collect();

        let mut added_count = 0;
        for position in positions {
            if !existing_ids.contains(&position.id) {
                existing_config.positions.push(position);
                added_count += 1;
            }
        }
        positions = existing_config.positions;
        log::info(format_args!("Merged {added_count} new positions"));
    }

    positions.sort_by(|a, b| {
        a.facility_type
            .cmp(&b.facility_type)
            .reverse()
            .then_with(|| a.id.cmp(&b.id))
    });

    let serialized_positions =
        match crate::format::serialize(&PositionConfigFile { positions }, format) {
            Ok(s) => s,
            Err(err) => {
                log::error(format_args!("Failed to serialize positions: {err:?}"));
                return Err(err);
            }
        };

    crate::write_output_file(&output_positions, &serialized_positions, "Positions")?;

    log::info(format_args!("Wrote output files to {output:?}"));
    Ok(())
}

trait ParsePosition: Sized {
    type Error;
    fn from_ese_line(line: &str) -> Result<Self, Self::Error>;
}

impl ParsePosition for position::PositionRaw {
    type Error = String;
    fn from_ese_line(line: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 7 {
            return Err("Invalid format".to_string());
        }

        let Ok(facility_type) = parts[6].parse() else {
            return Err("Invalid facility type".to_string());
        };

        Ok(Self {
            id: PositionId::from(parts[0]),
            frequency: parts[2].to_string(),
            prefixes: HashSet::from([parts[5].to_string()]),
            facility_type,
            profile_id: None,
        })
    }
}
