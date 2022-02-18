//! Types for representing languages.

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// An optional language specifier.
///
/// The CAP XML schemas refer to [`xs:language`], while the text of the standards refer to
/// [RFC 1766] and [RFC 3066]. `Language` here chooses the most permissive set of valid languages:
///
/// ```text
/// [a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*
/// ```
///
/// CAP languages are optional, in which case:
///
/// > (2) If not present, assumed value is "en-US".
///
/// > (2) If not present, an implicit default value of "en-US" SHALL be assumed.
/// > (3) A null value in this element SHALL be considered equivalent to “en-US.”
///
/// It is clear that the `<language>` element can be omitted altogether (which should default to
/// `en-US`), but it unclear whether `<language></language>` should be an error. Given the stated
/// default, `Language` chooses not to distinguish: `<language></language>` is treated as though
/// `<language>` was omitted entirely.
///
/// # Example
///
/// ```
/// # use oasiscap::language::Language;
/// #
/// // Default is Option::None
/// let default = Language::default();
/// assert_eq!(default, "en-US");
/// assert_eq!(default.as_str(), "en-US");
/// assert_eq!(default.as_option_str(), None);
/// assert_eq!(default.into_inner(), None);
///
/// // Language strings can be parsed into a Language
/// let en_us: Language = "en-US".parse().unwrap();
/// assert_eq!(en_us, "en-US");
/// assert_eq!(en_us, &Language::default());
/// assert_eq!(en_us.as_str(), "en-US");
/// assert_eq!(en_us.as_option_str(), Some("en-US"));
/// assert_eq!(en_us.into_inner(), Some("en-US".to_string()));
///
/// // Nonsense strings are rejected
/// assert!("-".parse::<Language>().is_err());
/// assert!("12-34".parse::<Language>().is_err());
/// ```
///
/// [`xs:language`]: https://www.w3.org/TR/xmlschema11-2/#language
/// [RFC 1766]: https://datatracker.ietf.org/doc/html/rfc1766
/// [RFC 3066]: https://datatracker.ietf.org/doc/html/rfc3066
///
///
#[derive(Debug, Clone, Default)]
pub struct Language(Option<String>);

impl Language {
    /// Instantiate a `Language` from an optional string.
    ///
    /// # Example
    ///
    /// ```
    /// # use oasiscap::language::Language;
    /// // Accepts strings, or absence of strings
    /// assert!(Language::new("en-GB".to_string()).is_ok());
    /// assert!(Language::new(None).is_ok());
    ///
    /// // As a special case, accepts empty strings as equivalent to None
    /// assert!(Language::new("".to_string()).is_ok());
    ///
    /// // Rejects strings which do not fit the `xs:language` pattern
    /// assert!(Language::new("not a language".to_string()).is_err());
    /// assert!(Language::new("r2-D2".to_string()).is_err());
    ///
    /// // Accepts strings which do fit the pattern, even if they're not real languages
    /// assert!(Language::new("artoo-D2".to_string()).is_ok());
    /// ```
    pub fn new<S: Into<Option<String>>>(value: S) -> Result<Self, InvalidLanguageError> {
        if let Some(string) = value.into() {
            string.try_into()
        } else {
            Ok(Self(None))
        }
    }

    /// Consume the `Language`, returning an `Option<String>`.
    ///
    /// # Example
    ///
    /// ```
    /// # use oasiscap::language::Language;
    /// assert_eq!(Language::default().into_inner(), None);
    /// assert_eq!(Language::new(String::from("foo")).unwrap().into_inner(), Some(String::from("foo")));
    ///
    /// // As a special case, empty strings are treated as None
    /// assert_eq!(Language::new(String::from("")).unwrap().into_inner(), None);
    /// ```
    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    /// Return the `Language` as a `&str`, using `"en-US"` as a default.
    ///
    /// # Example
    ///
    /// ```
    /// # use oasiscap::language::Language;
    /// assert_eq!(Language::default().as_str(), "en-US");
    /// assert_eq!(Language::new(String::from("foo")).unwrap().as_str(), "foo");
    /// ```
    pub fn as_str(&self) -> &str {
        self.as_option_str().unwrap_or("en-US")
    }

    /// Return the `Language` as an `Option<&str>`.
    ///
    /// # Example
    ///
    /// ```
    /// # use oasiscap::language::Language;
    /// assert_eq!(Language::default().as_option_str(), None);
    /// assert_eq!(Language::new(String::from("foo")).unwrap().as_option_str(), Some("foo"));
    /// ```
    pub fn as_option_str(&self) -> Option<&str> {
        self.0.as_deref()
    }

    /// Returns `true` if the `Language` was not specified.
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
impl PartialEq<&Self> for Language {
    fn eq(&self, other: &&Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl PartialEq<&str> for Language {
    fn eq(&self, other: &&str) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<Language> for &str {
    fn eq(&self, other: &Language) -> bool {
        *self == other.as_ref()
    }
}

impl FromStr for Language {
    type Err = InvalidLanguageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        String::from(s).try_into()
    }
}

impl TryFrom<String> for Language {
    type Error = InvalidLanguageError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Ok(Self(None));
        }

        let str = value.as_str();
        for (i, chunk) in str.split('-').enumerate() {
            let all_valid_chars = if i == 0 {
                chunk.chars().all(|c| c.is_ascii_alphabetic())
            } else {
                chunk.chars().all(|c| c.is_ascii_alphanumeric())
            };
            if !all_valid_chars || chunk.is_empty() || chunk.len() > 8 {
                return Err(InvalidLanguageError(value));
            }
        }

        Ok(Self(Some(value)))
    }
}

/// The error returned when a `Language` would be invalid.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvalidLanguageError(String);

impl std::fmt::Display for InvalidLanguageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid language value: {:?}", self.0)
    }
}
impl std::error::Error for InvalidLanguageError {}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.as_option_str() {
            Some(str) => serializer.serialize_str(str),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = <Option<String>>::deserialize(deserializer)?;
        Language::new(value).map_err(D::Error::custom)
    }
}
