use super::Point;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// A closed polygon, i.e. a geo-referenced polyline where the last point is the first point.
///
/// CAP encodes polygons as strings. This crate represents circles as `Polygon`s.
///
/// # Example
///
/// ```
/// # use oasiscap::v1dot0::{Polygon,Point};
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
    type Error = &'static str;

    fn try_from(value: Vec<Point>) -> Result<Self, Self::Error> {
        if value.len() <= 3 {
            Err("too few points")
        } else if !(value.first() == value.last()) {
            Err("points do not form a closed polygon")
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPolygonError(String);

impl std::fmt::Display for InvalidPolygonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid polygon definition: {:?}", self.0)
    }
}
impl std::error::Error for InvalidPolygonError {}

impl FromStr for Polygon {
    type Err = InvalidPolygonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(Point::from_str)
            .collect::<Result<Vec<Point>, _>>()
            .ok()
            .and_then(|vec| Self::try_from(vec).ok())
            .ok_or_else(|| InvalidPolygonError(s.into()))
    }
}
