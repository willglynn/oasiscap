use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// A geo-referenced circle with a given center point and radius.
///
/// CAP encodes circles as strings. This crate represents circles as `Circle`s.
///
/// # Example
///
/// ```
/// # use oasiscap::v1dot0::Circle;
/// #
/// // Valid strings parse as Circles
/// assert_eq!(
///     "32.9525,-115.5527 0".parse::<Circle>().unwrap(),
///     Circle {
///         latitude: 32.9525,
///         longitude: -115.5527,
///         radius: 0.0,
///     }
/// );
///
/// // Invalid strings parse as errors, due to:
/// // * Format problems,
/// assert!("32.9525,-115.5527".parse::<Circle>().is_err());
/// assert!("32.9525,-115.5527 0 0".parse::<Circle>().is_err());
/// assert!("32.9525,-115.5527,0".parse::<Circle>().is_err());
/// // * Invalid coordinates, or
/// assert!("32.9525,-185.5527 0".parse::<Circle>().is_err());
/// assert!("92.9525,-115.5527 0".parse::<Circle>().is_err());
/// // * Invalid radii
/// assert!("32.9525 -115.5527 -1".parse::<Circle>().is_err());
/// assert!("32.9525 -115.5527 0".parse::<Circle>().is_err());
/// assert!("32.9525 -115.5527 100000".parse::<Circle>().is_err());
///
/// // Circles can be formatted as strings
/// assert_eq!(
///     Circle {
///         latitude: 32.9525,
///         longitude: -115.5527,
///         radius: 0.0,
///     }
///     .to_string(),
///     "32.9525,-115.5527 0"
/// );
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    /// The latitude of the center of the circle, in WGS 84 (EPSG:4326) coordinates.
    pub latitude: f64,
    /// The longitude of the center of the circle, in WGS 84 (EPSG:4326) coordinates.
    pub longitude: f64,
    /// The radius of the circle, in kilometers
    pub radius: f64,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{} {}", self.latitude, self.longitude, self.radius)
    }
}

impl Serialize for Circle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Circle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = <std::borrow::Cow<str>>::deserialize(deserializer)?;
        str.parse().map_err(D::Error::custom)
    }
}

/// The error returned when a circle definition is invalid.
#[derive(Debug, Clone)]
pub struct InvalidCircleError(String);

impl std::fmt::Display for InvalidCircleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid circle definition: {:?}", self.0)
    }
}
impl std::error::Error for InvalidCircleError {}

impl FromStr for Circle {
    type Err = InvalidCircleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match {
            let mut i = s.split_whitespace();
            (
                i.next(),
                i.next().and_then(|s| s.trim().parse().ok()),
                i.next(),
            )
        } {
            (Some(lat_lon), Some(radius), None) => Ok((lat_lon, radius)),
            _ => Err(()),
        }
        .and_then(|(lat_lon, radius)| {
            match {
                let mut i = lat_lon.splitn(2, ',').map(|p| p.trim().parse());
                (i.next(), i.next())
            } {
                (Some(Ok(lat)), Some(Ok(lon))) => Ok(Self {
                    latitude: lat,
                    longitude: lon,
                    radius,
                }),
                _ => Err(()),
            }
        })
        .and_then(|circle| {
            if (-90.0..=90.0).contains(&circle.latitude)
                && (-180.0..=180.0).contains(&circle.longitude)
                && (0.0..20000.0).contains(&circle.radius)
            {
                Ok(circle)
            } else {
                Err(())
            }
        })
        .map_err(|_| InvalidCircleError(s.into()))
    }
}
