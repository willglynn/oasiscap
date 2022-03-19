use super::*;

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

impl From<v1dot0::Alert> for Alert {
    fn from(v: v1dot0::Alert) -> Self {
        Self::V1dot0(v)
    }
}

impl From<v1dot1::Alert> for Alert {
    fn from(v: v1dot1::Alert) -> Self {
        Self::V1dot1(v)
    }
}

impl From<v1dot2::Alert> for Alert {
    fn from(v: v1dot2::Alert) -> Self {
        Self::V1dot2(v)
    }
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
