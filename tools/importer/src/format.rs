use serde::Serialize;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Toml,
    Json,
}

impl OutputFormat {
    #[must_use]
    pub const fn variants() -> &'static [&'static str] {
        &["toml", "json"]
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Toml => write!(f, "toml"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "toml" => Ok(OutputFormat::Toml),
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!(
                "Invalid output format: {}. Supported formats: {}",
                s,
                Self::variants().join(", ")
            )),
        }
    }
}

impl OutputFormat {
    #[must_use]
    pub const fn ext(&self) -> &'static str {
        match self {
            OutputFormat::Toml => "toml",
            OutputFormat::Json => "json",
        }
    }
}

pub fn serialize<T: Serialize>(
    value: &T,
    format: OutputFormat,
) -> Result<String, Box<dyn std::error::Error>> {
    match format {
        OutputFormat::Toml => Ok(toml::to_string_pretty(value)?),
        OutputFormat::Json => Ok(serde_json::to_string_pretty(value)?),
    }
}
