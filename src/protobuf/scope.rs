impl From<super::alert::Scope> for crate::v1dot2::Scope {
    fn from(value: super::alert::Scope) -> Self {
        match value {
            super::alert::Scope::Public => Self::Public,
            super::alert::Scope::Restricted => Self::Restricted,
            super::alert::Scope::Private => Self::Private,
        }
    }
}

impl From<crate::v1dot2::Scope> for super::alert::Scope {
    fn from(value: crate::v1dot2::Scope) -> Self {
        match value {
            crate::v1dot2::Scope::Public => Self::Public,
            crate::v1dot2::Scope::Restricted => Self::Restricted,
            crate::v1dot2::Scope::Private => Self::Private,
        }
    }
}
