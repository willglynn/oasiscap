//! Types for Google's [CAP Protocol Buffers] encoding.
//!
//! [CAP Protocol Buffers]: https://github.com/google/cap-library/blob/master/proto/cap.proto

// Import the prost-generated types
mod prost;

pub use self::prost::*;
use crate::delimited_items::Items;
use crate::geo::InvalidCircleError;
use crate::map::Map;

impl From<crate::delimited_items::Items> for Group {
    fn from(value: Items) -> Self {
        Self {
            value: value.into_iter().map(String::from).collect(),
        }
    }
}

impl TryFrom<Group> for crate::delimited_items::Items {
    type Error = crate::delimited_items::InvalidItemError;

    fn try_from(value: Group) -> Result<Self, Self::Error> {
        value
            .value
            .into_iter()
            .map(crate::delimited_items::Item::try_from)
            .collect()
    }
}

impl TryFrom<Group> for crate::references::References {
    type Error = crate::references::ReferenceError;

    fn try_from(value: Group) -> Result<Self, Self::Error> {
        Ok(value
            .value
            .into_iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?)
    }
}

impl<E: crate::map::Entry> From<Vec<ValuePair>> for crate::map::Map<E> {
    fn from(value: Vec<ValuePair>) -> Self {
        value
            .into_iter()
            .map(|pair| (pair.value_name, pair.value))
            .collect()
    }
}

impl<E: crate::map::Entry> From<crate::map::Map<E>> for Vec<ValuePair> {
    fn from(value: Map<E>) -> Self {
        value
            .into_iter()
            .map(|(value_name, value)| ValuePair { value_name, value })
            .collect()
    }
}

impl TryFrom<Vec<ValuePair>> for crate::v1dot0::Map {
    type Error = crate::v1dot0::map::InvalidKeyError;

    fn try_from(value: Vec<ValuePair>) -> Result<Self, Self::Error> {
        value
            .into_iter()
            .map(|pair| {
                crate::v1dot0::map::Key::try_from(pair.value_name).map(|key| (key, pair.value))
            })
            .collect()
    }
}

impl From<crate::v1dot0::Map> for Vec<ValuePair> {
    fn from(value: crate::v1dot0::Map) -> Self {
        value
            .into_iter()
            .map(|(value_name, value)| ValuePair {
                value_name: value_name.into(),
                value,
            })
            .collect()
    }
}

impl From<crate::geo::Point> for Point {
    fn from(value: crate::geo::Point) -> Self {
        Self {
            latitude: value.latitude(),
            longitude: value.longitude(),
        }
    }
}

impl TryFrom<Point> for crate::geo::Point {
    type Error = crate::geo::InvalidPointError;

    fn try_from(value: Point) -> Result<Self, Self::Error> {
        (value.latitude, value.longitude).try_into()
    }
}

impl From<crate::geo::Polygon> for Polygon {
    fn from(value: crate::geo::Polygon) -> Self {
        Self {
            point: value.into_iter().map(Point::from).collect(),
        }
    }
}

impl TryFrom<Polygon> for crate::geo::Polygon {
    type Error = crate::geo::InvalidPolygonError;

    fn try_from(value: Polygon) -> Result<Self, Self::Error> {
        value
            .point
            .into_iter()
            .map(crate::geo::Point::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
    }
}

impl From<crate::geo::Circle> for Circle {
    fn from(value: crate::geo::Circle) -> Self {
        Self {
            point: value.center.into(),
            radius: value.radius,
        }
    }
}

impl TryFrom<Circle> for crate::geo::Circle {
    type Error = InvalidCircleError;

    fn try_from(value: Circle) -> Result<Self, Self::Error> {
        crate::geo::Circle::new(value.point.try_into()?, value.radius)
    }
}

/// The error returned when an `Area` conversion would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum AreaConversionError {
    /// A polygon is invalid
    #[error("polygon is invalid: {0}")]
    Polygon(#[from] crate::geo::InvalidPolygonError),
    /// A circle is invalid
    #[error("circle is invalid: {0}")]
    Circle(#[from] crate::geo::InvalidCircleError),
}

impl TryFrom<Area> for crate::v1dot2::Area {
    type Error = AreaConversionError;

    fn try_from(value: Area) -> Result<Self, Self::Error> {
        Ok(Self {
            description: value.area_desc,
            polygons: value
                .polygon
                .into_iter()
                .map(crate::geo::Polygon::try_from)
                .collect::<Result<_, _>>()?,
            circles: value
                .circle
                .into_iter()
                .map(crate::geo::Circle::try_from)
                .collect::<Result<_, _>>()?,
            geocode: value.geocode.into(),
            altitude: value.altitude,
            ceiling: value.ceiling,
        })
    }
}

impl From<crate::v1dot2::Area> for Area {
    fn from(value: crate::v1dot2::Area) -> Self {
        Self {
            area_desc: value.description,
            polygon: value.polygons.into_iter().map(Polygon::from).collect(),
            circle: value.circles.into_iter().map(Circle::from).collect(),
            geocode: value.geocode.into(),
            altitude: value.altitude,
            ceiling: value.ceiling,
        }
    }
}

/// The error returned when a `Resource` conversion would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum ResourceConversionError {
    /// Size is invalid
    #[error("size is invalid: {0}")]
    Size(i64),
    /// URI is invalid
    #[error("URI is invalid: {0:?}")]
    Uri(String),
    /// Deref URI is invalid
    #[error("deref URI is invalid")]
    DerefUri,
    /// Digest is invalid
    #[error("digest is invalid: {0}")]
    Digest(#[from] crate::digest::Sha1ParseError),
}

impl TryFrom<Resource> for crate::v1dot2::Resource {
    type Error = ResourceConversionError;

    fn try_from(value: Resource) -> Result<Self, Self::Error> {
        Ok(Self {
            description: value.resource_desc,
            mime_type: value
                .mime_type
                .unwrap_or_else(|| "application/octet-stream".into()),
            size: match value.size {
                Some(v) => Some(v.try_into().map_err(|_| ResourceConversionError::Size(v))?),
                None => None,
            },
            uri: match value.uri {
                Some(string) => {
                    crate::url::parse(&string).map_err(|_| ResourceConversionError::Uri(string))?
                }
                None => None,
            },
            embedded_content: match value.deref_uri {
                Some(string) => Some(
                    crate::EmbeddedContent::try_from(string)
                        .map_err(|_| ResourceConversionError::DerefUri)?,
                ),
                None => None,
            },
            digest: match value.digest {
                Some(string) => Some(string.parse()?),
                None => None,
            },
        })
    }
}

impl From<crate::v1dot2::Resource> for Resource {
    fn from(value: crate::v1dot2::Resource) -> Self {
        Self {
            resource_desc: value.description,
            mime_type: value.mime_type.into(),
            size: value.size.and_then(|v| v.try_into().ok()),
            uri: value.uri.map(|v| v.to_string()),
            deref_uri: value
                .embedded_content
                .as_ref()
                .map(crate::EmbeddedContent::to_string),
            digest: value.digest.as_ref().map(crate::digest::Sha1::to_string),
        }
    }
}

/// The error returned when an `Info` conversion would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum InfoConversionError {
    /// Language is invalid
    #[error("language is invalid: {0}")]
    Language(#[from] crate::language::InvalidLanguageError),
    /// Category is invalid
    #[error("category is invalid: {0}")]
    Category(i32),
    /// Response type is invalid
    #[error("response type is invalid: {0}")]
    ResponseType(i32),
    /// Urgency is invalid
    #[error("urgency is invalid: {0}")]
    Urgency(i32),
    /// Severity is invalid
    #[error("severity is invalid: {0}")]
    Severity(i32),
    /// Certainty is invalid
    #[error("certainty is invalid: {0}")]
    Certainty(i32),
    /// Effective is invalid
    #[error("effective is invalid: {0}")]
    Effective(chrono::ParseError),
    /// Effective is invalid
    #[error("onset is invalid: {0}")]
    Onset(chrono::ParseError),
    /// Effective is invalid
    #[error("expires is invalid: {0}")]
    Expires(chrono::ParseError),
    /// Web is invalid
    #[error("web is invalid: {0:?}")]
    Web(String),
    /// Resource is invalid
    #[error("resource is invalid: {0}")]
    Resource(#[from] ResourceConversionError),
    /// Area is invalid
    #[error("area is invalid: {0}")]
    Area(#[from] AreaConversionError),
}

impl TryFrom<Info> for crate::v1dot2::Info {
    type Error = InfoConversionError;

    fn try_from(value: Info) -> Result<Self, Self::Error> {
        Ok(Self {
            language: crate::language::Language::new(value.language)?,
            categories: value
                .category
                .into_iter()
                .map(|v| {
                    self::info::Category::from_i32(v)
                        .map(|v| v.into())
                        .ok_or(InfoConversionError::Category(v))
                })
                .collect::<Result<_, _>>()?,
            event: value.event,
            response_type: value
                .response_type
                .into_iter()
                .map(|v| {
                    self::info::ResponseType::from_i32(v)
                        .map(|v| v.into())
                        .ok_or(InfoConversionError::ResponseType(v))
                })
                .collect::<Result<_, _>>()?,
            urgency: self::info::Urgency::from_i32(value.urgency)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Urgency(value.urgency))?,
            severity: self::info::Severity::from_i32(value.severity)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Severity(value.severity))?,
            certainty: self::info::Certainty::from_i32(value.certainty)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Certainty(value.certainty))?,
            audience: value.audience,
            event_codes: value.event_code.into(),
            effective: datetime(value.effective).map_err(InfoConversionError::Effective)?,
            onset: datetime(value.onset).map_err(InfoConversionError::Onset)?,
            expires: datetime(value.expires).map_err(InfoConversionError::Expires)?,
            sender_name: value.sender_name,
            headline: value.headline,
            description: value.description,
            instruction: value.instruction,
            web: match value.web {
                Some(str) => Some(str.parse().map_err(|_| InfoConversionError::Web(str))?),
                None => None,
            },
            contact: value.contact,
            parameters: value.parameter.into(),
            resources: value
                .resource
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<_, _>>()?,
            areas: value
                .area
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<_, _>>()?,
        })
    }
}

fn datetime(optional: Option<String>) -> Result<Option<crate::DateTime>, chrono::ParseError> {
    match optional {
        Some(string) => string.parse().map(Some),
        None => Ok(None),
    }
}

impl From<self::info::Category> for crate::v1dot2::Category {
    fn from(value: self::info::Category) -> Self {
        match value {
            self::info::Category::Geo => Self::Geo,
            self::info::Category::Met => Self::Met,
            self::info::Category::Safety => Self::Safety,
            self::info::Category::Security => Self::Security,
            self::info::Category::Rescue => Self::Rescue,
            self::info::Category::Fire => Self::Fire,
            self::info::Category::Health => Self::Health,
            self::info::Category::Env => Self::Env,
            self::info::Category::Transport => Self::Transport,
            self::info::Category::Infra => Self::Infra,
            self::info::Category::Cbrne => Self::CBRNE,
            self::info::Category::Other => Self::Other,
        }
    }
}

impl From<crate::v1dot2::Category> for self::info::Category {
    fn from(value: crate::v1dot1::Category) -> Self {
        match value {
            crate::v1dot1::Category::Geo => Self::Geo,
            crate::v1dot1::Category::Met => Self::Met,
            crate::v1dot1::Category::Safety => Self::Safety,
            crate::v1dot1::Category::Security => Self::Security,
            crate::v1dot1::Category::Rescue => Self::Rescue,
            crate::v1dot1::Category::Fire => Self::Fire,
            crate::v1dot1::Category::Health => Self::Health,
            crate::v1dot1::Category::Env => Self::Env,
            crate::v1dot1::Category::Transport => Self::Transport,
            crate::v1dot1::Category::Infra => Self::Infra,
            crate::v1dot1::Category::CBRNE => Self::Cbrne,
            crate::v1dot1::Category::Other => Self::Other,
        }
    }
}

impl From<self::info::ResponseType> for crate::v1dot2::ResponseType {
    fn from(value: self::info::ResponseType) -> Self {
        match value {
            self::info::ResponseType::Shelter => Self::Shelter,
            self::info::ResponseType::Evacuate => Self::Evacuate,
            self::info::ResponseType::Prepare => Self::Prepare,
            self::info::ResponseType::Execute => Self::Execute,
            self::info::ResponseType::Avoid => Self::Avoid,
            self::info::ResponseType::Monitor => Self::Monitor,
            self::info::ResponseType::Assess => Self::Assess,
            self::info::ResponseType::AllClear => Self::AllClear,
            self::info::ResponseType::None => Self::None,
        }
    }
}

impl From<crate::v1dot2::ResponseType> for self::info::ResponseType {
    fn from(value: crate::v1dot2::ResponseType) -> Self {
        match value {
            crate::v1dot2::ResponseType::Shelter => Self::Shelter,
            crate::v1dot2::ResponseType::Evacuate => Self::Evacuate,
            crate::v1dot2::ResponseType::Prepare => Self::Prepare,
            crate::v1dot2::ResponseType::Execute => Self::Execute,
            crate::v1dot2::ResponseType::Avoid => Self::Avoid,
            crate::v1dot2::ResponseType::Monitor => Self::Monitor,
            crate::v1dot2::ResponseType::Assess => Self::Assess,
            crate::v1dot2::ResponseType::AllClear => Self::AllClear,
            crate::v1dot2::ResponseType::None => Self::None,
        }
    }
}

impl From<self::info::Urgency> for crate::v1dot2::Urgency {
    fn from(value: self::info::Urgency) -> Self {
        match value {
            self::info::Urgency::Immediate => Self::Immediate,
            self::info::Urgency::Expected => Self::Expected,
            self::info::Urgency::Future => Self::Future,
            self::info::Urgency::Past => Self::Past,
            self::info::Urgency::UnknownUrgency => Self::Unknown,
        }
    }
}

impl From<crate::v1dot2::Urgency> for self::info::Urgency {
    fn from(value: crate::v1dot2::Urgency) -> Self {
        match value {
            crate::v1dot2::Urgency::Immediate => Self::Immediate,
            crate::v1dot2::Urgency::Expected => Self::Expected,
            crate::v1dot2::Urgency::Future => Self::Future,
            crate::v1dot2::Urgency::Past => Self::Past,
            crate::v1dot2::Urgency::Unknown => Self::UnknownUrgency,
        }
    }
}

impl From<self::info::Severity> for crate::v1dot2::Severity {
    fn from(value: self::info::Severity) -> Self {
        match value {
            self::info::Severity::Extreme => Self::Extreme,
            self::info::Severity::Severe => Self::Severe,
            self::info::Severity::Moderate => Self::Moderate,
            self::info::Severity::Minor => Self::Minor,
            self::info::Severity::UnknownSeverity => Self::Unknown,
        }
    }
}

impl From<crate::v1dot2::Severity> for self::info::Severity {
    fn from(value: crate::v1dot2::Severity) -> Self {
        match value {
            crate::v1dot2::Severity::Extreme => Self::Extreme,
            crate::v1dot2::Severity::Severe => Self::Severe,
            crate::v1dot2::Severity::Moderate => Self::Moderate,
            crate::v1dot2::Severity::Minor => Self::Minor,
            crate::v1dot2::Severity::Unknown => Self::UnknownSeverity,
        }
    }
}

impl From<self::info::Certainty> for crate::v1dot2::Certainty {
    fn from(value: self::info::Certainty) -> Self {
        match value {
            self::info::Certainty::Observed => Self::Observed,
            self::info::Certainty::VeryLikely => Self::Likely, // N.B.
            self::info::Certainty::Likely => Self::Likely,
            self::info::Certainty::Possible => Self::Possible,
            self::info::Certainty::Unlikely => Self::Unlikely,
            self::info::Certainty::UnknownCertainty => Self::Unknown,
        }
    }
}

impl From<crate::v1dot2::Certainty> for self::info::Certainty {
    fn from(value: crate::v1dot2::Certainty) -> Self {
        match value {
            crate::v1dot2::Certainty::Observed => Self::Observed,
            crate::v1dot2::Certainty::Likely => Self::Likely,
            crate::v1dot2::Certainty::Possible => Self::Possible,
            crate::v1dot2::Certainty::Unlikely => Self::Unlikely,
            crate::v1dot2::Certainty::Unknown => Self::UnknownCertainty,
        }
    }
}

/// The error returned when an `Alert` conversion would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum AlertConversionError {
    /// Identifier is invalid
    #[error("identifier is invalid: {0}")]
    Identifier(crate::id::InvalidIdError),
    /// Sender is invalid
    #[error("sender is invalid: {0}")]
    Sender(crate::id::InvalidIdError),
    /// Sent is invalid
    #[error("sent is invalid: {0}")]
    Sent(chrono::ParseError),
    /// Status is invalid
    #[error("status is invalid: {0}")]
    Status(i32),
    /// Message type is invalid
    #[error("message type is invalid: {0}")]
    MessageType(i32),
    /// Scope is invalid
    #[error("scope is invalid: {0}")]
    Scope(i32),
    /// Scope is missing
    #[error("scope is missing")]
    ScopeMissing,
    /// Addresses contains an invalid item
    #[error("addresses contains an invalid item: {0}")]
    Addresses(crate::delimited_items::InvalidItemError),
    /// References contains an invalid reference
    #[error("references contains an invalid reference: {0}")]
    References(crate::references::ReferenceError),
    /// Incidents contains an invalid item
    #[error("incidents contains an invalid item: {0}")]
    Incidents(crate::delimited_items::InvalidItemError),
    /// Info is invalid
    #[error("info is invalid: {0}")]
    Info(InfoConversionError),
}

impl TryFrom<Alert> for crate::v1dot2::Alert {
    type Error = AlertConversionError;

    fn try_from(value: Alert) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value
                .identifier
                .try_into()
                .map_err(AlertConversionError::Identifier)?,
            sender: value
                .sender
                .try_into()
                .map_err(AlertConversionError::Sender)?,
            sent: value.sent.parse().map_err(AlertConversionError::Sent)?,
            status: self::alert::Status::from_i32(value.status)
                .map(|v| v.into())
                .ok_or(AlertConversionError::Status(value.status))?,
            message_type: self::alert::MsgType::from_i32(value.msg_type)
                .map(|v| v.into())
                .ok_or(AlertConversionError::MessageType(value.msg_type))?,
            source: value.source,
            scope: {
                let scope = value.scope.ok_or(AlertConversionError::ScopeMissing)?;
                self::alert::Scope::from_i32(value.msg_type)
                    .map(|v| v.into())
                    .ok_or(AlertConversionError::Scope(scope))?
            },
            restriction: value.restriction,
            addresses: match value.addresses {
                Some(v) => Some(v.try_into().map_err(AlertConversionError::Addresses)?),
                None => None,
            },
            codes: value.code,
            note: value.note,
            references: match value.references {
                Some(v) => Some(v.try_into().map_err(AlertConversionError::References)?),
                None => None,
            },
            incidents: match value.incidents {
                Some(v) => Some(v.try_into().map_err(AlertConversionError::Incidents)?),
                None => None,
            },
            info: value
                .info
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<_, _>>()
                .map_err(AlertConversionError::Info)?,
        })
    }
}

impl From<self::alert::Status> for crate::v1dot2::Status {
    fn from(value: self::alert::Status) -> Self {
        match value {
            self::alert::Status::Actual => Self::Actual,
            self::alert::Status::Exercise => Self::Exercise,
            self::alert::Status::System => Self::System,
            self::alert::Status::Test => Self::Test,
            self::alert::Status::Draft => Self::Draft,
        }
    }
}

impl From<crate::v1dot2::Status> for self::alert::Status {
    fn from(value: crate::v1dot2::Status) -> Self {
        match value {
            crate::v1dot2::Status::Actual => Self::Actual,
            crate::v1dot2::Status::Exercise => Self::Exercise,
            crate::v1dot2::Status::System => Self::System,
            crate::v1dot2::Status::Test => Self::Test,
            crate::v1dot2::Status::Draft => Self::Draft,
        }
    }
}

impl From<self::alert::MsgType> for crate::v1dot2::MessageType {
    fn from(value: self::alert::MsgType) -> Self {
        match value {
            self::alert::MsgType::Alert => Self::Alert,
            self::alert::MsgType::Update => Self::Update,
            self::alert::MsgType::Cancel => Self::Cancel,
            self::alert::MsgType::Ack => Self::Ack,
            self::alert::MsgType::Error => Self::Error,
        }
    }
}

impl From<crate::v1dot2::MessageType> for self::alert::MsgType {
    fn from(value: crate::v1dot2::MessageType) -> Self {
        match value {
            crate::v1dot2::MessageType::Alert => Self::Alert,
            crate::v1dot2::MessageType::Update => Self::Update,
            crate::v1dot2::MessageType::Cancel => Self::Cancel,
            crate::v1dot2::MessageType::Ack => Self::Ack,
            crate::v1dot2::MessageType::Error => Self::Error,
        }
    }
}

impl From<self::alert::Scope> for crate::v1dot2::Scope {
    fn from(value: self::alert::Scope) -> Self {
        match value {
            self::alert::Scope::Public => Self::Public,
            self::alert::Scope::Restricted => Self::Restricted,
            self::alert::Scope::Private => Self::Private,
        }
    }
}

impl From<crate::v1dot2::Scope> for self::alert::Scope {
    fn from(value: crate::v1dot2::Scope) -> Self {
        match value {
            crate::v1dot2::Scope::Public => Self::Public,
            crate::v1dot2::Scope::Restricted => Self::Restricted,
            crate::v1dot2::Scope::Private => Self::Private,
        }
    }
}
