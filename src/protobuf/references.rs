impl TryFrom<super::Group> for crate::references::References {
    type Error = crate::references::ReferenceError;

    fn try_from(value: super::Group) -> Result<Self, Self::Error> {
        value
            .value
            .into_iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
    }
}

impl From<crate::references::References> for super::Group {
    fn from(value: crate::references::References) -> Self {
        Self {
            value: value.into_iter().map(|r| r.to_string()).collect(),
        }
    }
}
