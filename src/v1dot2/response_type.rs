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
    /// Avoid the subject event as per the `instruction`
    Avoid,
    /// Attend to information sources as described in `instruction`
    Monitor,
    /// Evaluate the information in this message
    Assess,
    /// The subject event no longer poses a threat or concern and any follow on action is described in `instruction`
    AllClear,
    /// No action recommended
    None,
}

impl From<crate::v1dot1::ResponseType> for ResponseType {
    fn from(prev: crate::v1dot1::ResponseType) -> Self {
        use crate::v1dot1::ResponseType as Prev;
        match prev {
            Prev::Shelter => ResponseType::Shelter,
            Prev::Evacuate => ResponseType::Evacuate,
            Prev::Prepare => ResponseType::Prepare,
            Prev::Execute => ResponseType::Execute,
            Prev::Monitor => ResponseType::Monitor,
            Prev::Assess => ResponseType::Assess,
            Prev::None => ResponseType::None,
        }
    }
}
