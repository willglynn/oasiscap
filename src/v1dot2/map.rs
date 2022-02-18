use serde::{Deserialize, Serialize};

/// A CAP v1.2 key-value map.
pub type Map = crate::map::Map<Entry>;

/// A CAP v1.2 map entry
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    #[serde(rename = "{urn:oasis:names:tc:emergency:cap:1.2;}cap:valueName")]
    value_name: String,
    #[serde(rename = "{urn:oasis:names:tc:emergency:cap:1.2;}cap:value")]
    value: String,
}

impl From<(String, String)> for Entry {
    fn from((value_name, value): (String, String)) -> Self {
        Self { value_name, value }
    }
}

impl Into<(String, String)> for Entry {
    fn into(self) -> (String, String) {
        (self.value_name, self.value)
    }
}

impl crate::map::Entry for Entry {
    fn value_name(&self) -> &str {
        &self.value_name
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn set_value(&mut self, mut new_value: String) -> String {
        std::mem::swap(&mut new_value, &mut self.value);
        new_value
    }
}
