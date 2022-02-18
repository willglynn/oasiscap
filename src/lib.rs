//! Types for the OASIS [Common Alerting Protocol].
//!
//! # Example
//!
//! ```rust
//! let alert: oasiscap::Alert = r#"
//! <?xml version = "1.0" encoding = "UTF-8"?>
//! <alert xmlns = "urn:oasis:names:tc:emergency:cap:1.2">
//!   <identifier>43b080713727</identifier>
//!   <sender>hsas@dhs.gov</sender>
//!   <sent>2003-04-02T14:39:01-05:00</sent>
//!   <status>Actual</status>
//!   <msgType>Alert</msgType>
//!   <scope>Public</scope>
//!   <info>
//!     <!-- … -->
//!#     <category>Security</category>
//!#     <event>Homeland Security Advisory System Update</event>
//!#     <urgency>Immediate</urgency>
//!#     <severity>Severe</severity>
//!#     <certainty>Likely</certainty>
//!#     <senderName>U.S. Government, Department of Homeland Security</senderName>
//!#     <headline>Homeland Security Sets Code ORANGE</headline>
//!#     <description>The Department of Homeland Security has elevated the Homeland Security Advisory System threat level to ORANGE / High in response to intelligence which may indicate a heightened threat of terrorism.</description>
//!#     <instruction> A High Condition is declared when there is a high risk of terrorist attacks. In addition to the Protective Measures taken in the previous Threat Conditions, Federal departments and agencies should consider agency-specific Protective Measures in accordance with their existing plans.</instruction>
//!#     <web>http://www.dhs.gov/dhspublic/display?theme=29</web>
//!   </info>
//! </alert>
//! "#.parse().expect("parse CAP");
//!
//! // Handle CAP alerts of various versions
//! match &alert {
//!     oasiscap::Alert::V1dot0(alert) => println!("CAP v1.0: {:?}", alert),
//!     oasiscap::Alert::V1dot1(alert) => println!("CAP v1.1: {:?}", alert),
//!     oasiscap::Alert::V1dot2(alert) => println!("CAP v1.2: {:?}", alert),
//! }
//!
//! // Upgrade to the latest CAP version
//! let alert: oasiscap::v1dot2::Alert = alert.into_latest();
//!
//! // Convert back to XML again
//! let alert_xml = alert.to_string();
//! ```
//!
//! # Conformance
//!
//! The CAP specifications are split between human- and machine-readable components. CAP v1.2 § 4.2
//! explains:
//!
//! > An XML 1.0 document is a conforming CAP V1.2 Message if and only if:
//! >
//! > a) it is valid according to [the schema] and
//! >
//! > b) the content of its elements and the values of its attributes meet all the additional
//! >    mandatory requirements specified in Section 3.
//!
//! Consider the `<polygon>` element. The machine-readable XML schema says that a polygon is just a
//! string:
//!
//! ```xml
//! <element name = "polygon" type = "xs:string" minOccurs = "0" maxOccurs = "unbounded"/>
//! ```
//!
//! The human-readable document says that a polygon is specifically a string describing a closed
//! polyline in a particular geospatial reference frame, and imposes the following requirements
//! in section 3:
//!
//! > (1) Code Values: The geographic polygon is represented by a whitespace-delimited list of WGS
//! > 84 coordinate pairs. (See WGS 84 Note at end of this section)
//! >
//! > (2) A minimum of 4 coordinate pairs MUST be present and the first and last pairs of
//! > coordinates MUST be the same.
//!
//! This crate implements those rules from section 3:
//!
//! ```rust
//! use oasiscap::geo::Polygon;
//!
//! // 4 points, where the last point is the first point, makes a Polygon:
//! assert!("1,1 2,2 3,3 1,1".parse::<Polygon>().is_ok());
//!
//! // 4 points where the last point differs does not make a Polygon:
//! assert!("1,1 2,2 3,3 4,4".parse::<Polygon>().is_err());
//!
//! // 3 points does not make a Polygon:
//! assert!("1,1 2,2 1,1".parse::<Polygon>().is_err());
//!
//! // invalid WGS-84 coordinates do not make a Polygon:
//! assert!("100,100 200,200 300,300 100,100".parse::<Polygon>().is_err());
//! ```
//!
//! All of those strings are permitted by the XML schema, but only the first one makes sense as a
//! polygon. This crate therefore accepts the first string and rejects the others.
//!
//! Having said that, some real-world CAP alerts violate the requirements in section 3 but _do_
//! still make sense:
//!
//! ```xml
//! <polygon></polygon>
//! ```
//!
//! Polygons are optional, so the element can and should have been omitted in its entirety. On the
//! other hand, an empty string _is_ valid according to the XML schema, and its intent is
//! unambiguous even if it is technically non-conforming. This crate therefore accepts an empty
//! polygon element as a synonym for omitting the polygon, rather than returning an error.
//!
//! This crate intends to always parse conforming CAP messages and to always generate conforming CAP
//! messages. At the same time, this crate intends to be pedantic to preserve _meaning_, not to be
//! pendantic for pedantry's sake. It therefore does not reject all non-conforming CAP messages,
//! particularly for common implementation mistakes which have reasonable and unambiguous
//! interpretations.
//!
//! # Performance
//!
//! `oasiscap` prioritizes being correct over being fast, but it is still reasonably fast. On an
//! industry standard developer's laptop using unspecified versions of this library, Rust, and the
//! underlying operating system, parsing a typical `oasiscap::Alert` from XML takes approximately
//! 55µs, for a throughput of roughly 18,000 alerts per second per core. Generating XML from a
//! typical `oasiscap::Alert` takes approximately 27µs, for a throughput of roughly 38,000 alerts
// per second per core.
//!
//! Clone the repository and run `cargo bench` to see how it performs in your environment.
//!
//! [Common Alerting Protocol]: https://en.wikipedia.org/wiki/Common_Alerting_Protocol
//! [xml_serde]: https://crates.io/crates/xml_serde
//! [the schema]: http://docs.oasis-open.org/emergency/cap/v1.2/CAP-v1.2.xsd

#![forbid(unsafe_code)]
#![deny(missing_docs)]

extern crate core;

use serde::{Deserialize, Serialize};

mod datetime;
pub use datetime::DateTime;

pub mod digest;

mod embedded_data;
pub use embedded_data::EmbeddedContent;

pub mod delimited_items;
pub mod geo;
pub mod id;
pub mod language;
pub mod map;
pub mod references;

pub mod v1dot0;
pub mod v1dot1;
pub mod v1dot2;

#[cfg(feature = "prost")]
pub mod protobuf;

pub(crate) mod url;

pub use ::url::Url;

/// A CAP alert message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Alert {
    /// A CAP v1.0 alert message
    #[serde(rename = "{http://www.incident.com/cap/1.0;}cap:alert")]
    V1dot0(v1dot0::Alert),
    /// A CAP v1.1 alert message
    #[serde(
        rename = "{urn:oasis:names:tc:emergency:cap:1.1;https://docs.oasis-open.org/emergency/cap/v1.1/errata/approved/cap.xsd}cap:alert"
    )]
    V1dot1(v1dot1::Alert),
    /// A CAP v1.2 alert message
    #[serde(rename = "{urn:oasis:names:tc:emergency:cap:1.2;}cap:alert")]
    V1dot2(v1dot2::Alert),
}

impl Alert {
    /// A unique identifier for this alert, assigned by the sender
    pub fn identifier(&self) -> &crate::id::Id {
        match self {
            Alert::V1dot0(alert) => &alert.identifier,
            Alert::V1dot1(alert) => &alert.identifier,
            Alert::V1dot2(alert) => &alert.identifier,
        }
    }

    /// A globally-unique identifier for the sender
    pub fn sender(&self) -> &crate::id::Id {
        match self {
            Alert::V1dot0(alert) => &alert.sender,
            Alert::V1dot1(alert) => &alert.sender,
            Alert::V1dot2(alert) => &alert.sender,
        }
    }

    /// The date and time at which this alert originated
    pub fn sent(&self) -> crate::DateTime {
        match self {
            Alert::V1dot0(alert) => alert.sent,
            Alert::V1dot1(alert) => alert.sent,
            Alert::V1dot2(alert) => alert.sent,
        }
    }

    /// Returns the XML namespace corresponding to the encapsulated CAP alert version.
    pub fn xml_namespace(&self) -> &'static str {
        match self {
            Alert::V1dot0(_) => "http://www.incident.com/cap/1.0",
            Alert::V1dot1(_) => "urn:oasis:names:tc:emergency:cap:1.1",
            Alert::V1dot2(_) => "urn:oasis:names:tc:emergency:cap:1.2",
        }
    }

    /// Return this alert as the latest supported alert version, upgrading it as necessary.
    ///
    /// CAP v1.2 is mostly a superset of earlier versions, with two exceptions:
    ///
    /// 1. CAP <= v1.1 `Resource` has an optional `mime_type`, whereas it's required for CAP v1.2.
    /// This crate supplies `application/octet-stream` as a default if needed.
    ///
    /// ```
    /// # let input = include_str!("../fixtures/v1dot0_appendix_adot1.xml");
    /// // let input: &str = /* CAP v1.0 appendix A.1 */;
    /// let alert: oasiscap::Alert = input.parse().unwrap();
    /// match &alert {
    ///     oasiscap::Alert::V1dot0(alert) => {
    ///         assert!(alert.info[0].resources[0].mime_type.is_none());
    ///     }
    ///     _ => unreachable!(),
    /// }
    ///
    /// let alert = alert.into_latest();
    /// assert_eq!(alert.info[0].resources[0].mime_type, "application/octet-stream");
    /// #
    /// # let expected = include_str!("../fixtures/v1dot2_appendix_adot1.xml");
    /// # let expected: oasiscap::v1dot2::Alert = expected.parse().unwrap();
    /// # let mut alert = alert;
    /// # alert.info[0].instruction = None;
    /// # alert.info[0].description = None;
    /// # alert.info[0].resources[0].mime_type = "image/gif".into();
    /// # let mut expected = expected;
    /// # expected.info[0].instruction = None;
    /// # expected.info[0].description = None;
    /// # assert_eq!(alert, expected);
    /// ```
    ///
    /// 2. CAP v1.0 has `Certainty::VeryLikely`, while later versions do not. The specification
    ///    recommends substituting `Certainty::Likely`, so this crate does.
    ///
    /// ```
    /// # let input = include_str!("../fixtures/v1dot0_appendix_adot3.xml");
    /// // let input: &str = /* CAP v1.0 appendix A.3 */;
    /// let alert: oasiscap::Alert = input.parse().unwrap();
    /// match &alert {
    ///     oasiscap::Alert::V1dot0(alert) => {
    ///         assert_eq!(alert.info[0].certainty, oasiscap::v1dot0::Certainty::VeryLikely);
    ///     }
    ///     _ => unreachable!(),
    /// }
    ///
    /// let alert = alert.into_latest();
    /// assert_eq!(alert.info[0].certainty, oasiscap::v1dot2::Certainty::Likely);
    /// ```
    pub fn into_latest(self) -> crate::v1dot2::Alert {
        match self {
            Alert::V1dot0(alert) => alert.into(),
            Alert::V1dot1(alert) => alert.into(),
            Alert::V1dot2(alert) => alert,
        }
    }
}

impl std::str::FromStr for Alert {
    type Err = xml_serde::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        xml_serde::from_str(s)
    }
}

impl std::fmt::Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        xml_serde::to_string(self)
            .map_err(|_| std::fmt::Error)
            .and_then(|str| f.write_str(&str))
    }
}
