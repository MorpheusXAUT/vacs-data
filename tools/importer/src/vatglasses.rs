use console::style;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use vacs_data_diagnostics::log;
use vacs_protocol::vatsim::{PositionId, StationId};
use vacs_vatsim::coverage::position::PositionConfigFile;
use vacs_vatsim::coverage::station::{StationConfigFile, StationRaw};
use vacs_vatsim::{FacilityType, coverage};

pub fn parse(
    input: &PathBuf,
    output: &PathBuf,
    overwrite: bool,
    merge: bool,
    format: crate::OutputFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info(format_args!(
        "Parsing VATglasses data from {input:?} to {output:?}"
    ));

    crate::check_input_exists(input)?;
    crate::ensure_output_directory(output)?;

    let ext = format.ext();
    let output_stations = crate::check_output_file(
        output,
        &format!("stations.{ext}"),
        "Stations",
        overwrite,
        merge,
    )?;

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

    let data: VatglassesData = match serde_json::from_reader(file) {
        Ok(d) => d,
        Err(err) => {
            log::error(format_args!(
                "Failed to parse input file {input:?}: {err:?}"
            ));
            return Err(err.into());
        }
    };

    log::info(format_args!("Parsed VATglasses data: {data:?}"));

    let mut stations = match StationConfigFile::try_from_ref(&data) {
        Ok(s) => s,
        Err(err) => {
            log::error(format_args!(
                "Failed to convert VATglasses data to stations: {err:?}"
            ));
            return Err(err.into());
        }
    };

    if merge && output_stations.exists() {
        log::info(format_args!(
            "Reading existing stations from {output_stations:?}"
        ));
        let content = std::fs::read_to_string(&output_stations)?;
        let mut existing_config: StationConfigFile = toml::from_str(&content)?;
        let existing_ids: HashSet<_> = existing_config
            .stations
            .iter()
            .map(|s| s.id.clone())
            .collect();

        let mut added_count = 0;
        for station in stations.stations {
            if !existing_ids.contains(&station.id) {
                existing_config.stations.push(station);
                added_count += 1;
            }
        }
        stations.stations = existing_config.stations;
        log::info(format_args!("Merged {added_count} new stations"));
    }

    stations.stations.sort_by(|a, b| a.id.cmp(&b.id));

    let serialized_stations = match crate::format::serialize(&stations, format) {
        Ok(s) => s,
        Err(err) => {
            log::error(format_args!("Failed to serialize stations: {err:?}"));
            return Err(err);
        }
    };

    crate::write_output_file(&output_stations, &serialized_stations, "Stations")?;

    let mut positions = match PositionConfigFile::try_from_ref(&data) {
        Ok(p) => p,
        Err(err) => {
            log::error(format_args!(
                "Failed to convert VATglasses data to positions: {err:?}"
            ));
            return Err(err.into());
        }
    };

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
        for position in positions.positions {
            if !existing_ids.contains(&position.id) {
                existing_config.positions.push(position);
                added_count += 1;
            }
        }
        positions.positions = existing_config.positions;
        log::info(format_args!("Merged {added_count} new positions"));
    }

    positions.positions.sort_by(|a, b| {
        a.facility_type
            .cmp(&b.facility_type)
            .reverse()
            .then_with(|| a.id.cmp(&b.id))
    });

    let serialized_positions = match crate::format::serialize(&positions, format) {
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

#[derive(Deserialize)]
struct VatglassesData {
    pub airspace: Vec<Airspace>,
    pub positions: HashMap<String, Position>,
}

impl std::fmt::Debug for VatglassesData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VatglassesData")
            .field("airspace", &self.airspace.len())
            .field("positions", &self.positions.len())
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct Airspace {
    id: String,
    group: String,
    owner: Vec<String>,
}

#[derive(Deserialize)]
struct Position {
    pre: Vec<String>,
    r#type: String,
    frequency: Option<String>,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("pre", &self.pre.len())
            .field("type", &self.r#type)
            .field("frequency", &self.frequency)
            .finish()
    }
}

trait TryFromRef<T: ?Sized>: Sized {
    type Error;
    fn try_from_ref(value: &T) -> Result<Self, Self::Error>;
}

impl TryFromRef<VatglassesData> for PositionConfigFile {
    type Error = String;
    fn try_from_ref(value: &VatglassesData) -> Result<Self, Self::Error> {
        Ok(Self {
            positions: value
                .positions
                .iter()
                .map(|(id, p)| coverage::position::PositionRaw {
                    id: PositionId::from(id.clone()),
                    facility_type: p.r#type.parse().unwrap(), // TODO handle error
                    frequency: p.frequency.clone().unwrap_or("199.998".to_string()),
                    prefixes: p.pre.iter().cloned().collect(),
                    profile_id: None,
                })
                .collect(),
        })
    }
}

impl TryFromRef<VatglassesData> for StationConfigFile {
    type Error = String;
    fn try_from_ref(value: &VatglassesData) -> Result<Self, Self::Error> {
        let mut seen = HashSet::new();
        let mut duplicates = 0;

        let stations = value
            .airspace
            .iter()
            .map(|a| {
                let facility_type = FacilityType::from(a.group.clone());

                if !seen.insert((a.id.clone(), facility_type)) {
                    log::warn(format_args!(
                        "Duplicate airspace ID {} ({})",
                        style(format!("`{}`", a.id)).cyan(),
                        facility_type.as_str()
                    ));
                    duplicates += 1;
                }

                StationRaw {
                    id: StationId::from(a.id.clone()),
                    parent_id: None,
                    controlled_by: a
                        .owner
                        .iter()
                        .map(|o| PositionId::from(o.clone()))
                        .collect(),
                }
            })
            .collect();

        Ok(Self { stations })
    }
}
