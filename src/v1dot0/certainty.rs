use serde::{Deserialize, Serialize};

/// The confidence in an observation or prediction.
///
/// # Example
///
/// ```
/// use serde_test::*;
/// use oasiscap::v1dot0::Certainty;
///
/// // Each value maps to and from a `<Certainty>` tag
/// for value in [
///     Certainty::VeryLikely,
///     Certainty::Likely,
///     Certainty::Possible,
///     Certainty::Unlikely,
///     Certainty::Unknown,
/// ] {
///     assert_tokens(&value, &[Token::UnitVariant{ name: "Certainty", variant: value.name() }]);
/// }
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Certainty {
    /// Highly likely (p > ~ 85%) or certain
    ///
    /// `VeryLikely` was removed in CAP v1.1.
    #[serde(rename = "Very Likely")]
    VeryLikely,

    /// Likely (p > ~50%)
    Likely,

    /// Possible but not likely (p <= ~50%)
    Possible,

    /// Not expected to occur (p ~ 0)
    Unlikely,

    /// Certainty unknown
    Unknown,
}

impl Certainty {
    /// Returns the name of the `Certainty` as a `&str`.
    pub fn name(&self) -> &'static str {
        match self {
            Certainty::VeryLikely => "Very Likely",
            Certainty::Likely => "Likely",
            Certainty::Possible => "Possible",
            Certainty::Unlikely => "Unlikely",
            Certainty::Unknown => "Unknown",
        }
    }

    /// Returns the description of the `Certainty` as a `&str`.
    pub fn description(&self) -> &'static str {
        match self {
            Certainty::VeryLikely => "Highly likely (p > ~ 85%) or certain",
            Certainty::Likely => "Likely (p > ~50%)",
            Certainty::Possible => "Possible but not likely (p <= ~50%)",
            Certainty::Unlikely => "Not expected to occur (p ~ 0)",
            Certainty::Unknown => "Certainty unknown",
        }
    }
}

impl std::fmt::Display for Certainty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
