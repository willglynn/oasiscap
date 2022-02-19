impl TryFrom<super::alert::Status> for crate::v1dot0::Status {
    type Error = super::alert::Status;
    fn try_from(value: super::alert::Status) -> Result<Self, Self::Error> {
        match value {
            super::alert::Status::Actual => Ok(Self::Actual),
            super::alert::Status::Exercise => Ok(Self::Exercise),
            super::alert::Status::System => Ok(Self::System),
            super::alert::Status::Test => Ok(Self::Test),
            super::alert::Status::Draft => Err(value),
        }
    }
}

impl From<crate::v1dot0::Status> for super::alert::Status {
    fn from(value: crate::v1dot0::Status) -> Self {
        match value {
            crate::v1dot0::Status::Actual => Self::Actual,
            crate::v1dot0::Status::Exercise => Self::Exercise,
            crate::v1dot0::Status::System => Self::System,
            crate::v1dot0::Status::Test => Self::Test,
        }
    }
}

impl From<super::alert::Status> for crate::v1dot1::Status {
    fn from(value: super::alert::Status) -> Self {
        match value {
            super::alert::Status::Actual => Self::Actual,
            super::alert::Status::Exercise => Self::Exercise,
            super::alert::Status::System => Self::System,
            super::alert::Status::Test => Self::Test,
            super::alert::Status::Draft => Self::Draft,
        }
    }
}

impl From<crate::v1dot1::Status> for super::alert::Status {
    fn from(value: crate::v1dot1::Status) -> Self {
        match value {
            crate::v1dot1::Status::Actual => Self::Actual,
            crate::v1dot1::Status::Exercise => Self::Exercise,
            crate::v1dot1::Status::System => Self::System,
            crate::v1dot1::Status::Test => Self::Test,
            crate::v1dot1::Status::Draft => Self::Draft,
        }
    }
}
