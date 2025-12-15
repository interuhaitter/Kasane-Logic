use kasane_logic::{
    geometry::coordinate::Coordinate,
    id::space_id::{SpaceID, range::RangeID},
};

fn main() {
    let id = RangeID::new(5, [-10, -5], [8, 9], [5, 10]).unwrap();

    let vertices: [Coordinate; 8] = id.vertices();
    println!("{:?}", vertices);
}
