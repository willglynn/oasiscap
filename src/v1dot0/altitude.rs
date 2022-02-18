use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "XmlAltitude", into = "XmlAltitude")]
pub enum Altitude {
    Unspecified,
    Specific(i64),
    Range(RangeInclusive<i64>),
}

#[derive(Debug, Serialize, Deserialize)]
struct XmlAltitude {
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:altitude",
        skip_serializing_if = "Option::is_none"
    )]
    altitude: Option<i64>,
    #[serde(
        rename = "{http://www.incident.com/cap/1.0}cap:ceiling",
        skip_serializing_if = "Option::is_none"
    )]
    ceiling: Option<i64>,
}

impl TryFrom<XmlAltitude> for Altitude {
    type Error = &'static str;

    fn try_from(value: XmlAltitude) -> Result<Self, Self::Error> {
        match value {
            XmlAltitude {
                altitude: None,
                ceiling: None,
            } => Ok(Altitude::Unspecified),
            XmlAltitude {
                altitude: Some(a),
                ceiling: None,
            } => Ok(Altitude::Specific(a)),
            XmlAltitude {
                altitude: Some(a),
                ceiling: Some(b),
            } => Ok(Altitude::Range(a..=b)),
            _ => Err("ceiling must not be specified without altitude"),
        }
    }
}

impl From<Altitude> for XmlAltitude {
    fn from(a: Altitude) -> Self {
        match a {
            Altitude::Unspecified => XmlAltitude {
                altitude: None,
                ceiling: None,
            },
            Altitude::Specific(altitude) => XmlAltitude {
                altitude: Some(altitude),
                ceiling: None,
            },
            Altitude::Range(range) => XmlAltitude {
                altitude: Some(*range.start()),
                ceiling: Some(*range.end()),
            },
        }
    }
}

#[test]
fn from_xml() {
    #[derive(Deserialize)]
    struct Doc {
        #[serde(rename = "{http://www.incident.com/cap/1.0}:cap:container")]
        container: Container,
    }

    #[derive(Deserialize)]
    struct Container {
        #[serde(flatten)]
        altitude: Altitude,
    }

    assert_eq!(
        xml_serde::from_str::<Doc>(
            r#"
<cap:container xmlns:cap="http://www.incident.com/cap/1.0">
    <cap:altitude>100</cap:altitude>
</cap:container>
"#,
        )
        .unwrap()
        .container
        .altitude,
        Altitude::Specific(100)
    );
}
