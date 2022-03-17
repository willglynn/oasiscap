use chrono::{FixedOffset, Offset, TimeZone, Timelike};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// A CAP timestamp.
///
/// `DateTime` corresponds exactly to a `chrono::DateTime<FixedOffset>`, except that CAP timestamps
/// are limited to 1-second resolution.
///
/// # Example
///
/// ```
/// # use chrono::Timelike;
/// // Get a chrono DateTime
/// let chrono_now = chrono::Local::now();
///
/// // Convert a chrono DateTime to a CAP DateTime
/// let cap_now = oasiscap::DateTime::from(chrono_now);
/// assert_eq!(chrono_now, cap_now);
/// assert_eq!(cap_now, chrono_now);
///
/// // Convert back
/// // The fractional seconds are gone, but the timestamp is otherwise equal
/// let chrono_roundtrip = chrono::DateTime::from(cap_now);
/// assert_eq!(chrono_roundtrip.nanosecond(), 0);
/// assert_eq!(chrono_now.with_nanosecond(0).unwrap(), chrono_roundtrip);
///
/// // Format the CAP DateTime as a CAP string
/// let cap_string = cap_now.to_string();
/// assert_eq!(cap_string.len(), 25);
///
/// // Parse a CAP DateTime back from a CAP string
/// let parsed: oasiscap::DateTime = cap_string.parse().unwrap();
/// assert_eq!(parsed, cap_now);
/// assert_eq!(parsed, chrono_now);
/// ```
///
/// # Textual encoding history
///
/// CAP v1.0 specifies timestamps to be "in ISO 8601 format", which sounds reasonable but
/// unfortunately permits a very wide range of possible formats. The XML schema simply states that
/// timestamps are `xs:dateTime`.
///
/// `xs:dateTime` is "closely related to" and "inspired by" ISO 8601, but _is_ a different model,
/// so CAP v1.0 is a bit self-contradictory. A larger concern is that `xs:dateTime` specifies that
/// the canonical encoding for UTC is "Z":
///
/// > The mapping so defined is one-to-one, except that '+00:00', '-00:00', and 'Z' all represent
/// > the same zero-length duration timezone, UTC; 'Z' is its canonical representation.
///
/// This caused problems for some implementations which expected numeric offsets.
///
/// CAP v1.1 addressed this point, specified that timestamps are "in [dateTime] format, and
/// specifically links to `xs:dateTime`'s definition. It also requires a numeric offset:
///
/// > Alphabetic timezone designators such as “Z” MUST NOT be used. The timezone for UTC MUST be
/// > represented as “-00:00”.
///
/// However, `xs:dateTime` permits both "untimezoned times" and fractional seconds:
///
/// > "Local" or untimezoned times are presumed to be the time in the timezone of some unspecified
/// > locality as prescribed by the appropriate legal authority
///
/// Untimezoned times are not desirable for automatic processing of public safety information, and
/// fractional seconds offer needless flexibility.
///
/// CAP v1.2 finally settled the matter, with the XML schema specifying that timezone offsets must
/// always be numeric, and that fractional seconds are not permitted:
///
/// ```xml
///  <simpleType>
///     <restriction base = "xs:dateTime">
///      <pattern value = "\d\d\d\d-\d\d-\d\dT\d\d:\d\d:\d\d[-,+]\d\d:\d\d"/>
///     </restriction>
///   </simpleType>
/// ```
///
/// Other implementers subsequently back-ported this formatting requirement to CAP v1.1 and v1.0,
/// e.g. [Google's Public Alert extended CAP v1.0 schema].
///
/// # Textual encoding decisions
///
/// `DateTime` accepts values with fractional seconds because they are legal `xs:dateTime` values,
/// but it discards the fraction, because fractional seconds seem to have been permitted by
/// accident.
///
/// ```
/// # use oasiscap::DateTime;
/// assert_eq!(
///     "2002-05-24T16:49:00.123-01:00".parse::<DateTime>().unwrap(),
///     "2002-05-24T16:49:00-01:00".parse::<DateTime>().unwrap(),
/// );
///
/// assert_eq!(
///     "2002-05-24T16:49:00.123-01:00".parse::<DateTime>().unwrap().to_string(),
///     "2002-05-24T16:49:00-01:00",
/// );
/// ```
///
/// `DateTime` rejects untimezoned values, even though they are legal `xs:dateTime` values and
/// therefore are permitted by some versions of the CAP standards, because the lack of reference
/// frame makes them unsuited for automatic processing.
///
/// ```
/// # use oasiscap::DateTime;
/// assert!("2002-05-24T16:49:00".parse::<DateTime>().is_err());
/// ```
///
/// `DateTime` accepts `+00:00`, `-00:00`, and `Z` as synonyms for UTC, since all three are legal
/// `xs:dateTime` values, and all three clearly indicate UTC. This is more permissive than CAP v1.2,
/// but these timezone offsets are unambiguous, and it is useful for interoperability. `DateTime`
/// always formats UTC as `-00:00`, which strictly complies with CAP v1.2 and is also accepted by
/// CAP v1.1 and v1.0 systems.
///
/// ```
/// # use oasiscap::DateTime;
/// assert_eq!(
///     "2002-05-24T16:49:00-00:00".parse::<DateTime>().unwrap().to_string(),
///     "2002-05-24T16:49:00-00:00",
/// );
/// assert_eq!(
///     "2002-05-24T16:49:00+00:00".parse::<DateTime>().unwrap().to_string(),
///     "2002-05-24T16:49:00-00:00",
/// );
/// assert_eq!(
///     "2002-05-24T16:49:00Z".parse::<DateTime>().unwrap().to_string(),
///     "2002-05-24T16:49:00-00:00",
/// );
/// ```
///
/// [dateTime]: https://www.w3.org/TR/xmlschema-2/#dateTime
/// [Google's Public Alert extended CAP v1.0 schema]: https://github.com/google/cap-library/blob/master/schema/cap10_extended.xsd#L54
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DateTime(chrono::DateTime<FixedOffset>);

impl FromStr for DateTime {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('Z') {
            FixedOffset::west(0).datetime_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")
        } else {
            chrono::DateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f%:z")
        }
        .map(Self::from)
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0.offset().local_minus_utc() == 0 {
            self.0.format("%Y-%m-%dT%H:%M:%S-00:00").fmt(f)
        } else {
            self.0.format("%Y-%m-%dT%H:%M:%S%:z").fmt(f)
        }
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = <std::borrow::Cow<str>>::deserialize(deserializer)?;
        let dt = str.parse().map_err(serde::de::Error::custom)?;
        Ok(Self(dt))
    }
}

impl From<DateTime> for chrono::DateTime<FixedOffset> {
    fn from(dt: DateTime) -> Self {
        dt.0
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for DateTime {
    fn from(dt: chrono::DateTime<Tz>) -> Self {
        Self(
            dt.with_timezone(&dt.offset().fix())
                .with_nanosecond(0)
                .expect("timestamp must be valid with zero nanoseconds"),
        )
    }
}

impl<Tz: chrono::TimeZone> PartialEq<chrono::DateTime<Tz>> for DateTime {
    fn eq(&self, other: &chrono::DateTime<Tz>) -> bool {
        self.0.eq(&other.with_nanosecond(0).unwrap())
    }
}

impl<Tz: chrono::TimeZone> PartialEq<DateTime> for chrono::DateTime<Tz> {
    fn eq(&self, other: &DateTime) -> bool {
        self.with_nanosecond(0).unwrap().eq(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn parse() {
        assert_eq!(
            "2002-05-24T16:49:00-07:00".parse(),
            Ok(DateTime(
                FixedOffset::west(7 * 3600)
                    .ymd(2002, 5, 24)
                    .and_hms(16, 49, 0)
            ))
        );

        // Omitting offset is not permitted
        assert!(DateTime::from_str("2002-05-24T16:49:00").is_err());
    }

    #[test]
    fn parse_utc() {
        // Zero offset is fine according to CAP v1.2
        let reference = DateTime::from_str("2002-05-24T16:49:00-00:00").unwrap();

        // We decide that positive zero offset is equivalent
        assert_eq!(
            DateTime::from_str("2002-05-24T16:49:00+00:00").unwrap(),
            reference
        );

        // And we decide that Z is equivalent
        assert_eq!(
            DateTime::from_str("2002-05-24T16:49:00Z").unwrap(),
            reference
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            DateTime(
                FixedOffset::west(7 * 3600)
                    .ymd(2002, 5, 24)
                    .and_hms(16, 49, 0)
            )
            .to_string(),
            "2002-05-24T16:49:00-07:00"
        );

        // Alphabetic timezone designators such as “Z” MUST NOT be used. The timezone for UTC MUST be represented as “-00:00”.
        assert_eq!(
            DateTime(FixedOffset::west(0).ymd(2002, 5, 24).and_hms(16, 49, 0)).to_string(),
            "2002-05-24T16:49:00-00:00"
        );
    }

    #[test]
    fn conversions() {
        assert_eq!(
            chrono::DateTime::from(DateTime::from_str("2002-05-24T16:49:00-00:00").unwrap()),
            FixedOffset::west(0).ymd(2002, 5, 24).and_hms(16, 49, 0),
        );

        assert_eq!(
            DateTime::from(
                FixedOffset::west(7 * 3600)
                    .ymd(2002, 5, 24)
                    .and_hms(16, 49, 0)
            ),
            DateTime::from_str("2002-05-24T16:49:00-07:00").unwrap()
        );

        assert_eq!(
            DateTime::from(chrono::Utc.ymd(2002, 5, 24).and_hms(16, 49, 0)),
            DateTime::from_str("2002-05-24T16:49:00-00:00").unwrap()
        );
    }
}
