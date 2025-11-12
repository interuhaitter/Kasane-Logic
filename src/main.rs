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
    let id = SpaceTimeId::new(
        2,
        [Some(-4), Some(-2)],
        [Some(3), Some(2)],
        [Some(3), Some(2)],
        0,
        [None, None],
    )
    .unwrap();
    let id2 = SpaceTimeId::random_z_max(6);

    set.insert(id2);

    println!("{}", id2);

    println!("-------------");

    for ele in set.get_all() {
        println!("{},", ele);
    }

    // while true {

    //     let f = convert_f(id2.z, id2.f);

    //     for ele in f {
    //         let convert = convert_bitmask_f(ele.0, ele.1);

    //         let invert = invert_bitmask_f(&convert);

    //         if ele != invert {
    //             println!("-----------");
    //             println!("before:{}/{}/-/-,", { ele.0 }, { ele.1 });
    //             println!("after:{}/{}/-/-,", { invert.0 }, { invert.1 });

    //             break;
    //         }
    //     }
    // }
}
