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
}
