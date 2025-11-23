use std::{collections::HashSet, fs::File};

use kasane_logic::{space_time_id::SpaceTimeID, space_time_id_set::SpaceTimeIDSet};
use std::io::Write;
fn main() {
    let mut set = SpaceTimeIDSet::new();

    let id1 = SpaceTimeID::new(5, [-1, 10], [2, 10], [5, 10], 10, [10, 40]).unwrap();
    let id2 = SpaceTimeID::new(4, [-1, 10], [2, 10], [5, 10], 10, [10, 40]).unwrap();
    let id3 = SpaceTimeID::new(1, [1, 1], [1, 1], [1, 1], 10, [10, 40]).unwrap();

    let mut file1 = File::create("output.txt").expect("cannot create file");

    let mut file2 = File::create("output_debug.txt").expect("cannot create file");

    set.insert(id1);
    set.insert(id2);
    set.insert(id3);

    println!("{},", id1);
    println!("{},", id2);
    println!("{},", id3);

    for ele in set.iter() {
        writeln!(file1, "{},", ele).expect("cannot write to file");
    }

    writeln!(file2, "{:?},", set).expect("cannot write to file");

    println!("output.txt に書き出しました");
}
