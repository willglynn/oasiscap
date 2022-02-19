impl From<crate::geo::Point> for super::Point {
    fn from(value: crate::geo::Point) -> Self {
        Self {
            latitude: value.latitude(),
            longitude: value.longitude(),
        }
    }
}

impl TryFrom<super::Point> for crate::geo::Point {
    type Error = crate::geo::InvalidPointError;

    fn try_from(value: super::Point) -> Result<Self, Self::Error> {
        (value.latitude, value.longitude).try_into()
    }
}

impl From<crate::geo::Polygon> for super::Polygon {
    fn from(value: crate::geo::Polygon) -> Self {
        Self {
            point: value.into_iter().map(super::Point::from).collect(),
        }
    }
}

impl TryFrom<super::Polygon> for crate::geo::Polygon {
    type Error = crate::geo::InvalidPolygonError;

    fn try_from(value: super::Polygon) -> Result<Self, Self::Error> {
        value
            .point
            .into_iter()
            .map(crate::geo::Point::try_from)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
    }
}

impl From<crate::geo::Circle> for super::Circle {
    fn from(value: crate::geo::Circle) -> Self {
        Self {
            point: value.center.into(),
            radius: value.radius,
        }
    }
}

impl TryFrom<super::Circle> for crate::geo::Circle {
    type Error = crate::geo::InvalidCircleError;

    fn try_from(value: super::Circle) -> Result<Self, Self::Error> {
        crate::geo::Circle::new(value.point.try_into()?, value.radius)
    }
}
