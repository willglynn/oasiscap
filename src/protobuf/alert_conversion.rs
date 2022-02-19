use super::alert::{MsgType, Scope, Status};
use crate::delimited_items::InvalidItemError;
use crate::id::InvalidIdError;
use crate::protobuf::{Alert, InfoConversionError};
use crate::references::ReferenceError;

/// The error returned when an `Alert` conversion would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum AlertConversionError {
    /// xmlns is invalid
    #[error("xmlns is invalid: {0:?}")]
    Xmlns(String),
    /// Identifier is invalid
    #[error("identifier is invalid: {0}")]
    Identifier(InvalidIdError),
    /// Sender is invalid
    #[error("sender is invalid: {0}")]
    Sender(InvalidIdError),
    /// Sent is invalid
    #[error("sent is invalid: {0}")]
    Sent(chrono::ParseError),
    /// Status is invalid
    #[error("status is invalid: {0}")]
    Status(i32),
    /// Unrepresentable status
    #[error("unrepresentable status: {0:?}")]
    UnrepresentableStatus(super::alert::Status),
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
    Addresses(InvalidItemError),
    /// References contains an invalid reference
    #[error("references contains an invalid reference: {0}")]
    References(ReferenceError),
    /// Incidents contains an invalid item
    #[error("incidents contains an invalid item: {0}")]
    Incidents(InvalidItemError),
    /// Info is invalid
    #[error("info is invalid: {0}")]
    Info(InfoConversionError),
}

impl TryFrom<Alert> for crate::v1dot0::Alert {
    type Error = AlertConversionError;

    fn try_from(value: Alert) -> Result<Self, Self::Error> {
        // Needed only for `password:`, but https://github.com/rust-lang/rust/issues/60681
        #[allow(deprecated)]
        Ok(Self {
            identifier: value
                .identifier
                .try_into()
                .map_err(AlertConversionError::Identifier)?,
            sender: value
                .sender
                .try_into()
                .map_err(AlertConversionError::Sender)?,
            password: value.password,
            sent: value.sent.parse().map_err(AlertConversionError::Sent)?,
            status: Status::from_i32(value.status)
                .ok_or(AlertConversionError::Status(value.status))
                .and_then(|v| {
                    v.try_into()
                        .map_err(AlertConversionError::UnrepresentableStatus)
                })?,
            message_type: MsgType::from_i32(value.msg_type)
                .map(|v| v.into())
                .ok_or(AlertConversionError::MessageType(value.msg_type))?,
            source: value.source,
            scope: {
                let scope = value.scope.ok_or(AlertConversionError::ScopeMissing)?;
                Scope::from_i32(scope)
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

impl From<crate::v1dot0::Alert> for Alert {
    fn from(value: crate::v1dot0::Alert) -> Self {
        // Needed only for `password:`, but https://github.com/rust-lang/rust/issues/60681
        #[allow(deprecated)]
        Self {
            xmlns: "http://www.incident.com/cap/1.0".into(),
            identifier: value.identifier.into(),
            sender: value.sender.into(),
            password: value.password,
            sent: value.sent.to_string(),
            status: Status::from(value.status) as i32,
            msg_type: MsgType::from(value.message_type) as i32,
            source: value.source,
            scope: Some(Scope::from(value.scope) as i32),
            restriction: value.restriction,
            addresses: value.addresses.map(|v| v.into()),
            code: value.codes,
            note: value.note,
            references: value.references.map(|v| v.into()),
            incidents: value.incidents.map(|v| v.into()),
            info: value.info.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl TryFrom<Alert> for crate::v1dot1::Alert {
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
            status: Status::from_i32(value.status)
                .map(|v| v.into())
                .ok_or(AlertConversionError::Status(value.status))?,
            message_type: MsgType::from_i32(value.msg_type)
                .map(|v| v.into())
                .ok_or(AlertConversionError::MessageType(value.msg_type))?,
            source: value.source,
            scope: {
                let scope = value.scope.ok_or(AlertConversionError::ScopeMissing)?;
                Scope::from_i32(scope)
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

impl From<crate::v1dot1::Alert> for Alert {
    fn from(value: crate::v1dot1::Alert) -> Self {
        // Needed only for `password:`, but https://github.com/rust-lang/rust/issues/60681
        #[allow(deprecated)]
        Self {
            xmlns: "urn:oasis:names:tc:emergency:cap:1.1".into(),
            identifier: value.identifier.into(),
            sender: value.sender.into(),
            password: None,
            sent: value.sent.to_string(),
            status: Status::from(value.status) as i32,
            msg_type: MsgType::from(value.message_type) as i32,
            source: value.source,
            scope: Some(Scope::from(value.scope) as i32),
            restriction: value.restriction,
            addresses: value.addresses.map(|v| v.into()),
            code: value.codes,
            note: value.note,
            references: value.references.map(|v| v.into()),
            incidents: value.incidents.map(|v| v.into()),
            info: value.info.into_iter().map(|v| v.into()).collect(),
        }
    }
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
            status: Status::from_i32(value.status)
                .map(|v| v.into())
                .ok_or(AlertConversionError::Status(value.status))?,
            message_type: MsgType::from_i32(value.msg_type)
                .map(|v| v.into())
                .ok_or(AlertConversionError::MessageType(value.msg_type))?,
            source: value.source,
            scope: {
                let scope = value.scope.ok_or(AlertConversionError::ScopeMissing)?;
                Scope::from_i32(scope)
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

impl From<crate::v1dot2::Alert> for Alert {
    fn from(value: crate::v1dot2::Alert) -> Self {
        // Needed only for `password:`, but https://github.com/rust-lang/rust/issues/60681
        #[allow(deprecated)]
        Self {
            xmlns: "urn:oasis:names:tc:emergency:cap:1.2".into(),
            identifier: value.identifier.into(),
            sender: value.sender.into(),
            password: None,
            sent: value.sent.to_string(),
            status: Status::from(value.status) as i32,
            msg_type: MsgType::from(value.message_type) as i32,
            source: value.source,
            scope: Some(Scope::from(value.scope) as i32),
            restriction: value.restriction,
            addresses: value.addresses.map(|v| v.into()),
            code: value.codes,
            note: value.note,
            references: value.references.map(|v| v.into()),
            incidents: value.incidents.map(|v| v.into()),
            info: value.info.into_iter().map(|v| v.into()).collect(),
        }
    }
}
