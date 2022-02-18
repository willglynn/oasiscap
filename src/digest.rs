//! Types for cryptographic digests.
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// A SHA-1 digest.
///
/// # Example
///
/// ```
/// use oasiscap::digest::Sha1;
///
/// let bytes: [u8; 20] = [
///         0xb2, 0xfd, 0xc4, 0xf4, 0x78, 0xc3, 0x0b, 0x52, 0x45, 0x57,
///         0x98, 0x53, 0x36, 0x69, 0x23, 0xcc, 0xfb, 0x66, 0x6a, 0xb5
///     ];
///
/// // digest::Sha1 parses from a hex string
/// let digest = "b2fdc4f478c30b5245579853366923ccfb666ab5".parse::<Sha1>().unwrap();
/// assert_eq!(digest, Sha1::from(bytes));
///
/// // digest::Sha1 converts to a byte array
/// assert_eq!(<[u8; 20]>::from(digest), bytes);
///
/// // digest::Sha1 compares against byte arrays and byte slices
/// assert_eq!(digest, bytes);
/// assert_eq!(bytes, digest);
/// assert_eq!(digest, bytes.as_slice());
/// assert_eq!(bytes.as_slice(), digest);
///
/// // digest::Sha1 displays as a hex string
/// assert_eq!(digest.to_string(), "b2fdc4f478c30b5245579853366923ccfb666ab5");
///
/// // digest::Sha1 is case-insensitive
/// let upper_digest = "B2FDC4F478C30B5245579853366923CCFB666AB5".parse::<Sha1>().unwrap();
/// assert_eq!(upper_digest, digest);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Sha1([u8; 20]);

impl AsRef<[u8; 20]> for Sha1 {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl AsRef<[u8]> for Sha1 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl PartialEq<&[u8]> for Sha1 {
    fn eq(&self, other: &&[u8]) -> bool {
        self.0.as_slice() == *other
    }
}
impl PartialEq<Sha1> for &[u8] {
    fn eq(&self, other: &Sha1) -> bool {
        *self == other.0
    }
}

impl PartialEq<[u8; 20]> for Sha1 {
    fn eq(&self, other: &[u8; 20]) -> bool {
        &self.0 == other
    }
}
impl PartialEq<Sha1> for [u8; 20] {
    fn eq(&self, other: &Sha1) -> bool {
        self == &other.0
    }
}

/// The error returned when a `Sha1` would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum Sha1ParseError {
    /// SHA-1 digest must be 40 characters long
    #[error("SHA-1 digest must be 40 characters long: got {0}")]
    Length(usize),
    /// SHA-1 digest must be hexadecimal
    #[error("SHA-1 digest must hexadecimal: got {0}")]
    Digits(String),
}

impl FromStr for Sha1 {
    type Err = Sha1ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.len() != 40 {
            return Err(Sha1ParseError::Length(s.len()))?;
        }

        let mut bytes = [0u8; 20];
        for octet in 0..20 {
            bytes[octet] = u8::from_str_radix(&s[octet * 2..octet * 2 + 2], 16)
                .map_err(|_| Sha1ParseError::Digits(s.into()))?;
        }

        Ok(Self(bytes))
    }
}

impl std::fmt::Display for Sha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
               self.0[0],
               self.0[1],
               self.0[2],
               self.0[3],
               self.0[4],
               self.0[5],
               self.0[6],
               self.0[7],
               self.0[8],
               self.0[9],
               self.0[10],
               self.0[11],
               self.0[12],
               self.0[13],
               self.0[14],
               self.0[15],
               self.0[16],
               self.0[17],
               self.0[18],
               self.0[19],
        )
    }
}

impl From<[u8; 20]> for Sha1 {
    fn from(v: [u8; 20]) -> Self {
        Self(v)
    }
}

impl From<Sha1> for [u8; 20] {
    fn from(v: Sha1) -> Self {
        v.0
    }
}

impl Serialize for Sha1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Sha1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = <std::borrow::Cow<str>>::deserialize(deserializer)?;
        str.parse().map_err(D::Error::custom)
    }
}
