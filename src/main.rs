use std::collections::HashSet;

use kasane_logic::{
    space_time_id::SpaceTimeId,
    space_time_id_set::{
        SpaceTimeIdSet,
        single::{
            convert_bitvec_f::convert_bitmask_f, convert_bitvec_xy::convert_bitmask_xy,
            convert_single_f::convert_f, convert_single_xy::convert_xy,
            invert_bitvec_f::invert_bitmask_f, invert_bitvec_xy::invert_bitmask_xy,
        },
    },
    r#type::bit_vec::BitVec,
};

fn main() {
    let top = BitVec::from_vec(vec![0b10000000]);
    let mut under = BitVec::from_vec(vec![0b10101100]);

    let result = SpaceTimeIdSet::split_dimension(&top, &mut under);

    for ele in result {
        println!("{}", ele);
    }
}
