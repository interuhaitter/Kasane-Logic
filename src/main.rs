use kasane_logic::{space_time_id::SpaceTimeId, space_time_id_set::SpaceTimeIdSet};

fn main() {
    let id = SpaceTimeId::random_z_max(8);
    println!("{}", id);

    let mut set = SpaceTimeIdSet::new();

    set.insert(id);
}
