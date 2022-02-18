//! A container type for CAP >=1.1 key value maps.

use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An order-preserving `String` => `String` key/value map which supports duplicate entries.
///
/// `Map` is parameterized by its element type, since different CAP standards use different
/// kinds of elements which must be serialized/deserialized differently.
///
/// `Map<E: MapEntry>` implements `FromIterator` for key-value `(String, String)` or `(&str, &str)`
/// tuples:
///
/// ```
/// let map: oasiscap::v1dot1::Map = [
///     ("foo", "bar"),
///     ("foo", "baz"),
///     ("quxx", "flummox"),
/// ].into_iter().collect();
/// ```
///
/// `&Map<E: MapEntry>` implements `IntoIterator` for easy iteration:
///
/// ```
/// # let map: oasiscap::v1dot1::Map = [
/// #     ("foo", "bar"),
/// #     ("foo", "baz"),
/// #     ("quxx", "flummox"),
/// # ].into_iter().collect();
/// for (key, value) in &map {
///     println!("{:?} = {:?}", key, value);
/// }
/// ```
///
/// `Map<E: MapEntry>` implements `IntoIterator<Item=(String, String)>`, which allows easy
/// conversion from `Map` to other kinds of containers:
///
/// ```
/// # let map: oasiscap::v1dot1::Map = [
/// #     ("foo", "bar"),
/// #     ("foo", "baz"),
/// #     ("quxx", "flummox"),
/// # ].into_iter().collect();
/// // Just note that Map is duplicate-preserving:
/// assert_eq!(map.get("foo"), Some("bar"));
/// assert_eq!(map.get_all("foo").collect::<Vec<_>>(), vec!["bar", "baz"]);
///
/// // ...while most other containers are not:
/// let btree_map: std::collections::BTreeMap<String, String> = map.into_iter().collect();
/// assert_eq!(btree_map.get("foo"), Some(&"baz".into()));
/// ```
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Map<E>(Vec<E>);

/// The behaviors needed for a map entry.
pub trait Entry: From<(String, String)> + Into<(String, String)> {
    /// The value name (i.e. key) of this entry.
    fn value_name(&self) -> &str;

    /// The value of this entry.
    fn value(&self) -> &str;

    /// Set the value of this entry, returning the old value.
    fn set_value(&mut self, new_value: String) -> String;
}

impl<E: Entry> Map<E> {
    /// Instantiate an empty map.
    ///
    /// # Example
    ///
    /// ```
    /// let map = oasiscap::v1dot1::Map::new();
    /// ```
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Get the first value for this key, if any.
    ///
    /// # Example
    ///
    /// ```
    /// let map: oasiscap::v1dot1::Map = [
    ///     ("foo", "bar"),
    ///     ("foo", "baz"),
    ///     ("quxx", "flummox"),
    /// ].into_iter().collect();
    ///
    /// assert_eq!(map.get("foo"), Some("bar"));
    /// ```
    pub fn get<S: AsRef<str>>(&self, value_name: S) -> Option<&str> {
        let value_name = value_name.as_ref();
        self.0
            .iter()
            .find(|e| e.value_name() == value_name)
            .map(|e| e.value())
    }

    /// Iterate over all the values for a given key.
    ///
    /// # Example
    ///
    /// ```
    /// let map: oasiscap::v1dot1::Map = [
    ///     ("foo", "bar"),
    ///     ("foo", "baz"),
    ///     ("quxx", "flummox"),
    /// ].into_iter().collect();
    ///
    /// assert_eq!(map.get_all("foo").collect::<Vec<&str>>(), vec!["bar", "baz"]);
    /// ```
    pub fn get_all<S: AsRef<str>>(&self, value_name: S) -> impl Iterator<Item = &str> {
        self.0.iter().filter_map(move |e| {
            if e.value_name() == value_name.as_ref() {
                Some(e.value())
            } else {
                None
            }
        })
    }

    /// Push a new key-value entry onto an existing map.
    ///
    /// # Example
    ///
    /// ```
    /// let mut map: oasiscap::v1dot1::Map = [
    ///     ("foo", "bar"),
    ///     ("foo", "baz"),
    ///     ("quxx", "flummox"),
    /// ].into_iter().collect();
    ///
    /// map.push("foo", "waldo");
    ///
    /// assert_eq!(map.get_all("foo").collect::<Vec<&str>>(), vec!["bar", "baz", "waldo"]);
    /// ```
    pub fn push<K: Into<String>, V: Into<String>>(&mut self, value_name: K, value: V) {
        self.0.push(E::from((value_name.into(), value.into())));
    }

    /// Returns the number of entries in the map.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the map contains no entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over the map.
    #[must_use]
    pub fn iter(&self) -> Iter<E> {
        Iter(self.0.iter())
    }
}

impl<E> Default for Map<E> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<E: Entry + Serialize> Serialize for Map<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for entry in &self.0 {
            seq.serialize_element(entry)?;
        }
        seq.end()
    }
}

impl<'de, E: Entry + Deserialize<'de>> Deserialize<'de> for Map<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries = <Vec<E>>::deserialize(deserializer)?;
        Ok(Self(entries))
    }
}

impl<'a, E: Entry> FromIterator<(&'a str, &'a str)> for Map<E> {
    fn from_iter<T: IntoIterator<Item = (&'a str, &'a str)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(value_name, value)| E::from((value_name.into(), value.into())))
                .collect(),
        )
    }
}

impl<E: Entry> FromIterator<(String, String)> for Map<E> {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(value_name, value)| E::from((value_name, value)))
                .collect(),
        )
    }
}

impl<E> FromIterator<E> for Map<E> {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a, E: Entry> IntoIterator for &'a Map<E> {
    type Item = (&'a str, &'a str);
    type IntoIter = Iter<'a, E>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<E: Entry> IntoIterator for Map<E> {
    type Item = (String, String);
    type IntoIter = IntoIter<E>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

/// An iterator over a map.
#[derive(Debug)]
pub struct Iter<'a, E>(std::slice::Iter<'a, E>);

impl<'a, E: Entry> Iterator for Iter<'a, E> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|e| (e.value_name(), e.value()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, E: Entry> ExactSizeIterator for Iter<'a, E> {}

/// An iterator that moves out of a map.
#[derive(Debug)]
pub struct IntoIter<E>(std::vec::IntoIter<E>);

impl<E: Entry> Iterator for IntoIter<E> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|e| e.into())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<E: Entry> ExactSizeIterator for IntoIter<E> {}

impl<E: Entry> From<crate::v1dot0::Map> for Map<E> {
    fn from(prev: crate::v1dot0::Map) -> Self {
        prev.into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect()
    }
}
