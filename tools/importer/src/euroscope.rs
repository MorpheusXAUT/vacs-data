use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use vacs_data_diagnostics::log;
use vacs_protocol::vatsim::{PositionId, StationId};
use vacs_vatsim::FacilityType;
use vacs_vatsim::coverage::position::{PositionConfigFile, PositionRaw};

const STUB_FREQUENCY: &str = "199.998";

pub fn parse(
    input: &PathBuf,
    output: &PathBuf,
    ese_path: Option<&PathBuf>,
    profile_paths: &[PathBuf],
    overwrite: bool,
    merge: bool,
    format: crate::OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info(format_args!(
        "Parsing EuroScope sectorfile data from {input:?} to {output:?}"
    ));

    if !input.is_dir() {
        log::error(format_args!(
            "Input {input:?} is not a directory. Expected an unpacked sectorfile directory."
        ));
        return Err("Input must be an unpacked sectorfile directory".into());
    }

    crate::ensure_output_directory(output)?;

    let ext = format.ext();
    let output_positions = crate::check_output_file(
        output,
        &format!("positions.{ext}"),
        "Positions",
        overwrite,
        merge,
    )?;

    let ese_file = resolve_ese_file(input, ese_path)?;
    log::info(format_args!("Using .ese file: {ese_file:?}"));

    let profiles = resolve_profile_files(input, profile_paths)?;
    for profile in &profiles {
        log::info(format_args!("Using profile: {profile:?}"));
    }

    let profile_ids = parse_profiles(&profiles)?;
    log::info(format_args!(
        "Found {} positions in profile files",
        profile_ids.len()
    ));

    let ese_positions = parse_ese_positions(&ese_file)?;
    log::info(format_args!(
        "Found {} positions in .ese file",
        ese_positions.len()
    ));

    let mut positions = build_positions(&profile_ids, &ese_positions);

    if merge && output_positions.exists() {
        log::info(format_args!(
            "Reading existing positions from {output_positions:?}"
        ));
        let content = std::fs::read_to_string(&output_positions)?;
        let mut existing_config: PositionConfigFile = crate::format::deserialize(&content, format)?;
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

/// Resolve the .ese file path. If an explicit path is given, use it.
/// Otherwise, search for a single .ese file in the input directory root.
fn resolve_ese_file(
    input: &Path,
    explicit: Option<&PathBuf>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(path) = explicit {
        crate::check_input_exists(path)?;
        return Ok(path.clone());
    }

    let mut ese_files = Vec::new();
    for entry in std::fs::read_dir(input)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("ese"))
        {
            ese_files.push(path);
        }
    }

    match ese_files.len() {
        0 => {
            log::error(format_args!(
                "No .ese file found in {input:?}. Use --ese to specify the path."
            ));
            Err("No .ese file found in sectorfile directory".into())
        }
        1 => Ok(ese_files.remove(0)),
        n => {
            log::error(format_args!(
                "Found {n} .ese files in {input:?}. Use --ese to specify which one to use."
            ));
            Err("Multiple .ese files found in sectorfile directory".into())
        }
    }
}

/// Resolve profile file paths. If explicit paths are given, use them.
/// Otherwise, recursively search for profile files in the input directory.
fn resolve_profile_files(
    input: &Path,
    explicit: &[PathBuf],
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    if !explicit.is_empty() {
        for path in explicit {
            crate::check_input_exists(path)?;
        }
        return Ok(explicit.to_vec());
    }

    let mut profiles = Vec::new();
    discover_profiles(input, &mut profiles)?;

    if profiles.is_empty() {
        log::error(format_args!(
            "No profile files found in {input:?}. Use --profiles to specify the paths."
        ));
        return Err("No profile files found in sectorfile directory".into());
    }

    Ok(profiles)
}

fn discover_profiles(
    dir: &Path,
    results: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            discover_profiles(&path, results)?;
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("txt"))
            && is_profile_file(&path)
        {
            results.push(path);
        }
    }
    Ok(())
}

/// Check if a file is a EuroScope profile file by looking at the first line.
fn is_profile_file(path: &Path) -> bool {
    let Ok(file) = std::fs::File::open(path) else {
        return false;
    };
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    if reader.read_line(&mut first_line).is_err() {
        return false;
    }
    first_line.trim() == "PROFILE"
}

/// Parse profile files and return the set of position IDs.
fn parse_profiles(paths: &[PathBuf]) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let mut ids = HashSet::new();
    for path in paths {
        let content = std::fs::read_to_string(path)?;
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("PROFILE:")
                && let Some(id) = rest.split(':').next()
            {
                if id.is_empty() || id.to_ascii_uppercase().ends_with("OBS") {
                    continue;
                }
                ids.insert(id.to_string());
            }
        }
    }
    Ok(ids)
}

/// Parse the [POSITIONS] section of an .ese file into a map of ID -> PositionRaw.
fn parse_ese_positions(
    ese_path: &Path,
) -> Result<HashMap<String, PositionRaw>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(ese_path).map_err(|err| {
        log::error(format_args!(
            "Failed to open .ese file {ese_path:?}: {err:?}"
        ));
        err
    })?;

    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(WINDOWS_1252))
        .build(file);
    let reader = BufReader::new(decoder);

    let mut positions = HashMap::new();
    let mut in_positions_section = false;

    for line in reader.lines() {
        let Ok(line) = line else {
            break;
        };
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with(';') {
            continue;
        }
        if trimmed == "[POSITIONS]" {
            in_positions_section = true;
            continue;
        }
        if in_positions_section && trimmed.starts_with('[') && trimmed.ends_with(']') {
            break;
        }
        if !in_positions_section {
            continue;
        }

        if let Some(position) = parse_ese_position_line(trimmed) {
            positions.insert(position.id.to_string(), position);
        }
    }

    Ok(positions)
}

/// Parse a single .ese position line into a PositionRaw.
///
/// Format: `ID:NAME:FREQ:IDENT:SUFFIX:ICAO:FACILITY:...`
fn parse_ese_position_line(line: &str) -> Option<PositionRaw> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() < 7 {
        return None;
    }

    let facility_type: FacilityType = parts[6].parse().ok()?;
    if facility_type == FacilityType::Unknown {
        return None;
    }

    Some(PositionRaw {
        id: PositionId::from(parts[0]),
        frequency: parts[2].to_string(),
        prefixes: HashSet::from([parts[5].to_string()]),
        facility_type,
        profile_id: None,
        default_call_sources: vec![StationId::from(parts[0])],
    })
}

/// Build output positions by matching profile IDs against .ese position data.
/// Positions found in .ese get full data; missing ones get a stub with fallback frequency.
fn build_positions(
    profile_ids: &HashSet<String>,
    ese_positions: &HashMap<String, PositionRaw>,
) -> Vec<PositionRaw> {
    let mut positions = Vec::with_capacity(profile_ids.len());

    for id in profile_ids {
        if let Some(position) = ese_positions.get(id) {
            positions.push(position.clone());
        } else {
            log::warn(format_args!(
                "Position {id} from profile not found in .ese file, creating stub"
            ));
            positions.push(create_stub_position(id));
        }
    }

    positions
}

/// Create a stub position for a profile entry not found in the .ese file.
/// Derives prefix from the first segment of the ID and facility type from the last segment.
fn create_stub_position(id: &str) -> PositionRaw {
    let prefix = id.split('_').next().unwrap_or(id).to_string();
    let facility_type = id.parse().unwrap_or_default();

    PositionRaw {
        id: PositionId::from(id),
        frequency: STUB_FREQUENCY.to_string(),
        prefixes: HashSet::from([prefix]),
        facility_type,
        profile_id: None,
        default_call_sources: vec![StationId::from(id)],
    }
}
