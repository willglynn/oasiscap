impl From<super::info::Urgency> for crate::v1dot0::Urgency {
    fn from(value: super::info::Urgency) -> Self {
        match value {
            super::info::Urgency::Immediate => Self::Immediate,
            super::info::Urgency::Expected => Self::Expected,
            super::info::Urgency::Future => Self::Future,
            super::info::Urgency::Past => Self::Past,
            super::info::Urgency::UnknownUrgency => Self::Unknown,
        }
    }
}

impl From<crate::v1dot0::Urgency> for super::info::Urgency {
    fn from(value: crate::v1dot0::Urgency) -> Self {
        match value {
            crate::v1dot0::Urgency::Immediate => Self::Immediate,
            crate::v1dot0::Urgency::Expected => Self::Expected,
            crate::v1dot0::Urgency::Future => Self::Future,
            crate::v1dot0::Urgency::Past => Self::Past,
            crate::v1dot0::Urgency::Unknown => Self::UnknownUrgency,
        }
    }
}
