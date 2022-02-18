use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;

/// An identifier, i.e. a string which must not contain internal whitespace.
///
/// `Id`s are `String`s with a reduced domain.
///
/// # Example
///
/// ```rust
/// # use oasiscap::v1dot0::Id;
/// let id: Id = "foo".parse().unwrap();
/// assert_eq!(id, "foo");
/// assert_eq!("foo", id);
///
/// assert!(Id::new("internal whitespace").is_err());
/// assert!(Id::new("prohibited<characters").is_err());
/// assert!(Id::new("prohibited<characters").is_err());
/// ```
///
/// # Whitespace behavior
///
/// `new()` enforces invariants, including that `Id` must not contain whitespace. However,
/// `Deserialize` and `FromStr` are called from XML-related contexts where leading and trailing
/// whitespace may be added. `Serialize` and `FromStr` therefore trim whitespace, while `new()` does
/// not.
///
/// ```rust
/// # use oasiscap::v1dot0::Id;
/// assert_eq!(" parsing-trims-whitespace ".parse::<Id>().unwrap(), "parsing-trims-whitespace");
///
/// assert!(Id::new(" new-does-not ").is_err());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Id(String);

impl Id {
    /// Return the `Id` as a `&str`.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Instantiate a new `Id`.
    pub fn new<S: Into<String>>(string: S) -> Result<Self, InvalidIdError> {
        let string = string.into();
        if string.is_empty() {
            Err(InvalidIdError::Empty)
        } else if string.chars().any(char::is_whitespace) {
            Err(InvalidIdError::ContainsWhitespace)
        } else if string.contains(',') || string.contains('<') || string.contains('&') {
            Err(InvalidIdError::ContainsProhibitedCharacter)
        } else {
            Ok(Self(string))
        }
    }
}

impl std::str::FromStr for Id {
    type Err = InvalidIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.trim())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InvalidIdError {
    Empty,
    ContainsWhitespace,
    ContainsProhibitedCharacter,
}

impl std::fmt::Display for InvalidIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            InvalidIdError::Empty => "ID must not be empty",
            InvalidIdError::ContainsWhitespace => "ID must not contain whitespace",
            InvalidIdError::ContainsProhibitedCharacter => {
                "ID must not contain prohibited characters"
            }
        })
    }
}

impl std::error::Error for InvalidIdError {}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for Id {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl PartialEq<Id> for &str {
    fn eq(&self, other: &Id) -> bool {
        *self == other.0.as_str()
    }
}

impl Deref for Id {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        let trimmed = string.trim();

        // avoid a copy if we can
        if trimmed.len() == string.len() {
            Id::new(string)
        } else {
            Id::new(trimmed)
        }
        .map_err(D::Error::custom)
    }
}
