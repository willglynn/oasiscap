//! Types for geospatial data.
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// A geographic point, in WGS 84 (EPSG:4326) coordinates.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    latitude: f64,
    longitude: f64,
}

impl Point {
    /// Instantiate a new point for a given latitude and longitude.
    ///
    /// Returns `Some(Point)` if the latitude and longitude are in bounds, or `None` otherwise.
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, InvalidPointError> {
        if (-90.0..=90.0).contains(&latitude) && (-180.0..=180.0).contains(&longitude) {
            Ok(Self {
                latitude,
                longitude,
            })
        } else {
            Err(InvalidPointError::CoordinatesOutOfRange {
                latitude,
                longitude,
            })
        }
    }

    /// The latitude of the point
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// The longitude of the point
    pub fn longitude(&self) -> f64 {
        self.longitude
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{}", self.latitude, self.longitude)
    }
}

impl TryFrom<(f64, f64)> for Point {
    type Error = InvalidPointError;

    fn try_from((latitude, longitude): (f64, f64)) -> Result<Self, Self::Error> {
        Self::new(latitude, longitude)
    }
}

/// The error returned when a `Point` would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum InvalidPointError {
    /// The point string could not be parsed
    #[error("bad format: {0:?}")]
    BadFormat(String),

    /// The coordinates are out of range
    #[error("coordinates out of range: {latitude} latitude, {longitude} longitude")]
    CoordinatesOutOfRange {
        /// The specified latitude
        latitude: f64,
        /// The specified longitude
        longitude: f64,
    },
}

impl FromStr for Point {
    type Err = InvalidPointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match {
            let mut i = s.split(',').map(str::trim).map(f64::from_str);
            (i.next(), i.next(), i.next())
        } {
            (Some(Ok(latitude)), Some(Ok(longitude)), None) => Point::new(latitude, longitude),
            _ => Err(InvalidPointError::BadFormat(s.into())),
        }
    }
}

/// A closed polygon, i.e. a geo-referenced polyline where the last point is the first point.
///
/// CAP encodes polygons as strings. This crate represents circles as `Polygon`s.
///
/// # Example
///
/// ```
/// # use oasiscap::geo::{Polygon,Point};
/// #
/// // Valid strings parse as polygons
/// assert_eq!(
///     "38.47,-120.14 38.34,-119.95 38.52,-119.74 38.62,-119.89 38.47,-120.14"
///         .parse::<Polygon>()
///         .unwrap(),
///     Polygon::try_from(vec![
///         Point::new(38.47, -120.14).unwrap(),
///         Point::new(38.34, -119.95).unwrap(),
///         Point::new(38.52, -119.74).unwrap(),
///         Point::new(38.62, -119.89).unwrap(),
///         Point::new(38.47, -120.14).unwrap(),
///     ]).unwrap()
/// );
/// assert_eq!(
///     "1,1 2,2 3,3 1,1".parse::<Polygon>().unwrap(),
///     Polygon::try_from(vec![
///         Point::new(1.0, 1.0).unwrap(),
///         Point::new(2.0, 2.0).unwrap(),
///         Point::new(3.0, 3.0).unwrap(),
///         Point::new(1.0, 1.0).unwrap(),
///     ]).unwrap(),
/// );
///
/// // Invalid strings parse as errors, due to:
/// // * Not ending where it starts
/// assert!("1,1 2,2 3,3 4,4".parse::<Polygon>().is_err());
/// // * Format errors
/// assert!("1,1,1 2,2,2 3,3,3 1,1,1".parse::<Polygon>().is_err());
/// assert!("1 2 3 1".parse::<Polygon>().is_err());
/// assert!("-100,-100 100,100 200,200 -100,-100".parse::<Polygon>().is_err());
///
/// // Polygons can be formatted as strings
/// assert_eq!(
///    Polygon::try_from(vec![
///        Point::new(38.47, -120.14).unwrap(),
///        Point::new(38.34, -119.95).unwrap(),
///        Point::new(38.52, -119.74).unwrap(),
///        Point::new(38.62, -119.89).unwrap(),
///        Point::new(38.47, -120.14).unwrap(),
///    ]).unwrap().to_string(),
///    "38.47,-120.14 38.34,-119.95 38.52,-119.74 38.62,-119.89 38.47,-120.14",
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon(Vec<Point>);

impl Polygon {
    /// Returns an iterator over the points in this `Polygon`.
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.0.iter()
    }

    /// Returns an iterator which moves points out of this `Polygon`.
    #[must_use]
    pub fn into_iter(self) -> impl Iterator<Item = Point> {
        self.0.into_iter()
    }

    // Deserialize, but treat `<polygon></polygon>` the same as ``.
    pub(crate) fn deserialize_optional<'de, D>(deserializer: D) -> Result<Vec<Polygon>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let strs = <Vec<std::borrow::Cow<str>>>::deserialize(deserializer)?;
        strs.into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| Polygon::from_str(&s))
            .collect::<Result<Vec<_>, _>>()
            .map_err(D::Error::custom)
    }
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, point) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " {}", point)?;
            } else {
                write!(f, "{}", point)?;
            }
        }
        Ok(())
    }
}

impl Serialize for Polygon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Polygon {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = <std::borrow::Cow<str>>::deserialize(deserializer)?;
        str.parse().map_err(D::Error::custom)
    }
}

impl TryFrom<Vec<Point>> for Polygon {
    type Error = InvalidPolygonError;

    fn try_from(value: Vec<Point>) -> Result<Self, Self::Error> {
        if value.len() <= 3 {
            Err(InvalidPolygonError::TooFewPoints(value.len()))
        } else if !(value.first() == value.last()) {
            Err(InvalidPolygonError::ShapeNotClosed(
                *value.first().unwrap(),
                *value.last().unwrap(),
            ))
        } else {
            Ok(Self(value))
        }
    }
}

/// The error returned when a `Polygon` would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum InvalidPolygonError {
    /// The polygon contained too few points
    #[error("polygon contains too few points: got {0} vs 4 minimum")]
    TooFewPoints(
        /// The specified number of points
        usize,
    ),

    /// The shape was not closed
    #[error("shape not closed: first point {0} != last point {0}")]
    ShapeNotClosed(
        /// The first point
        Point,
        /// The last point
        Point,
    ),

    /// The polygon contained an invalid point
    #[error("polygon contains invalid point: {0}")]
    InvalidPoint(#[from] InvalidPointError),
}

impl FromStr for Polygon {
    type Err = InvalidPolygonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(
            s.split_whitespace()
                .map(Point::from_str)
                .collect::<Result<Vec<Point>, _>>()?,
        )
    }
}

/// A geo-referenced circle with a given center point and radius.
///
/// CAP encodes circles as strings. This crate represents circles as `Circle`s.
///
/// # Example
///
/// ```
/// # use oasiscap::geo::{Circle, Point};
/// #
/// // Valid strings parse as Circles
/// assert_eq!(
///     "32.9525,-115.5527 0".parse::<Circle>().unwrap(),
///     Circle {
///         center: Point::new(32.9525, -115.5527).unwrap(),
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
///         center: Point::new(32.9525, -115.5527).unwrap(),
///         radius: 0.0,
///     }
///     .to_string(),
///     "32.9525,-115.5527 0"
/// );
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    /// The center point of the circle
    pub center: Point,
    /// The radius of the circle, in kilometers
    pub radius: f64,
}

impl Circle {
    /// Instantiate a new `Circle` around a given `center` with a specified `radius` in kilometers.
    pub fn new(center: Point, radius: f64) -> Result<Self, InvalidCircleError> {
        if (0.0..20000.0).contains(&radius) {
            Ok(Self { center, radius })
        } else {
            Err(InvalidCircleError::RadiusTooLarge(radius))
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{},{} {}",
            self.center.latitude(),
            self.center.longitude(),
            self.radius
        )
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
#[derive(thiserror::Error, Debug)]
pub enum InvalidCircleError {
    /// The string could not be parsed
    #[error("unparseable circle string: {0:?}")]
    UnparseableString(String),
    /// The center point was invalid
    #[error("circle center point is invalid: {0}")]
    InvalidCenterPoint(#[from] InvalidPointError),
    /// The circle radius was too large
    #[error("circle radius is too large: {0} km")]
    RadiusTooLarge(f64),
}

impl FromStr for Circle {
    type Err = InvalidCircleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (center, radius) = match {
            let mut i = s.split_whitespace();
            (
                i.next(),
                i.next().and_then(|s| s.trim().parse().ok()),
                i.next(),
            )
        } {
            (Some(center), Some(radius), None) => Ok((center, radius)),
            _ => Err(InvalidCircleError::UnparseableString(s.into())),
        }?;

        let center = center.parse()?;

        Self::new(center, radius)
    }
}
