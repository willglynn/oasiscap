use serde::{Deserialize, Serialize};

/// General categories into which an alert may be classified.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Category {
    /// Geophysical (including landslide)
    Geo,
    /// Meteorological (including flood)
    Met,
    /// General emergency and public safety
    Safety,
    /// Law enforcement, military, homeland and local/private security
    Security,
    /// Rescue and recovery
    Rescue,
    /// Fire suppression and rescue
    Fire,
    /// Medical and public health
    Health,
    /// Pollution and other environmental
    Env,
    /// Public and private transportation
    Transport,
    /// Utility, telecommunication, other non-transport infrastructure
    Infra,
    /// Other events
    Other,
}
