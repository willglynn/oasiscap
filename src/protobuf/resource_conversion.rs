use super::Resource;

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
    /// Deref URI is present
    #[error("deref URI is present")]
    DerefUriPresent,
    /// Digest is invalid
    #[error("digest is invalid: {0}")]
    Digest(#[from] crate::digest::Sha1ParseError),
}

impl TryFrom<Resource> for crate::v1dot0::Resource {
    type Error = ResourceConversionError;

    fn try_from(value: Resource) -> Result<Self, Self::Error> {
        if value.deref_uri.is_some() {
            // CAP v1.0 doesn't have this field
            return Err(ResourceConversionError::DerefUriPresent);
        }

        Ok(Self {
            description: value.resource_desc,
            mime_type: value.mime_type,
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
            digest: match value.digest {
                Some(string) => Some(string.parse()?),
                None => None,
            },
        })
    }
}

impl From<crate::v1dot0::Resource> for Resource {
    fn from(value: crate::v1dot0::Resource) -> Self {
        Self {
            resource_desc: value.description,
            mime_type: value.mime_type,
            size: value.size.and_then(|v| v.try_into().ok()),
            uri: value.uri.map(|v| v.to_string()),
            deref_uri: None,
            digest: value.digest.as_ref().map(crate::digest::Sha1::to_string),
        }
    }
}

impl TryFrom<Resource> for crate::v1dot1::Resource {
    type Error = ResourceConversionError;

    fn try_from(value: Resource) -> Result<Self, Self::Error> {
        Ok(Self {
            description: value.resource_desc,
            mime_type: value.mime_type,
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

impl From<crate::v1dot1::Resource> for Resource {
    fn from(value: crate::v1dot1::Resource) -> Self {
        Self {
            resource_desc: value.description,
            mime_type: value.mime_type,
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
