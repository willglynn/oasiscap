use super::{Area, Circle, Polygon};

/// The error returned when an `Area` conversion would be invalid.
#[derive(thiserror::Error, Debug)]
pub enum AreaConversionError {
    /// A geocode parameter was invalid
    #[error("geocode map key is invalid: {0}")]
    Geocode(#[from] crate::v1dot0::map::InvalidKeyError),
    /// A polygon is invalid
    #[error("polygon is invalid: {0}")]
    Polygon(#[from] crate::geo::InvalidPolygonError),
    /// A circle is invalid
    #[error("circle is invalid: {0}")]
    Circle(#[from] crate::geo::InvalidCircleError),
}

impl TryFrom<Area> for crate::v1dot0::Area {
    type Error = AreaConversionError;

    fn try_from(value: Area) -> Result<Self, Self::Error> {
        Ok(Self {
            description: value.area_desc,
            polygons: value
                .polygon
                .into_iter()
                .map(crate::geo::Polygon::try_from)
                .collect::<Result<_, _>>()?,
            circles: value
                .circle
                .into_iter()
                .map(crate::geo::Circle::try_from)
                .collect::<Result<_, _>>()?,
            geocode: value.geocode.try_into()?,
            altitude: value.altitude,
            ceiling: value.ceiling,
        })
    }
}

impl From<crate::v1dot0::Area> for Area {
    fn from(value: crate::v1dot0::Area) -> Self {
        Self {
            area_desc: value.description,
            polygon: value.polygons.into_iter().map(Polygon::from).collect(),
            circle: value.circles.into_iter().map(Circle::from).collect(),
            geocode: value.geocode.into(),
            altitude: value.altitude,
            ceiling: value.ceiling,
        }
    }
}

impl TryFrom<Area> for crate::v1dot1::Area {
    type Error = AreaConversionError;

    fn try_from(value: Area) -> Result<Self, Self::Error> {
        Ok(Self {
            description: value.area_desc,
            polygons: value
                .polygon
                .into_iter()
                .map(crate::geo::Polygon::try_from)
                .collect::<Result<_, _>>()?,
            circles: value
                .circle
                .into_iter()
                .map(crate::geo::Circle::try_from)
                .collect::<Result<_, _>>()?,
            geocode: value.geocode.into(),
            altitude: value.altitude,
            ceiling: value.ceiling,
        })
    }
}

impl From<crate::v1dot1::Area> for Area {
    fn from(value: crate::v1dot1::Area) -> Self {
        Self {
            area_desc: value.description,
            polygon: value.polygons.into_iter().map(Polygon::from).collect(),
            circle: value.circles.into_iter().map(Circle::from).collect(),
            geocode: value.geocode.into(),
            altitude: value.altitude,
            ceiling: value.ceiling,
        }
    }
}

impl TryFrom<Area> for crate::v1dot2::Area {
    type Error = AreaConversionError;

    fn try_from(value: Area) -> Result<Self, Self::Error> {
        Ok(Self {
            description: value.area_desc,
            polygons: value
                .polygon
                .into_iter()
                .map(crate::geo::Polygon::try_from)
                .collect::<Result<_, _>>()?,
            circles: value
                .circle
                .into_iter()
                .map(crate::geo::Circle::try_from)
                .collect::<Result<_, _>>()?,
            geocode: value.geocode.into(),
            altitude: value.altitude,
            ceiling: value.ceiling,
        })
    }
}

impl From<crate::v1dot2::Area> for Area {
    fn from(value: crate::v1dot2::Area) -> Self {
        Self {
            area_desc: value.description,
            polygon: value.polygons.into_iter().map(Polygon::from).collect(),
            circle: value.circles.into_iter().map(Circle::from).collect(),
            geocode: value.geocode.into(),
            altitude: value.altitude,
            ceiling: value.ceiling,
        }
    }
}
