use super::{DateTime, Id};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

/// A list of references to other alerts.
///
/// # Example
///
/// ```
/// # use oasiscap::v1dot0::References;
/// // References converts directly to/from Vec<References>
/// let references = References::from(vec![]);
/// assert_eq!(references.len(), 0);
/// assert_eq!(references.into_inner(), vec![]);
///
/// // References implements FromIterator<Reference> and IntoIterator<Reference>
/// let references: References = [].into_iter().collect();
/// for reference in references {
/// }
///
/// // References implements `FromStr`
/// let references: References = r#"
///     wcatwc@noaa.gov,PAAQ-1-mg5a94,2013-01-05T09:01:16-00:00
///     wcatwc@noaa.gov,PAAQ-2-mg5a94,2013-01-05T09:30:16-00:00
///     wcatwc@noaa.gov,PAAQ-3-mg5a94,2013-01-05T10:17:31-00:00
/// "#.parse().unwrap();
///
/// assert_eq!(references.len(), 3);
/// for reference in &references {
///     assert_eq!(reference.sender, "wcatwc@noaa.gov");
///     println!("alert identifier: {}, sent: {}", reference.identifier, reference.sent);
/// }
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct References(Vec<Reference>);

impl References {
    /// Instantiate `References` from a `Vec<Reference>`.
    pub fn new(references: Vec<Reference>) -> Self {
        Self(references)
    }

    /// Move out of `References` into a `Vec<Reference>`.
    pub fn into_inner(self) -> Vec<Reference> {
        self.0
    }

    /// Return a `&[Reference]` slice.
    pub fn as_slice(&self) -> &[Reference] {
        self.0.as_slice()
    }

    /// Returns an iterator over the references.
    #[must_use]
    pub fn iter(&self) -> std::slice::Iter<Reference> {
        self.0.iter()
    }
}

impl Deref for References {
    type Target = [Reference];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl From<Vec<Reference>> for References {
    fn from(v: Vec<Reference>) -> Self {
        Self(v)
    }
}

impl From<References> for Vec<Reference> {
    fn from(r: References) -> Self {
        r.0
    }
}

impl FromIterator<Reference> for References {
    fn from_iter<T: IntoIterator<Item = Reference>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> IntoIterator for &'a References {
    type Item = &'a Reference;
    type IntoIter = std::slice::Iter<'a, Reference>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for References {
    type Item = Reference;
    type IntoIter = std::vec::IntoIter<Reference>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// An alert reference
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Reference {
    pub sender: Id,
    pub identifier: Id,
    pub sent: DateTime,
}

impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{}", self.sender, self.identifier, self.sent)
    }
}

impl std::str::FromStr for References {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(|chunk| {
                match {
                    let mut i = chunk.split(',');
                    (
                        i.next().map(Id::new),
                        i.next().map(Id::new),
                        i.next().map(DateTime::from_str),
                        i.next(),
                    )
                } {
                    (Some(Ok(sender)), Some(Ok(identifier)), Some(Ok(sent)), None) => {
                        Ok(Reference {
                            sender,
                            identifier,
                            sent,
                        })
                    }
                    _ => Err("invalid reference"),
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
    }
}

impl std::fmt::Display for References {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, reference) in self.0.iter().enumerate() {
            let space = if i == 0 { "" } else { " " };
            write!(f, "{}{}", space, reference)?;
        }
        Ok(())
    }
}

impl Serialize for References {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for References {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = <std::borrow::Cow<str>>::deserialize(deserializer)?;
        str.parse().map_err(D::Error::custom)
    }
}
