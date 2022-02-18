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
    pub fn new(latitude: f64, longitude: f64) -> Option<Self> {
        if (-90.0..=90.0).contains(&latitude) && (-180.0..=180.0).contains(&longitude) {
            Some(Self {
                latitude,
                longitude,
            })
        } else {
            None
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
    type Error = &'static str;

    fn try_from((latitude, longitude): (f64, f64)) -> Result<Self, Self::Error> {
        Self::new(latitude, longitude).ok_or("point out of range")
    }
}

#[derive(Debug, Clone)]
pub struct InvalidPointError(String);

impl std::fmt::Display for InvalidPointError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid point: {:?}", self.0)
    }
}
impl std::error::Error for InvalidPointError {}

impl FromStr for Point {
    type Err = InvalidPointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match {
            let mut i = s.split(',').map(str::trim).map(f64::from_str);
            (i.next(), i.next(), i.next())
        } {
            (Some(Ok(latitude)), Some(Ok(longitude)), None) => Point::new(latitude, longitude),
            _ => None,
        }
        .ok_or(InvalidPointError(s.into()))
    }
}
