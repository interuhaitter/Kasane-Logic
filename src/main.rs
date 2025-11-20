use std::fs::File;

use kasane_logic::{
    function::line::line,
    point::{Coordinate, Point},
    space_time_id::SpaceTimeId,
    space_time_id_set::SpaceTimeIdSet,
};
use std::io::Write;
fn main() {
    let mut set = SpaceTimeIdSet::new();

    let id1 = SpaceTimeId::new(
        5,
        [Some(-1), Some(10)],
        [Some(2), Some(10)],
        [Some(5), Some(10)],
        10,
        [Some(10), Some(40)],
    );
    let id2 = SpaceTimeId::random_z_max(5);

    let mut file = File::create("output.txt").expect("cannot create file");

    for ele in set.get_all() {
        writeln!(file, "{},", ele).expect("cannot write to file");
    }

    println!("output.txt に書き出しました");
}
