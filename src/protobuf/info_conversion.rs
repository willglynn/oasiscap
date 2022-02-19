use super::*;

/// The error returned when an `Info` conversion would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum InfoConversionError {
    /// Language is invalid
    #[error("language is invalid: {0}")]
    Language(#[from] crate::language::InvalidLanguageError),
    /// Category is invalid
    #[error("category is invalid: {0}")]
    Category(i32),
    /// Category cannot be represented
    #[error("unrepresentable category: {0:?}")]
    UnrepresentableCategory(super::info::Category),
    /// Response type is invalid
    #[error("response type is invalid: {0}")]
    ResponseType(i32),
    /// Response type cannot be represented
    #[error("unrepresentable response type: {0:?}")]
    UnrepresentableResponseType(super::info::ResponseType),
    /// Urgency is invalid
    #[error("urgency is invalid: {0}")]
    Urgency(i32),
    /// Severity is invalid
    #[error("severity is invalid: {0}")]
    Severity(i32),
    /// Certainty is invalid
    #[error("certainty is invalid: {0}")]
    Certainty(i32),
    /// Certainty cannot be represented
    #[error("unrepresentable certainty: {0:?}")]
    UnrepresentableCertainty(super::info::Certainty),
    /// Event code key is invalid
    #[error("invalid event code key: {0}")]
    EventCode(crate::v1dot0::map::InvalidKeyError),
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
    /// Parameters key is invalid
    #[error("parameters key is invalid: {0}")]
    Parameters(crate::v1dot0::map::InvalidKeyError),
    /// Resource is invalid
    #[error("resource is invalid: {0}")]
    Resource(#[from] ResourceConversionError),
    /// Area is invalid
    #[error("area is invalid: {0}")]
    Area(#[from] AreaConversionError),
}

impl TryFrom<Info> for crate::v1dot0::Info {
    type Error = InfoConversionError;

    fn try_from(value: Info) -> Result<Self, Self::Error> {
        // CAP v1.0 has no response type
        if let Some(v) = value.response_type.first().cloned() {
            self::info::ResponseType::from_i32(v)
                .ok_or(InfoConversionError::ResponseType(v))
                .and_then(|v| Err(InfoConversionError::UnrepresentableResponseType(v)))?;
        }

        Ok(Self {
            language: crate::language::Language::new(value.language)?,
            categories: value
                .category
                .into_iter()
                .map(|v| {
                    self::info::Category::from_i32(v)
                        .ok_or(InfoConversionError::Category(v))
                        .and_then(|v| {
                            v.try_into()
                                .map_err(InfoConversionError::UnrepresentableCategory)
                        })
                })
                .collect::<Result<_, _>>()?,
            event: value.event,

            urgency: super::info::Urgency::from_i32(value.urgency)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Urgency(value.urgency))?,
            severity: super::info::Severity::from_i32(value.severity)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Severity(value.severity))?,
            certainty: super::info::Certainty::from_i32(value.certainty)
                .ok_or(InfoConversionError::Certainty(value.certainty))
                .and_then(|v| {
                    v.try_into()
                        .map_err(InfoConversionError::UnrepresentableCertainty)
                })?,
            audience: value.audience,
            event_codes: value
                .event_code
                .try_into()
                .map_err(InfoConversionError::EventCode)?,
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
            parameters: value
                .parameter
                .try_into()
                .map_err(InfoConversionError::Parameters)?,
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

impl From<crate::v1dot0::Info> for Info {
    fn from(value: crate::v1dot0::Info) -> Self {
        Self {
            language: value.language.into_inner(),
            category: value
                .categories
                .into_iter()
                .map(|v| super::info::Category::from(v) as i32)
                .collect(),
            event: value.event,
            response_type: vec![],
            urgency: super::info::Urgency::from(value.urgency) as i32,
            severity: super::info::Severity::from(value.severity) as i32,
            certainty: super::info::Certainty::from(value.certainty) as i32,
            audience: value.audience,
            event_code: value.event_codes.into(),
            effective: value.effective.map(|v| v.to_string()),
            onset: value.onset.map(|v| v.to_string()),
            expires: value.expires.map(|v| v.to_string()),
            sender_name: value.sender_name,
            headline: value.headline,
            description: value.description,
            instruction: value.instruction,
            web: value.web.map(|v| v.to_string()),
            contact: value.contact,
            parameter: value.parameters.into(),
            resource: value.resources.into_iter().map(|v| v.into()).collect(),
            area: value.areas.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl TryFrom<Info> for crate::v1dot1::Info {
    type Error = InfoConversionError;

    fn try_from(value: Info) -> Result<Self, Self::Error> {
        Ok(Self {
            language: crate::language::Language::new(value.language)?,
            categories: value
                .category
                .into_iter()
                .map(|v| {
                    super::info::Category::from_i32(v)
                        .ok_or(InfoConversionError::Category(v))
                        .map(|v| v.into())
                })
                .collect::<Result<_, _>>()?,
            event: value.event,
            response_type: value
                .response_type
                .into_iter()
                .map(|v| {
                    self::info::ResponseType::from_i32(v)
                        .ok_or(InfoConversionError::ResponseType(v))
                        .and_then(|v| {
                            v.try_into()
                                .map_err(InfoConversionError::UnrepresentableResponseType)
                        })
                })
                .collect::<Result<_, _>>()?,
            urgency: super::info::Urgency::from_i32(value.urgency)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Urgency(value.urgency))?,
            severity: super::info::Severity::from_i32(value.severity)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Severity(value.severity))?,
            certainty: super::info::Certainty::from_i32(value.certainty)
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

impl From<crate::v1dot1::Info> for Info {
    fn from(value: crate::v1dot1::Info) -> Self {
        Self {
            language: value.language.into_inner(),
            category: value
                .categories
                .into_iter()
                .map(|v| super::info::Category::from(v) as i32)
                .collect(),
            event: value.event,
            response_type: value
                .response_type
                .into_iter()
                .map(|v| super::info::ResponseType::from(v) as i32)
                .collect(),
            urgency: super::info::Urgency::from(value.urgency) as i32,
            severity: super::info::Severity::from(value.severity) as i32,
            certainty: super::info::Certainty::from(value.certainty) as i32,
            audience: value.audience,
            event_code: value.event_codes.into(),
            effective: value.effective.map(|v| v.to_string()),
            onset: value.onset.map(|v| v.to_string()),
            expires: value.expires.map(|v| v.to_string()),
            sender_name: value.sender_name,
            headline: value.headline,
            description: value.description,
            instruction: value.instruction,
            web: value.web.map(|v| v.to_string()),
            contact: value.contact,
            parameter: value.parameters.into(),
            resource: value.resources.into_iter().map(|v| v.into()).collect(),
            area: value.areas.into_iter().map(|v| v.into()).collect(),
        }
    }
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
                    super::info::Category::from_i32(v)
                        .map(|v| v.into())
                        .ok_or(InfoConversionError::Category(v))
                })
                .collect::<Result<_, _>>()?,
            event: value.event,
            response_type: value
                .response_type
                .into_iter()
                .map(|v| {
                    super::info::ResponseType::from_i32(v)
                        .map(|v| v.into())
                        .ok_or(InfoConversionError::ResponseType(v))
                })
                .collect::<Result<_, _>>()?,
            urgency: super::info::Urgency::from_i32(value.urgency)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Urgency(value.urgency))?,
            severity: super::info::Severity::from_i32(value.severity)
                .map(|v| v.into())
                .ok_or(InfoConversionError::Severity(value.severity))?,
            certainty: super::info::Certainty::from_i32(value.certainty)
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

impl From<crate::v1dot2::Info> for Info {
    fn from(value: crate::v1dot2::Info) -> Self {
        Self {
            language: value.language.into_inner(),
            category: value
                .categories
                .into_iter()
                .map(|v| super::info::Category::from(v) as i32)
                .collect(),
            event: value.event,
            response_type: value
                .response_type
                .into_iter()
                .map(|v| super::info::ResponseType::from(v) as i32)
                .collect(),
            urgency: super::info::Urgency::from(value.urgency) as i32,
            severity: super::info::Severity::from(value.severity) as i32,
            certainty: super::info::Certainty::from(value.certainty) as i32,
            audience: value.audience,
            event_code: value.event_codes.into(),
            effective: value.effective.map(|v| v.to_string()),
            onset: value.onset.map(|v| v.to_string()),
            expires: value.expires.map(|v| v.to_string()),
            sender_name: value.sender_name,
            headline: value.headline,
            description: value.description,
            instruction: value.instruction,
            web: value.web.map(|v| v.to_string()),
            contact: value.contact,
            parameter: value.parameters.into(),
            resource: value.resources.into_iter().map(|v| v.into()).collect(),
            area: value.areas.into_iter().map(|v| v.into()).collect(),
        }
    }
}
