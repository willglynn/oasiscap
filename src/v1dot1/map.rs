use serde::{Deserialize, Serialize};

/// A CAP v1.1 key-value map.
pub type Map = crate::map::Map<Entry>;

/// A CAP v1.1 map entry
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:valueName"
    )]
    value_name: String,
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:value"
    )]
    value: String,
}

impl From<(String, String)> for Entry {
    fn from((value_name, value): (String, String)) -> Self {
        Self { value_name, value }
    }
}

impl From<Entry> for (String, String) {
    fn from(e: Entry) -> Self {
        (e.value_name, e.value)
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
