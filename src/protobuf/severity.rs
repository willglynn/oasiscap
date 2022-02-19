impl From<super::info::Severity> for crate::v1dot0::Severity {
    fn from(value: super::info::Severity) -> Self {
        match value {
            super::info::Severity::Extreme => Self::Extreme,
            super::info::Severity::Severe => Self::Severe,
            super::info::Severity::Moderate => Self::Moderate,
            super::info::Severity::Minor => Self::Minor,
            super::info::Severity::UnknownSeverity => Self::Unknown,
        }
    }
}

impl From<crate::v1dot0::Severity> for super::info::Severity {
    fn from(value: crate::v1dot0::Severity) -> Self {
        match value {
            crate::v1dot0::Severity::Extreme => Self::Extreme,
            crate::v1dot0::Severity::Severe => Self::Severe,
            crate::v1dot0::Severity::Moderate => Self::Moderate,
            crate::v1dot0::Severity::Minor => Self::Minor,
            crate::v1dot0::Severity::Unknown => Self::UnknownSeverity,
        }
    }
}
