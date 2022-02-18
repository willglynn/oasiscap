//! Whitespace-delimited item lists.
//!
//! CAP uses repeated elements to provide repetition in most contexts:
//!
//! ```xml
//! <item>Prince</item>
//! <item>the Artist Formerly Known as Prince</item>
//! ```
//!
//! …but evidently some contexts are better suited to concatenating strings, and potentially
//! enclosing items in quotation marks:
//!
//! ```xml
//! <item>Prince "the Artist Formerly Known as Prince"</item>
//! ```
//!
//! The rationale behind this design choice is left as an exercise for the reader.
//!
//! This module implements this scheme. [`Items`] is a container of [`Item`]s, and [`Item`]s are
//! `String`s which do not contain double quotation marks. Implementation is not to be construed as
//! endorsement.
//!
//! [`Items`]: struct.Items.html
//! [`Item`]: struct.Item.html

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
use std::str::FromStr;

/// A container for whitespace-delimited strings, each of which may contain internal whitespace by
/// enclosing strings with double quotes.
///
/// `Items` can be parsed from strings:
///
/// ```
/// use oasiscap::delimited_items::*;
///
/// // Empty strings are okay
/// assert!("".parse::<Items>().unwrap().is_empty());
///
/// // The typical case is an item of one
/// assert_eq!(
///     "foo".parse::<Items>().unwrap().as_slice(),
///     ["foo"].as_slice(),
/// );
///
/// // The less-typical case is space- or newline-delimited items
/// assert_eq!(
///     "foo bar".parse::<Items>().unwrap().as_slice(),
///     ["foo", "bar"].as_slice(),
/// );
///
/// // The even-less-typical case involves double quotes
/// assert_eq!(
///     "foo \"bar baz\"".parse::<Items>().unwrap().as_slice(),
///     ["foo", "bar baz"].as_slice(),
/// );
///
/// // Unclosed quotes are an error
/// assert_eq!("foo \"bar baz".parse::<Items>(), Err(UnclosedQuotesError));
///
/// // Extra spaces are not semantically meaningful
/// assert_eq!(
///     "  foo     bar  ".parse::<Items>().unwrap().as_slice(),
///     ["foo", "bar"].as_slice(),
/// );
///
/// // ...unless they are in quotes
/// assert_eq!(
///     "  foo   \"  bar \" ".parse::<Items>().unwrap().as_slice(),
///     ["foo", "  bar "].as_slice(),
/// );
///
/// // All whitespace operates as a delimiter
/// assert_eq!(
///     "  foo\n\tbar ".parse::<Items>().unwrap().as_slice(),
///     ["foo", "bar"].as_slice(),
/// );
/// ```
///
/// `Items` can be formatted as a string:
///
/// ```
/// use oasiscap::delimited_items::*;
///
///
/// assert_eq!(
///     Items::default().to_string(),
///     "",
/// );
///
/// assert_eq!(
///     Items::try_from(vec!["foo", "bar"]).unwrap().to_string(),
///     "foo bar",
/// );
///
/// assert_eq!(
///     Items::try_from(vec!["foo", "bar baz"]).unwrap().to_string(),
///     "foo \"bar baz\"",
/// );
///
/// assert_eq!(
///     Items::try_from(vec![" foo "]).unwrap().to_string(),
///     "\" foo \"",
/// );
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Items(Vec<Item>);

impl Items {
    /// Get a slice of `Item`s.
    ///
    /// # Example
    ///
    /// ```
    /// use oasiscap::delimited_items::*;
    ///
    /// let items: Items = "foo bar".parse().unwrap();
    /// let slice = items.as_slice();
    /// assert_eq!(slice[0], "foo");
    /// assert_eq!(slice[1], "bar");
    /// ```
    pub fn as_slice(&self) -> &[Item] {
        self.0.as_slice()
    }

    /// Instantiate `Items` from a `Vec<Items>`.
    ///
    /// # Example
    ///
    /// ```
    /// use oasiscap::delimited_items::*;
    ///
    /// let items = Items::new(vec![
    ///     Item::try_from("foo").unwrap(),
    ///     Item::try_from("bar").unwrap(),
    /// ]);
    ///
    /// assert_eq!(items.to_string(), "foo bar");
    pub fn new(items: Vec<Item>) -> Self {
        Self(items)
    }

    /// Consume `Items`, returning the `Vec<Item>` it contains.
    ///
    /// # Example
    ///
    /// ```
    /// use oasiscap::delimited_items::*;
    ///
    /// let items: Items = "foo bar".parse().unwrap();
    /// assert_eq!(
    ///     items.into_inner(),
    ///     vec![
    ///         Item::try_from("foo").unwrap(),
    ///         Item::try_from("bar").unwrap(),
    ///     ],
    /// );
    /// ```
    pub fn into_inner(self) -> Vec<Item> {
        self.0
    }
}

impl Deref for Items {
    type Target = [Item];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> IntoIterator for &'a Items {
    type Item = &'a Item;
    type IntoIter = std::slice::Iter<'a, Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for Items {
    type Item = Item;
    type IntoIter = std::vec::IntoIter<Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Item> for Items {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromStr for Items {
    type Err = UnclosedQuotesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();

        let mut in_double_quotes = false;
        let mut item_start_at = 0;

        let mut chars = s.chars().enumerate().peekable();
        while let Some((index, char)) = chars.next() {
            if in_double_quotes {
                if char == '"' {
                    let item = String::from(&s[item_start_at..index]);
                    items.push(Item::try_from(item).unwrap());
                    item_start_at = index + 1;
                    in_double_quotes = false;
                } else {
                    continue;
                }
            } else if char == '"' {
                in_double_quotes = true;
                item_start_at = index + 1;
            } else if let Some(end) = Some(index)
                .filter(|_| char.is_ascii_whitespace())
                .or_else(|| Some(index + 1).filter(|_| chars.peek().is_none()))
            {
                // We're at the end of an item, either because this character is the space (in which
                // case we exclude it) or because the this character is the end (in which case we
                // include it)
                let item = String::from(s[item_start_at..end].trim());
                if !item.is_empty() {
                    items.push(Item::try_from(item).unwrap());
                }
                item_start_at = index + 1;
            }
        }

        if in_double_quotes {
            Err(UnclosedQuotesError)
        } else {
            Ok(Self(items))
        }
    }
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (index, item) in self.iter().enumerate() {
            let space = if index == 0 { "" } else { " " };

            if item.contains(' ') {
                write!(f, "{}\"{}\"", space, item)?;
            } else {
                write!(f, "{}{}", space, item)?;
            }
        }
        Ok(())
    }
}

impl<S: Into<String>> TryFrom<Vec<S>> for Items {
    type Error = InvalidItemError;

    fn try_from(value: Vec<S>) -> Result<Self, Self::Error> {
        value
            .into_iter()
            .map(|s| s.into())
            .map(Item::try_from)
            .collect()
    }
}

impl Serialize for Items {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Items {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}

/// A `String` which must not contain the double quote character `\"`.
///
/// # Example
///
/// ```
/// use oasiscap::delimited_items::*;
///
/// assert_eq!(Item::try_from("foo").unwrap(), "foo");
/// assert_eq!(Item::try_from("foo bar").unwrap(), "foo bar");
///
/// assert_eq!(Item::try_from("foo\"bar"), Err(InvalidItemError));
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item(String);

impl Item {
    /// Consume the `Item`, returning the `String` inside.
    ///
    /// # Example
    ///
    /// ```
    /// use oasiscap::delimited_items::*;
    ///
    /// let item: Item = "foo".parse().unwrap();
    ///
    /// let inner: String = item.into_inner();
    /// assert_eq!(inner, "foo");
    /// ```
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Deref for Item {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for Item {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl TryFrom<String> for Item {
    type Error = InvalidItemError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains('"') {
            Err(InvalidItemError)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<&str> for Item {
    type Error = InvalidItemError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        String::from(value).try_into()
    }
}

impl FromStr for Item {
    type Err = InvalidItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.try_into()
    }
}

impl From<Item> for String {
    fn from(i: Item) -> Self {
        i.0
    }
}

impl PartialEq<&str> for Item {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Item> for &str {
    fn eq(&self, other: &Item) -> bool {
        *self == other.0
    }
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        string.try_into().map_err(D::Error::custom)
    }
}

/// The error returned when an `Items` string contains an unterminated quoted section.
///
/// # Example
///
/// ```
/// use oasiscap::delimited_items::*;
///
/// assert_eq!(
///     "foo \"bar\" baz \"quxx".parse::<Items>(),
///     Err(UnclosedQuotesError)
/// );
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UnclosedQuotesError;

impl std::fmt::Display for UnclosedQuotesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("invalid items: contained an unclosed set of double quotes")
    }
}
impl std::error::Error for UnclosedQuotesError {}

/// The error returned when an `Item` would contain double quotes.
///
/// # Example
///
/// ```
/// use oasiscap::delimited_items::*;
///
/// assert_eq!(Item::try_from("double\"quote"), Err(InvalidItemError));
/// assert_eq!(Items::try_from(vec!["double\"quote"]), Err(InvalidItemError));
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct InvalidItemError;

impl std::fmt::Display for InvalidItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("invalid item: items must not contain double quotes")
    }
}
impl std::error::Error for InvalidItemError {}
