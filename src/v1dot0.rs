//! Types for [CAP v1.0].
//!
//! [CAP v1.0]: https://www.oasis-open.org/committees/download.php/6334/oasis-200402-cap-core-1.0.pdf

use super::DateTime;
use serde::{Deserialize, Serialize};

mod id;
pub use id::Id;

mod status;
pub use status::Status;

mod scope;
pub use scope::Scope;

mod message_type;
pub use message_type::MessageType;

mod references;
pub use references::References;

mod language;
pub use language::Language;

mod category;
pub use category::Category;

mod urgency;
pub use urgency::Urgency;

mod severity;
pub use severity::Severity;

mod certainty;
pub use certainty::Certainty;

pub mod map;
pub use map::Map;

mod point;
pub use point::Point;

mod polygon;
pub use polygon::Polygon;

mod circle;

use crate::delimited_items::Items;
pub use circle::Circle;

/// A CAP v1.0 alert message.
///
/// The `Alert` segment provides basic information about the current message: its purpose, its
/// `source and its status, as well as unique identifier for the current message and links to any
/// other related messages.
///
/// An `Alert` may be used alone for message acknowledgements, cancellations or other system
// functions, but most `Alert` segments will include at least one `Info` segment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "{http://www.incident.com/cap/1.0}cap:alert")]
pub struct Alert {
    /// A unique identifier for this alert, assigned by the sender
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:identifier")]
    pub identifier: Id,

    /// A globally-unique identifier for the sender
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:sender")]
    pub sender: Id,

    /// Used for authenticating the sender. Note that this element should only be used on secure
    /// channels, and that simple password authentication schemes have numerous well-known
    /// weaknesses.
    ///
    /// `password` was removed in CAP v1.1.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:password",
        skip_serializing_if = "Option::is_none"
    )]
    pub password: Option<String>,

    /// Text identifying the source of the alert message, which may be an operator or a device
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:source",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<String>,

    /// The date and time at which this alert originated
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:sent")]
    pub sent: DateTime,

    /// The intended handling of the alert message
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:status")]
    pub status: Status,

    /// The intended distribution scope of the alert message
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:scope")]
    pub scope: Scope,

    /// The rule by which the distribution of this alert is to be restricted, if `Scope::Restricted`
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:restriction",
        skip_serializing_if = "Option::is_none"
    )]
    pub restriction: Option<String>,

    /// The group listing of intended recipients of this alert message, if `Scope::Private`
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:addresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub addresses: Option<Items>,

    /// User-defined flags or special codes used to flag the alert message for special handling
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:code",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub codes: Vec<String>,

    /// A classification describing the nature of the alert message
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:msgType")]
    pub message_type: MessageType,

    /// Text describing the purpose or significance of this alert message, intended primarily for
    /// use with `MessageType::Cancel` and `MessageType::Error`.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:note",
        skip_serializing_if = "Option::is_none"
    )]
    pub note: Option<String>,

    /// Alert(s) to which this alert refers
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:references",
        skip_serializing_if = "Option::is_none"
    )]
    pub references: Option<References>,

    /// The group listing naming the referent incident(s) of the alert message.
    ///
    /// Used to collate multiple messages referring to different aspects of the same incident.
    ///
    /// (If you understand what this means, please open an issue.)
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:incidents",
        skip_serializing_if = "Option::is_none"
    )]
    pub incidents: Option<Items>,

    /// Sub-elements describing the alert.
    ///
    /// Multiple occurrences are permitted within a single `Alert`. If targeting of multiple `Info`
    /// blocks in the same language overlaps, information in later blocks may expand but may not
    /// override the corresponding values in earlier ones. Each set of `Info` blocks containing the
    /// same language identifier is to be treated as a separate sequence.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:info",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub info: Vec<Info>,
}

#[derive(Serialize, Deserialize)]
struct AlertDocument {
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:alert")]
    alert: Alert,
}

impl std::str::FromStr for Alert {
    type Err = xml_serde::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        xml_serde::from_str::<AlertDocument>(s).map(|doc| doc.alert)
    }
}

impl std::fmt::Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        xml_serde::to_string(self)
            .map_err(|_| std::fmt::Error)
            .and_then(|str| f.write_str(&str))
    }
}

/// Information about anticipated or actual event.
///
/// `Info` describes the event's `urgency` (time available to prepare), `severity` (intensity of
/// impact), and `certainty` (confidence in the observation or prediction), as well as providing
/// both categorical and textual descriptions of the subject event. It may also provide instructions
/// for appropriate response by message recipients and various other details (hazard duration,
/// technical parameters, contact information, links to additional information sources, etc.)
///
/// Multiple `Info` segments may be used to describe differing parameters (e.g., for different
/// probability or intensity “bands”), and/or to provide the information in multiple languages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "{http://www.incident.com/cap/1.0}cap:info")]
pub struct Info {
    /// The language of this `Info` section.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:language",
        skip_serializing_if = "Language::is_empty"
    )]
    pub language: Language,

    /// Zero or more categories describing the subject event.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:category",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<Category>,

    /// Text describing the subject event.
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:event")]
    pub event: String,

    /// The time available to prepare for the subject event.
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:urgency")]
    pub urgency: Urgency,

    /// The intensity of impact of the subject event.
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:severity")]
    pub severity: Severity,

    /// The confidence in the observation or prediction.
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:certainty")]
    pub certainty: Certainty,

    /// The target audience of the alert message.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:audience",
        skip_serializing_if = "Option::is_none"
    )]
    pub audience: Option<String>,

    /// System-specific codes identifying the event type of the alert message.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:eventCode",
        skip_serializing_if = "Map::is_empty"
    )]
    pub event_codes: Map,

    /// The effective time of the information of the alert message
    ///
    /// If this item is not included, the effective time SHALL be assumed to be the same as in
    /// `sent`.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:effective",
        skip_serializing_if = "Option::is_none"
    )]
    pub effective: Option<DateTime>,

    /// The expected time of the beginning of the subject event of the alert message.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:onset",
        skip_serializing_if = "Option::is_none"
    )]
    pub onset: Option<DateTime>,

    /// The expiry time of the information of the alert message.
    ///
    /// If this item is not provided, each recipient is free to set its own policy as to when the
    /// message is no longer in effect.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:expires",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<DateTime>,

    /// The human-readable name of the agency or authority issuing this alert.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:senderName",
        skip_serializing_if = "Option::is_none"
    )]
    pub sender_name: Option<String>,

    /// A brief human-readable headline.
    ///
    /// Note that some displays (for example, short messaging service devices) may only present this
    /// headline; it SHOULD be made as direct and actionable as possible while remaining short. 160
    /// characters MAY be a useful target limit for headline length.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:headline",
        skip_serializing_if = "Option::is_none"
    )]
    pub headline: Option<String>,

    /// An extended human readable description of the hazard or event that occasioned this message.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:description",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    /// An extended human readable instruction to targeted recipients. If different instructions are
    /// intended for different recipients, they should be represented by use of multiple `Info`
    /// blocks.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:instruction",
        skip_serializing_if = "Option::is_none"
    )]
    pub instruction: Option<String>,

    /// A full, absolute URI for an HTML page or other text resource with additional or reference
    /// information regarding this alert.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:web",
        deserialize_with = "crate::url::deserialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub web: Option<url::Url>,

    /// The text describing the contact for follow-up and confirmation of the alert message
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:contact",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact: Option<String>,

    /// System-specific additional parameters associated with the alert message
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:parameter",
        skip_serializing_if = "Map::is_empty"
    )]
    pub parameters: Map,

    /// Additional content related to this event.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:resource",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<Resource>,

    /// Geographical (and usually also geospatial) information describing the expected or actual
    /// location of the event.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:area",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub areas: Vec<Area>,
}

/// A reference to additional information related to an event, in the form of a digital asset such
/// as an image or audio file.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "{http://www.incident.com/cap/1.0}cap:resource")]
pub struct Resource {
    /// The text describing the type and content of the resource file
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:resourceDesc")]
    pub description: String,

    /// The identifier of the MIME content type and sub-type describing the resource file
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:mimeType",
        skip_serializing_if = "Option::is_none"
    )]
    pub mime_type: Option<String>,

    /// Approximate size of the resource file in bytes
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:size",
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<u64>,

    /// A full absolute URI, typically a Uniform Resource Locator that can be used to retrieve the
    /// resource over the Internet
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:uri",
        deserialize_with = "crate::url::deserialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub uri: Option<url::Url>,

    /// A cryptographic hash of the resource content.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:digest",
        skip_serializing_if = "Option::is_none"
    )]
    pub digest: Option<crate::Sha1Digest>,
}

/// Geographical (and usually also geospatial) information describing the expected or actual
/// location of the event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "{http://www.incident.com/cap/1.0}cap:area")]
pub struct Area {
    /// A text description of the affected area.
    #[serde(rename = "{http://www.incident.com/cap/1.0}cap:areaDesc")]
    pub description: String,

    /// Geospatial polygons denoting the affected area, if any.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:polygon",
        default,
        deserialize_with = "Polygon::deserialize_optional",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub polygons: Vec<Polygon>,

    /// Geospatial circles denoting the affected area, if any.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:circle",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub circles: Vec<Circle>,

    /// Geographic codes delineating the affected area of the alert message.
    ///
    /// This element is primarily for compatibility with other systems. Use of this element presumes
    /// knowledge of the coding system on the part of recipients; therefore, for interoperability,
    /// it should be used in concert with an equivalent description in the more universally
    /// understood `polygons` and `circles` forms whenever possible.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:geocode",
        skip_serializing_if = "Map::is_empty"
    )]
    pub geocode: Map,

    /// The specific or minimum altitude of the affected area of the alert message, in feet above
    /// WGS 84 mean sea level.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:altitude",
        skip_serializing_if = "Option::is_none"
    )]
    pub altitude: Option<i64>,

    /// The maximum altitude of the affected area of the alert message, in feet above WGS 84 mean
    /// sea level.
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:ceiling",
        skip_serializing_if = "Option::is_none"
    )]
    pub ceiling: Option<i64>,
}

#[cfg(test)]
mod tests;
