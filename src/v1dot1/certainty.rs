use serde::{Deserialize, Serialize};

/// The confidence in an observation or prediction.
///
/// # Example
///
/// ```
/// use serde_test::*;
/// use oasiscap::v1dot1::Certainty;
///
/// // Each value maps to and from a `<Certainty>` tag
/// for value in [
///     Certainty::Observed,
///     Certainty::Likely,
///     Certainty::Possible,
///     Certainty::Unlikely,
///     Certainty::Unknown,
/// ] {
///     assert_tokens(&value, &[Token::UnitVariant{ name: "Certainty", variant: value.name() }]);
/// }
///
/// // ...with one special case:
/// //   > For backward compatibility with CAP 1.0, the deprecated value of “Very Likely” SHOULD be
/// //   > treated as equivalent to “Likely.”
/// assert_tokens(
///     &Certainty::Likely,
///     &[Token::UnitVariant{ name: "Certainty", variant: "Likely" }],
/// );
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Certainty {
    /// Determined to have occurred or to be ongoing
    Observed,
    /// Likely (p > ~50%)
    #[serde(alias = "Very Likely")]
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
            Certainty::Observed => "Observed",
            Certainty::Likely => "Likely",
            Certainty::Possible => "Possible",
            Certainty::Unlikely => "Unlikely",
            Certainty::Unknown => "Unknown",
        }
    }

    /// Returns the description of the `Certainty` as a `&str`.
    pub fn description(&self) -> &'static str {
        match self {
            Certainty::Observed => "Determined to have occurred or to be ongoing",
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

impl From<crate::v1dot0::Certainty> for Certainty {
    fn from(value: crate::v1dot0::Certainty) -> Self {
        use crate::v1dot0::Certainty as V1dot0;
        match value {
            V1dot0::VeryLikely => Certainty::Likely,
            V1dot0::Likely => Certainty::Likely,
            V1dot0::Possible => Certainty::Possible,
            V1dot0::Unlikely => Certainty::Unlikely,
            V1dot0::Unknown => Certainty::Unknown,
        }
    }
}
