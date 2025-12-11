use std::collections::HashMap;

use kasane_logic::id::space_id::{
    SpaceID,
    constants::{F_MAX, F_MIN, XY_MAX},
    range::RangeID,
    single::SingleID,
};

fn main() {
    let mut id = SingleID::new(4, 6, 9, 14).unwrap();

    println!("{}", id);
    println!("FMAX{}", id.max_f());
    println!("FMIN{}", id.min_f());

    let _ = id.bound_down(50).unwrap();

    println!("{}", id);
}
