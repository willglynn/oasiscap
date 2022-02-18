use base64ct::Encoding;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Debug;
use std::ops::Deref;

/// Binary data embedded inside a CAP message.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmbeddedContent(Vec<u8>);

impl EmbeddedContent {
    /// Returns a byte slice of the embedded data.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Deref for EmbeddedContent {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl AsRef<[u8]> for EmbeddedContent {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<Vec<u8>> for EmbeddedContent {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<EmbeddedContent> for Vec<u8> {
    fn from(v: EmbeddedContent) -> Self {
        v.0
    }
}

impl TryFrom<String> for EmbeddedContent {
    type Error = &'static str;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        // Promptly treat it as bytes
        let mut bytes = string.into_bytes();

        // Keep everything that isn't whitespace
        bytes.retain(|b| !(*b as char).is_ascii_whitespace());

        // Decode the bytes in place, returning the decoded length
        let len = base64ct::Base64::decode_in_place(bytes.as_mut_slice())
            .map(|slice| slice.len())
            .map_err(|_| "invalid base64 data")?;

        // Truncate to the decoded length
        bytes.truncate(len);
        Ok(Self(bytes))
    }
}

impl<'de> Deserialize<'de> for EmbeddedContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize a string
        String::deserialize(deserializer)?
            .try_into()
            .map_err(D::Error::custom)
    }
}

impl Serialize for EmbeddedContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.0.as_slice();

        // Make an output buffer of the right length
        let len = base64ct::Base64::encoded_len(bytes);
        let mut output = vec![0u8; len];

        // Encode into it
        let str = base64ct::Base64::encode(bytes, &mut output).unwrap();

        // Serialize the string
        serializer.serialize_str(&str)
    }
}

impl ToString for EmbeddedContent {
    fn to_string(&self) -> String {
        let bytes = self.0.as_slice();

        // Make an output buffer of the right length
        let len = base64ct::Base64::encoded_len(bytes);
        let mut output = vec![0u8; len];

        // Encode into it
        let str = base64ct::Base64::encode(bytes, &mut output).unwrap();
        str.into()
    }
}
