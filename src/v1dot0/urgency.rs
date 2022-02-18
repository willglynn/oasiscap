use serde::{Deserialize, Serialize};

/// The time-sensitivity of an alert.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Urgency {
    /// Responsive action SHOULD be taken immediately
    Immediate,
    /// Responsive action SHOULD be taken soon (within next hour)
    Expected,
    /// Responsive action SHOULD be taken in the near future
    Future,
    /// Responsive action is no longer required
    Past,
    /// Urgency not known
    Unknown,
}
