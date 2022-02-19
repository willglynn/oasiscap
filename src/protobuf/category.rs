impl From<crate::v1dot0::Category> for super::info::Category {
    fn from(value: crate::v1dot0::Category) -> Self {
        match value {
            crate::v1dot0::Category::Geo => Self::Geo,
            crate::v1dot0::Category::Met => Self::Met,
            crate::v1dot0::Category::Safety => Self::Safety,
            crate::v1dot0::Category::Security => Self::Security,
            crate::v1dot0::Category::Rescue => Self::Rescue,
            crate::v1dot0::Category::Fire => Self::Fire,
            crate::v1dot0::Category::Health => Self::Health,
            crate::v1dot0::Category::Env => Self::Env,
            crate::v1dot0::Category::Transport => Self::Transport,
            crate::v1dot0::Category::Infra => Self::Infra,
            crate::v1dot0::Category::Other => Self::Other,
        }
    }
}

impl TryFrom<super::info::Category> for crate::v1dot0::Category {
    type Error = super::info::Category;

    fn try_from(value: super::info::Category) -> Result<Self, Self::Error> {
        match value {
            super::info::Category::Geo => Ok(Self::Geo),
            super::info::Category::Met => Ok(Self::Met),
            super::info::Category::Safety => Ok(Self::Safety),
            super::info::Category::Security => Ok(Self::Security),
            super::info::Category::Rescue => Ok(Self::Rescue),
            super::info::Category::Fire => Ok(Self::Fire),
            super::info::Category::Health => Ok(Self::Health),
            super::info::Category::Env => Ok(Self::Env),
            super::info::Category::Transport => Ok(Self::Transport),
            super::info::Category::Infra => Ok(Self::Infra),
            super::info::Category::Cbrne => Err(value),
            super::info::Category::Other => Ok(Self::Other),
        }
    }
}

impl From<crate::v1dot1::Category> for super::info::Category {
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

impl From<super::info::Category> for crate::v1dot1::Category {
    fn from(value: super::info::Category) -> Self {
        match value {
            super::info::Category::Geo => Self::Geo,
            super::info::Category::Met => Self::Met,
            super::info::Category::Safety => Self::Safety,
            super::info::Category::Security => Self::Security,
            super::info::Category::Rescue => Self::Rescue,
            super::info::Category::Fire => Self::Fire,
            super::info::Category::Health => Self::Health,
            super::info::Category::Env => Self::Env,
            super::info::Category::Transport => Self::Transport,
            super::info::Category::Infra => Self::Infra,
            super::info::Category::Cbrne => Self::CBRNE,
            super::info::Category::Other => Self::Other,
        }
    }
}
