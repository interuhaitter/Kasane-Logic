use kasane_logic::space_id::{SpaceID, range::RangeID, single::SingleID};

fn main() {
    let mut id = RangeID::new(4, [-5, 3], [3, 6], [1, 2]).unwrap();

    println!("{},", id);

    id.move_up(3).unwrap();
    id.move_east(3);

    println!("{},", id);
}
