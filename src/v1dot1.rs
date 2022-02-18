//! Types for [CAP v1.1].
//!
//! Some of the types in this module are re-exported from the `v1dot0` module, since they did not
//! change between specifications.
//!
//! # Example
//!
//! ```rust
//! let alert: oasiscap::v1dot1::Alert = r#"
//! <?xml version="1.0" encoding="UTF-8"?>
//! <alert xmlns="urn:oasis:names:tc:emergency:cap:1.1">
//!   <identifier>43b080713727</identifier>
//!   <sender>hsas@dhs.gov</sender>
//!   <sent>2003-04-02T14:39:01-05:00</sent>
//!   <status>Actual</status>
//!   <msgType>Alert</msgType>
//!   <scope>Public</scope>
//!   <info>
//!     <!-- … -->
//!#     <category>Security</category>
//!#     <event>Homeland Security Advisory System Update</event>
//!#     <urgency>Immediate</urgency>
//!#     <severity>Severe</severity>
//!#     <certainty>Likely</certainty>
//!#     <senderName>U.S. Government, Department of Homeland Security</senderName>
//!#     <headline>Homeland Security Sets Code ORANGE</headline>
//!#     <description>The Department of Homeland Security has elevated the Homeland Security Advisory
//!# System threat level to ORANGE / High in response to intelligence which may indicate a heightened
//!# threat of terrorism.</description>
//!#     <instruction> A High Condition is declared when there is a high risk of terrorist attacks. In
//!# addition to the Protective Measures taken in the previous Threat Conditions, Federal departments
//!# and agencies should consider agency-specific Protective Measures in accordance with their
//!# existing plans.</instruction>
//!#     <web>http://www.dhs.gov/dhspublic/display?theme=29</web>
//!   </info>
//! </alert>
//! "#.parse().expect("parse CAP");
//!
//! assert_eq!(alert.identifier, "43b080713727");
//! ```
//!
//! [CAP v1.1]: (https://www.oasis-open.org/committees/download.php/14759/emergency-CAPv1.1.pdf)

use super::DateTime;
use serde::{Deserialize, Serialize};

pub use crate::v1dot0::{
    Circle, Id, Language, MessageType, Point, Polygon, References, Scope, Severity, Urgency,
};

mod status;
pub use status::Status;

mod category;
pub use category::Category;

mod certainty;
pub use certainty::Certainty;

mod response_type;
pub use response_type::ResponseType;

mod map;
use crate::delimited_items::Items;
pub use map::Map;

/// A CAP v1.1 alert message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:alert"
)]
pub struct Alert {
    /// A unique identifier for this alert, assigned by the sender
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:identifier"
    )]
    pub identifier: Id,

    /// A globally-unique identifier for the sender
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:sender"
    )]
    pub sender: Id,

    /// The date and time at which this alert originated
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:sent"
    )]
    pub sent: DateTime,

    /// The intended handling of the alert message
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:status"
    )]
    pub status: Status,

    /// A classification describing the nature of the alert message
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:msgType"
    )]
    pub message_type: MessageType,

    /// Text identifying the source of the alert message, which may be an operator or a device
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:source",
        skip_serializing_if = "Option::is_none"
    )]
    pub source: Option<String>,

    /// The intended distribution scope of the alert message
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:scope"
    )]
    pub scope: Scope,

    /// The rule by which the distribution of this alert is to be restricted, if `Scope::Restricted`
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:restriction",
        skip_serializing_if = "Option::is_none"
    )]
    pub restriction: Option<String>,

    /// The group listing of intended recipients of this alert message, if `Scope::Private`
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:addresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub addresses: Option<Items>,

    /// User-defined flags or special codes used to flag the alert message for special handling
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:code",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub codes: Vec<String>,

    /// Text describing the purpose or significance of this alert message, intended primarily for
    /// use with `MessageType::Cancel` and `MessageType::Error`.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:note",
        skip_serializing_if = "Option::is_none"
    )]
    pub note: Option<String>,

    /// Alert(s) to which this alert refers
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:references",
        skip_serializing_if = "Option::is_none"
    )]
    pub references: Option<References>,

    /// The group listing naming the referent incident(s) of the alert message.
    ///
    /// Used to collate multiple messages referring to different aspects of the same incident.
    ///
    /// (If you understand what this means, please open an issue.)
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:incidents",
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
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:info",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub info: Vec<Info>,
}

#[derive(Serialize, Deserialize)]
struct AlertDocument {
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:alert"
    )]
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

impl From<crate::v1dot0::Alert> for Alert {
    fn from(prev: crate::v1dot0::Alert) -> Self {
        Self {
            identifier: prev.identifier,
            sender: prev.sender,
            sent: prev.sent,
            status: prev.status.into(),
            message_type: prev.message_type,
            source: prev.source,
            scope: prev.scope,
            restriction: prev.restriction,
            addresses: prev.addresses,
            codes: prev.codes,
            note: prev.note,
            references: prev.references,
            incidents: prev.incidents,
            info: prev.info.into_iter().map(Info::from).collect(),
        }
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
#[serde(
    rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:info"
)]
pub struct Info {
    /// The language of this `Info` section.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:language",
        skip_serializing_if = "Language::is_empty"
    )]
    pub language: Language,

    /// Zero or more categories describing the subject event.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:category",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<Category>,

    /// Text describing the subject event.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:event"
    )]
    pub event: String,

    /// The recommended type of action for the target audience.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:responseType",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub response_type: Vec<ResponseType>,

    /// The time available to prepare for the subject event.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:urgency"
    )]
    pub urgency: Urgency,

    /// The intensity of impact of the subject event.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:severity"
    )]
    pub severity: Severity,

    /// The confidence in the observation or prediction.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:certainty"
    )]
    pub certainty: Certainty,

    /// The target audience of the alert message.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:audience",
        skip_serializing_if = "Option::is_none"
    )]
    pub audience: Option<String>,

    /// System-specific codes identifying the event type of the alert message
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:eventCode",
        default,
        skip_serializing_if = "Map::is_empty"
    )]
    pub event_codes: Map,

    /// The effective time of the information of the alert message
    ///
    /// If this item is not included, the effective time SHALL be assumed to be the same as in
    /// `sent`.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:effective",
        skip_serializing_if = "Option::is_none"
    )]
    pub effective: Option<DateTime>,

    /// The expected time of the beginning of the subject event of the alert message.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:onset",
        skip_serializing_if = "Option::is_none"
    )]
    pub onset: Option<DateTime>,

    /// The expiry time of the information of the alert message.
    ///
    /// If this item is not provided, each recipient is free to set its own policy as to when the
    /// message is no longer in effect.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:expires",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<DateTime>,

    /// The human-readable name of the agency or authority issuing this alert.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:senderName",
        skip_serializing_if = "Option::is_none"
    )]
    pub sender_name: Option<String>,

    /// A brief human-readable headline.
    ///
    /// Note that some displays (for example, short messaging service devices) may only present this
    /// headline; it SHOULD be made as direct and actionable as possible while remaining short. 160
    /// characters MAY be a useful target limit for headline length.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:headline",
        skip_serializing_if = "Option::is_none"
    )]
    pub headline: Option<String>,

    /// An extended human readable description of the hazard or event that occasioned this message.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:description",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    /// An extended human readable instruction to targeted recipients. If different instructions are
    /// intended for different recipients, they should be represented by use of multiple `Info`
    /// blocks.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:instruction",
        skip_serializing_if = "Option::is_none"
    )]
    pub instruction: Option<String>,

    /// A full, absolute URI for an HTML page or other text resource with additional or reference
    /// information regarding this alert.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:web",
        deserialize_with = "crate::url::deserialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub web: Option<url::Url>,

    /// The text describing the contact for follow-up and confirmation of the alert message
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:contact",
        skip_serializing_if = "Option::is_none"
    )]
    pub contact: Option<String>,

    /// System-specific additional parameters associated with the alert message
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:parameter",
        skip_serializing_if = "Map::is_empty",
        default
    )]
    pub parameters: Map,

    /// Additional content related to this event.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:resource",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub resources: Vec<Resource>,

    /// Geographical (and usually also geospatial) information describing the expected or actual
    /// location of the event.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:area",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub areas: Vec<Area>,
}

impl From<crate::v1dot0::Info> for Info {
    fn from(prev: crate::v1dot0::Info) -> Self {
        Self {
            language: prev.language,
            categories: prev.categories.into_iter().map(Category::from).collect(),
            event: prev.event,
            response_type: Default::default(),
            urgency: prev.urgency,
            severity: prev.severity,
            certainty: prev.certainty.into(),
            audience: prev.audience,
            event_codes: prev.event_codes.into(),
            effective: prev.effective,
            onset: prev.onset,
            expires: prev.expires,
            sender_name: prev.sender_name,
            headline: prev.headline,
            description: prev.description,
            instruction: prev.instruction,
            web: prev.web,
            contact: prev.contact,
            parameters: prev.parameters.into(),
            resources: prev.resources.into_iter().map(Resource::from).collect(),
            areas: prev.areas.into_iter().map(Area::from).collect(),
        }
    }
}

/// A reference to additional information related to an event, in the form of a digital asset such
/// as an image or audio file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:resource"
)]
pub struct Resource {
    /// The text describing the type and content of the resource file
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:resourceDesc"
    )]
    pub description: String,

    /// The identifier of the MIME content type and sub-type describing the resource file
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:mimeType",
        skip_serializing_if = "Option::is_none"
    )]
    pub mime_type: Option<String>,

    /// Approximate size of the resource file in bytes
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:size",
        skip_serializing_if = "Option::is_none"
    )]
    pub size: Option<u64>,

    /// A full absolute URI, typically a Uniform Resource Locator that can be used to retrieve the
    /// resource over the Internet
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:uri",
        deserialize_with = "crate::url::deserialize",
        skip_serializing_if = "Option::is_none",
        default
    )]
    // TODO:
    //  > OR
    //  > a relative URI to name the content of a <derefUri> element if one is present in this
    //  > resource block.
    pub uri: Option<url::Url>,

    /// The resource content itself, embedded inside the resource description.
    ///
    /// OASIS calls this `<derefUri>`, but it is literally the resource data encoded as base64 and
    /// transmitted inline. This crate calls it `embedded_content` for clarity.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:derefUri",
        skip_serializing_if = "Option::is_none"
    )]
    pub embedded_content: Option<crate::EmbeddedContent>,

    /// A cryptographic hash of the resource content.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:digest",
        skip_serializing_if = "Option::is_none"
    )]
    pub digest: Option<crate::Sha1Digest>,
}

impl From<crate::v1dot0::Resource> for Resource {
    fn from(prev: crate::v1dot0::Resource) -> Self {
        Self {
            description: prev.description,
            mime_type: prev.mime_type,
            size: prev.size,
            uri: prev.uri,
            embedded_content: None,
            digest: prev.digest,
        }
    }
}

/// Geographical (and usually also geospatial) information describing the expected or actual
/// location of the event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(
    rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:area"
)]
pub struct Area {
    /// A text description of the affected area.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:areaDesc"
    )]
    pub description: String,

    /// Geospatial polygons denoting the affected area, if any.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:polygon",
        default,
        deserialize_with = "Polygon::deserialize_optional",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub polygons: Vec<Polygon>,

    /// Geospatial circles denoting the affected area, if any.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:circle",
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
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:geocode",
        default,
        skip_serializing_if = "Map::is_empty"
    )]
    pub geocode: Map,

    /// The specific or minimum altitude of the affected area of the alert message, in feet above
    /// WGS 84 mean sea level.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:altitude",
        skip_serializing_if = "Option::is_none"
    )]
    pub altitude: Option<i64>,

    /// The maximum altitude of the affected area of the alert message, in feet above WGS 84 mean
    /// sea level.
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:ceiling",
        skip_serializing_if = "Option::is_none"
    )]
    pub ceiling: Option<i64>,
}

impl From<crate::v1dot0::Area> for Area {
    fn from(prev: crate::v1dot0::Area) -> Self {
        Self {
            description: prev.description,
            polygons: prev.polygons,
            circles: prev.circles,
            geocode: prev.geocode.into(),
            altitude: prev.altitude,
            ceiling: prev.ceiling,
        }
    }
}

#[cfg(test)]
mod tests;
