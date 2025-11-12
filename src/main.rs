use kasane_logic::{
    space_time_id::SpaceTimeId,
    space_time_id_set::{
        SpaceTimeIdSet,
        single::{
            convert_bitvec_f::convert_bitmask_f, convert_single_f::convert_f,
            invert_bitvec_f::invert_bitmask_f,
        },
    },
};

fn main() {
    let mut set = SpaceTimeIdSet::new();
    let id1 = SpaceTimeId::random_z_max(5);
    let id2 = SpaceTimeId::new(
        4,
        [Some(-4), Some(-2)],
        [Some(3), Some(2)],
        [Some(3), Some(2)],
        0,
        [None, None],
    )
    .unwrap();

    // set.insert(id1);

    // println!("{},", id1);
    // // println!("{}", id2);

    // for ele in set.get_all() {
    //     println!("{},", ele);
    // }
    println!("{}", id1);

    let f = convert_f(id1.z, id1.f);

    for a in f {
        println!("{}/{}/-/-,", { a.0 }, { a.1 });
    }
}
