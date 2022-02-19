impl TryFrom<super::info::Certainty> for crate::v1dot0::Certainty {
    type Error = super::info::Certainty;

    fn try_from(value: super::info::Certainty) -> Result<Self, Self::Error> {
        match value {
            super::info::Certainty::Observed => Err(value),
            super::info::Certainty::VeryLikely => Ok(Self::VeryLikely),
            super::info::Certainty::Likely => Ok(Self::Likely),
            super::info::Certainty::Possible => Ok(Self::Possible),
            super::info::Certainty::Unlikely => Ok(Self::Unlikely),
            super::info::Certainty::UnknownCertainty => Ok(Self::Unknown),
        }
    }
}

impl From<crate::v1dot0::Certainty> for super::info::Certainty {
    fn from(value: crate::v1dot0::Certainty) -> Self {
        match value {
            crate::v1dot0::Certainty::VeryLikely => Self::VeryLikely,
            crate::v1dot0::Certainty::Likely => Self::Likely,
            crate::v1dot0::Certainty::Possible => Self::Possible,
            crate::v1dot0::Certainty::Unlikely => Self::Unlikely,
            crate::v1dot0::Certainty::Unknown => Self::UnknownCertainty,
        }
    }
}

impl From<super::info::Certainty> for crate::v1dot1::Certainty {
    fn from(value: super::info::Certainty) -> Self {
        match value {
            super::info::Certainty::Observed => Self::Observed,
            super::info::Certainty::VeryLikely => Self::Likely, // N.B.
            super::info::Certainty::Likely => Self::Likely,
            super::info::Certainty::Possible => Self::Possible,
            super::info::Certainty::Unlikely => Self::Unlikely,
            super::info::Certainty::UnknownCertainty => Self::Unknown,
        }
    }
}

impl From<crate::v1dot1::Certainty> for super::info::Certainty {
    fn from(value: crate::v1dot1::Certainty) -> Self {
        match value {
            crate::v1dot1::Certainty::Observed => Self::Observed,
            crate::v1dot1::Certainty::Likely => Self::Likely,
            crate::v1dot1::Certainty::Possible => Self::Possible,
            crate::v1dot1::Certainty::Unlikely => Self::Unlikely,
            crate::v1dot1::Certainty::Unknown => Self::UnknownCertainty,
        }
    }
}
