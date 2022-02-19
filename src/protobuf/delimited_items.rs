impl From<crate::delimited_items::Items> for super::Group {
    fn from(value: crate::delimited_items::Items) -> Self {
        Self {
            value: value.into_iter().map(String::from).collect(),
        }
    }
}

impl TryFrom<super::Group> for crate::delimited_items::Items {
    type Error = crate::delimited_items::InvalidItemError;

    fn try_from(value: super::Group) -> Result<Self, Self::Error> {
        value
            .value
            .into_iter()
            .map(crate::delimited_items::Item::try_from)
            .collect()
    }
}
