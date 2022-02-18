use serde::{Deserialize, Serialize};

/// The recommended type of action for the target audience.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResponseType {
    /// Take shelter in place or per `instruction`
    Shelter,
    /// Relocate as instructed in the `instruction`
    Evacuate,
    /// Make preparations per the `instruction`
    Prepare,
    /// Execute a pre-planned activity identified in `instruction`
    Execute,
    /// Attend to information sources as described in `instruction`
    Monitor,
    /// Evaluate the information in this message
    Assess,
    /// No action recommended
    None,
}
