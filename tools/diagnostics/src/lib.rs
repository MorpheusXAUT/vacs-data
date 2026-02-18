use console::style;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LogFormat {
    #[default]
    Human,
    GitHub,
}

impl LogFormat {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            LogFormat::Human => "human",
            LogFormat::GitHub => "github",
        }
    }

    #[must_use]
    pub const fn variants() -> &'static [&'static str] {
        &["human", "github"]
    }
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for LogFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "human" => Ok(LogFormat::Human),
            "github" | "gh" => Ok(LogFormat::GitHub),
            other => Err(format!(
                "invalid diagnostics format '{other}'. expected one of: {}",
                Self::variants().join(", ")
            )),
        }
    }
}

pub struct Logger {
    format: LogFormat,
}

impl Logger {
    #[must_use]
    pub fn new(format: LogFormat) -> Self {
        Self { format }
    }

    #[must_use]
    pub fn is_human(&self) -> bool {
        self.format == LogFormat::Human
    }

    pub fn info(&self, message: impl std::fmt::Display) {
        match self.format {
            LogFormat::Human => eprintln!("{message}"),
            LogFormat::GitHub => println!("{message}"),
        }
    }

    pub fn warn(&self, message: impl std::fmt::Display) {
        match self.format {
            LogFormat::Human => {
                eprintln!("{} {}", style("warning:").yellow().bold(), message);
            }
            LogFormat::GitHub => println!("::warning::{message}"),
        }
    }

    pub fn error(&self, message: impl std::fmt::Display) {
        match self.format {
            LogFormat::Human => eprintln!("{} {}", style("error:").red().bold(), message),
            LogFormat::GitHub => println!("::error::{message}"),
        }
    }

    pub fn error_with_context(&self, context: &[String], message: impl std::fmt::Display) {
        match self.format {
            LogFormat::Human => {
                let context_str = if context.is_empty() {
                    String::new()
                } else {
                    format!(
                        "[{}]",
                        context
                            .iter()
                            .map(|c| style(c).cyan().to_string())
                            .collect::<Vec<_>>()
                            .join(" > ")
                    )
                };
                eprintln!(
                    "{}{} {}",
                    style("error:").red().bold(),
                    context_str,
                    message
                );
            }
            LogFormat::GitHub => {
                let msg = if context.is_empty() {
                    message.to_string()
                } else {
                    format!("{}: {}", context.join(" > "), message)
                };
                println!("::error::{msg}");
            }
        }
    }
}

use std::sync::OnceLock;

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn init(format: LogFormat) {
    let _ = LOGGER.set(Logger::new(format));
}

fn logger() -> &'static Logger {
    LOGGER.get_or_init(|| Logger::new(LogFormat::default()))
}

pub mod log {
    use super::logger;

    pub fn info(message: impl std::fmt::Display) {
        logger().info(message);
    }

    pub fn warn(message: impl std::fmt::Display) {
        logger().warn(message);
    }

    pub fn error(message: impl std::fmt::Display) {
        logger().error(message);
    }

    pub fn error_with_context(context: &[String], message: impl std::fmt::Display) {
        logger().error_with_context(context, message);
    }

    #[must_use]
    pub fn is_human() -> bool {
        logger().is_human()
    }
}
