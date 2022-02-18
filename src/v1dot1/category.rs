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
    /// Chemical, Biological, Radiological, Nuclear or High-Yield Explosive threat or attack
    CBRNE,
    /// Other events
    Other,
}

impl From<crate::v1dot0::Category> for Category {
    fn from(prev: crate::v1dot0::Category) -> Self {
        use crate::v1dot0::Category as Prev;
        match prev {
            Prev::Geo => Category::Geo,
            Prev::Met => Category::Met,
            Prev::Safety => Category::Safety,
            Prev::Security => Category::Security,
            Prev::Rescue => Category::Rescue,
            Prev::Fire => Category::Fire,
            Prev::Health => Category::Health,
            Prev::Env => Category::Env,
            Prev::Transport => Category::Transport,
            Prev::Infra => Category::Infra,
            Prev::Other => Category::Other,
        }
    }
}
