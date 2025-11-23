use crate::{point::Point, space_time_id::SpaceTimeID};

pub fn point(z: u8, point1: Point) -> SpaceTimeID {
    match point1 {
        Point::Coordinate(coordinate) => coordinate.to_id(z),
        Point::ECEF(ecef) => ecef.to_id(z),
    }
}
