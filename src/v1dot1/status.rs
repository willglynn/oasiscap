use serde::{Deserialize, Serialize};

/// The intended handling of an alert message.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Status {
    /// Actionable by all targeted recipients
    Actual,
    /// Actionable only by designated exercise participants; exercise identifier SHOULD appear in `note`
    Exercise,
    /// For messages that support alert network internal functions
    System,
    /// Technical testing only, all recipients disregard
    Test,
    /// A preliminary template or draft, not actionable in its current form
    Draft,
}

impl From<crate::v1dot0::Status> for Status {
    fn from(s: crate::v1dot0::Status) -> Self {
        use crate::v1dot0::Status as Prev;
        match s {
            Prev::Actual => Self::Actual,
            Prev::Exercise => Self::Exercise,
            Prev::System => Self::System,
            Prev::Test => Self::Test,
        }
    }
}
