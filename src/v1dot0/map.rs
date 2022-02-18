//! A container for CAP 1.0 key-value maps.
use serde::de::Error;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};

/// An order-preserving `Key` => `String` key/value map which supports duplicate entries.
///
/// CAP 1.0 `Map`s are encoded into key-value strings. `Key`s are prohibited from containing
/// certain characters.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Map(Vec<(Key, String)>);

impl Map {
    /// Instantiate an empty map.
    ///
    /// # Example
    ///
    /// ```
    /// let map = oasiscap::v1dot0::Map::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the first value for this key, if any.
    ///
    /// # Example
    ///
    /// ```
    /// let map: oasiscap::v1dot0::Map = [
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
            .filter_map(|(k, v)| {
                if k.as_str() == value_name {
                    Some(v.as_str())
                } else {
                    None
                }
            })
            .next()
    }

    /// Iterate over all the values for a given key.
    ///
    /// # Example
    ///
    /// ```
    /// let map: oasiscap::v1dot0::Map = [
    ///     ("foo", "bar"),
    ///     ("foo", "baz"),
    ///     ("quxx", "flummox"),
    /// ].into_iter().collect();
    ///
    /// assert_eq!(map.get_all("foo").collect::<Vec<&str>>(), vec!["bar", "baz"]);
    /// ```
    pub fn get_all<S: AsRef<str>>(&self, value_name: S) -> impl Iterator<Item = &str> {
        self.0.iter().filter_map(move |(k, v)| {
            if k.as_str() == value_name.as_ref() {
                Some(v.as_str())
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
    /// let mut map: oasiscap::v1dot0::Map = [
    ///     ("foo", "bar"),
    ///     ("foo", "baz"),
    ///     ("quxx", "flummox"),
    /// ].into_iter().collect();
    ///
    /// map.push("foo", "waldo");
    ///
    /// assert_eq!(map.get_all("foo").collect::<Vec<&str>>(), vec!["bar", "baz", "waldo"]);
    /// ```
    pub fn push<K: Into<Key>, V: Into<String>>(&mut self, value_name: K, value: V) {
        self.0.push((value_name.into(), value.into()));
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
    pub fn iter(&self) -> Iter {
        Iter(self.0.iter())
    }
}

impl Deref for Map {
    type Target = [(Key, String)];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}
impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for (key, value) in &self.0 {
            seq.serialize_element(&format!("{}={}", key, value))?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Map {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let entries = <Option<Vec<String>>>::deserialize(deserializer)?;

        let vec = entries
            .unwrap_or_default()
            .into_iter()
            .map(|mut string: String| {
                if let Some(eq_index) = string.find('=') {
                    let value = string.split_off(eq_index + 1);
                    string.truncate(eq_index);
                    Key::try_from(string)
                        .map(|key| (key, value))
                        .map_err(D::Error::custom)
                } else {
                    Err(D::Error::custom("invalid map entry: missing \"=\""))
                }
            })
            .collect::<Result<Vec<(Key, String)>, D::Error>>()?;

        Ok(Self(vec))
    }
}

/// A map key
///
/// Map keys are `String`s which cannot contain particular characters.
///
/// ```rust
/// use oasiscap::v1dot0::map::Key;
///
/// assert_eq!("fips".parse::<Key>(), Ok(Key::from_static("fips")));
///
/// assert!("no spaces".parse::<Key>().is_err());
/// assert!("no,commas".parse::<Key>().is_err());
/// assert!("no<XML>like&chars;".parse::<Key>().is_err());
/// assert!("no=equals".parse::<Key>().is_err());
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(String);

impl Key {
    /// Returns the key as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Instantiate a `Key` from a static string.
    ///
    /// # Panics
    ///
    /// Panics if the provided string is not a valid `Key`.
    ///
    /// This is intended as a convenience for values known at compile time. Values determined at
    /// runtime should use `TryFrom<String>` or `FromStr` instead.
    pub fn from_static(value: &'static str) -> Self {
        Self::try_from(String::from(value)).expect("static string must be a valid Key")
    }
}
impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq<&str> for Key {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl PartialEq<Key> for &str {
    fn eq(&self, other: &Key) -> bool {
        *self == other.as_str()
    }
}

impl TryFrom<String> for Key {
    type Error = InvalidKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Designators may not include spaces or XML-restricted characters (<, >, &, ',").
        if value.contains(' ')
            || value.contains('<')
            || value.contains('>')
            || value.contains('&')
            || value.contains(',')
            || value.contains('=')
        {
            Err(InvalidKeyError(value))
        } else {
            Ok(Key(value))
        }
    }
}

impl From<&'static str> for Key {
    fn from(key: &'static str) -> Self {
        Key::from_static(key)
    }
}

impl std::str::FromStr for Key {
    type Err = InvalidKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        String::from(s).try_into()
    }
}

impl From<Key> for String {
    fn from(key: Key) -> Self {
        key.0
    }
}

/// The error returned when a map `Key` contains a prohibited character.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvalidKeyError(String);

impl std::fmt::Display for InvalidKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid map key: {:?}", self.0)
    }
}
impl std::error::Error for InvalidKeyError {}

impl FromIterator<(Key, String)> for Map {
    fn from_iter<T: IntoIterator<Item = (Key, String)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<(Key, &'a str)> for Map {
    fn from_iter<T: IntoIterator<Item = (Key, &'a str)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(k, v)| (k, String::from(v)))
                .collect(),
        )
    }
}

impl<'a> FromIterator<(&'static str, &'a str)> for Map {
    fn from_iter<T: IntoIterator<Item = (&'static str, &'a str)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(k, v)| (Key::from_static(k), String::from(v)))
                .collect(),
        )
    }
}

/// An iterator over a map.
pub struct Iter<'a>(std::slice::Iter<'a, (Key, String)>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a Key, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (k, v.as_str()))
    }
}

/// A mutable iterator over a map.
pub struct IterMut<'a>(std::slice::IterMut<'a, (Key, String)>);

impl<'a> Iterator for IterMut<'a> {
    type Item = (&'a mut Key, &'a mut String);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (k, v))
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = (&'a Key, &'a str);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for Map {
    type Item = (Key, String);
    type IntoIter = std::vec::IntoIter<(Key, String)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        #[derive(Deserialize)]
        struct Doc {
            #[serde(rename = "{http://www.incident.com/cap/1.0}:cap:area")]
            area: Area,
        }

        #[derive(Deserialize)]
        struct Area {
            #[serde(rename = "{http://www.incident.com/cap/1.0}cap:geocode")]
            pub geocode: Map,
        }

        let doc: Doc = xml_serde::from_str(
            r#"
        <cap:area xmlns:cap="http://www.incident.com/cap/1.0">
            <cap:geocode>fips6=006109</cap:geocode>
            <cap:geocode>fips6=006109</cap:geocode>
            <cap:geocode>fips6=006103</cap:geocode>
        </cap:area>
        "#,
        )
        .expect("parse");
        assert_eq!(
            doc.area
                .geocode
                .0
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("fips6", "006109"),
                ("fips6", "006109"),
                ("fips6", "006103"),
            ]
        );
    }
}
