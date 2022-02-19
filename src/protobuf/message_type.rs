impl From<super::alert::MsgType> for crate::v1dot2::MessageType {
    fn from(value: super::alert::MsgType) -> Self {
        match value {
            super::alert::MsgType::Alert => Self::Alert,
            super::alert::MsgType::Update => Self::Update,
            super::alert::MsgType::Cancel => Self::Cancel,
            super::alert::MsgType::Ack => Self::Ack,
            super::alert::MsgType::Error => Self::Error,
        }
    }
}

impl From<crate::v1dot2::MessageType> for super::alert::MsgType {
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
