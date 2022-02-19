use super::ValuePair;

impl<E: crate::map::Entry> From<Vec<ValuePair>> for crate::map::Map<E> {
    fn from(value: Vec<ValuePair>) -> Self {
        value
            .into_iter()
            .map(|pair| (pair.value_name, pair.value))
            .collect()
    }
}

impl<E: crate::map::Entry> From<crate::map::Map<E>> for Vec<ValuePair> {
    fn from(value: crate::map::Map<E>) -> Self {
        value
            .into_iter()
            .map(|(value_name, value)| ValuePair { value_name, value })
            .collect()
    }
}

impl TryFrom<Vec<ValuePair>> for crate::v1dot0::Map {
    type Error = crate::v1dot0::map::InvalidKeyError;

    fn try_from(value: Vec<ValuePair>) -> Result<Self, Self::Error> {
        value
            .into_iter()
            .map(|pair| {
                crate::v1dot0::map::Key::try_from(pair.value_name).map(|key| (key, pair.value))
            })
            .collect()
    }
}

impl From<crate::v1dot0::Map> for Vec<ValuePair> {
    fn from(value: crate::v1dot0::Map) -> Self {
        value
            .into_iter()
            .map(|(value_name, value)| ValuePair {
                value_name: value_name.into(),
                value,
            })
            .collect()
    }
}
