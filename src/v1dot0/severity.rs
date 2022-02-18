use serde::{Deserialize, Serialize};

/// The expected impact of an alert to those it may affect.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    /// Extraordinary threat to life or property
    Extreme,
    /// Significant threat to life or property
    Severe,
    /// Possible threat to life or property
    Moderate,
    /// Minimal to no known threat to life or property
    Minor,
    /// Severity unknown
    Unknown,
}

impl Severity {
    /// Returns the name of the `Severity` as a `&str`.
    pub fn name(&self) -> &'static str {
        match self {
            Severity::Extreme => "Extreme",
            Severity::Severe => "Severe",
            Severity::Moderate => "Moderate",
            Severity::Minor => "Minor",
            Severity::Unknown => "Unknown",
        }
    }

    /// Returns the description of the `Severity` as a `&str`.
    pub fn description(&self) -> &'static str {
        match self {
            Severity::Extreme => "Extraordinary threat to life or property",
            Severity::Severe => "Significant threat to life or property",
            Severity::Moderate => "Possible threat to life or property",
            Severity::Minor => "Minimal to no known threat to life or property",
            Severity::Unknown => "Severity unknown",
        }
    }
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
