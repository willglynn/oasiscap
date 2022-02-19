//! Types for Google's [CAP Protocol Buffers] encoding.
//!
//! [CAP Protocol Buffers]: https://github.com/google/cap-library/blob/master/proto/cap.proto

// Import the prost-generated types
mod prost;

pub use self::prost::*;

mod alert_conversion;
mod area_conversion;
mod category;
mod certainty;
mod delimited_items;
mod geo;
mod info_conversion;
mod map;
mod message_type;
mod references;
mod resource_conversion;
mod response_type;
mod scope;
mod severity;
mod status;
mod urgency;

pub use alert_conversion::AlertConversionError;
pub use area_conversion::AreaConversionError;
pub use info_conversion::InfoConversionError;
pub use resource_conversion::ResourceConversionError;

impl TryFrom<Alert> for crate::Alert {
    type Error = AlertConversionError;

    fn try_from(value: Alert) -> Result<Self, Self::Error> {
        match value.xmlns.as_ref() {
            "http://www.incident.com/cap/1.0" => value.try_into().map(crate::Alert::V1dot0),
            "urn:oasis:names:tc:emergency:cap:1.1" => value.try_into().map(crate::Alert::V1dot1),
            "urn:oasis:names:tc:emergency:cap:1.2" => value.try_into().map(crate::Alert::V1dot2),
            _ => Err(AlertConversionError::Xmlns(value.xmlns)),
        }
    }
}

impl From<crate::Alert> for Alert {
    fn from(alert: crate::Alert) -> Self {
        match alert {
            crate::Alert::V1dot0(alert) => alert.into(),
            crate::Alert::V1dot1(alert) => alert.into(),
            crate::Alert::V1dot2(alert) => alert.into(),
        }
    }
}

fn datetime(optional: Option<String>) -> Result<Option<crate::DateTime>, chrono::ParseError> {
    match optional {
        Some(string) => string.parse().map(Some),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests;
