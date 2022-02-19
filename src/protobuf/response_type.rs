impl TryFrom<super::info::ResponseType> for crate::v1dot1::ResponseType {
    type Error = super::info::ResponseType;

    fn try_from(value: super::info::ResponseType) -> Result<Self, Self::Error> {
        match value {
            super::info::ResponseType::Shelter => Ok(Self::Shelter),
            super::info::ResponseType::Evacuate => Ok(Self::Evacuate),
            super::info::ResponseType::Prepare => Ok(Self::Prepare),
            super::info::ResponseType::Execute => Ok(Self::Execute),
            super::info::ResponseType::Avoid => Err(value),
            super::info::ResponseType::Monitor => Ok(Self::Monitor),
            super::info::ResponseType::Assess => Ok(Self::Assess),
            super::info::ResponseType::AllClear => Err(value),
            super::info::ResponseType::None => Ok(Self::None),
        }
    }
}

impl From<crate::v1dot1::ResponseType> for super::info::ResponseType {
    fn from(value: crate::v1dot1::ResponseType) -> Self {
        match value {
            crate::v1dot1::ResponseType::Shelter => Self::Shelter,
            crate::v1dot1::ResponseType::Evacuate => Self::Evacuate,
            crate::v1dot1::ResponseType::Prepare => Self::Prepare,
            crate::v1dot1::ResponseType::Execute => Self::Execute,
            crate::v1dot1::ResponseType::Monitor => Self::Monitor,
            crate::v1dot1::ResponseType::Assess => Self::Assess,
            crate::v1dot1::ResponseType::None => Self::None,
        }
    }
}

impl From<super::info::ResponseType> for crate::v1dot2::ResponseType {
    fn from(value: super::info::ResponseType) -> Self {
        match value {
            super::info::ResponseType::Shelter => Self::Shelter,
            super::info::ResponseType::Evacuate => Self::Evacuate,
            super::info::ResponseType::Prepare => Self::Prepare,
            super::info::ResponseType::Execute => Self::Execute,
            super::info::ResponseType::Avoid => Self::Avoid,
            super::info::ResponseType::Monitor => Self::Monitor,
            super::info::ResponseType::Assess => Self::Assess,
            super::info::ResponseType::AllClear => Self::AllClear,
            super::info::ResponseType::None => Self::None,
        }
    }
}

impl From<crate::v1dot2::ResponseType> for super::info::ResponseType {
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
