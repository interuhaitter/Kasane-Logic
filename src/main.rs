use std::{collections::HashSet, fs::File};

use kasane_logic::{encode_id, encode_id_set::EncodeIDSet, id, space_time_id::SpaceTimeID};
use std::io::Write;
fn main() {
    let mut set1 = EncodeIDSet::new();
    let mut set2 = EncodeIDSet::new();

    let id1 = id! {
        z: 6,
        f: [-1, 10],
        x: [2, 10],
        y: [5, 10],
    }
    .unwrap();

    let id2 = id! {
        z: 4,
        f: [-1, 10],
        x: [2, 10],
        y: [5, 10],
    }
    .unwrap();

    let id3 = id! {
        z: 1,
        f: [1, 1],
        x: [1, 1],
        y: [1, 1],
    }
    .unwrap();

    let id4 = id! {
        z: 2,
        f: [2, 2],
        x: [1, 1],
        y: [1, 1],
    }
    .unwrap();

    let id5 = id! {
        z: 1,
        f: [0, 0],
        x: [0, 0],
        y: [0, 0],
    }
    .unwrap();

    let mut file1 = File::create("output1.txt").expect("cannot create file");

    let mut file2 = File::create("output2.txt").expect("cannot create file");

    let mut file3 = File::create("output3.txt").expect("cannot create file");

    id1.to_encode().iter().for_each(|encode_id| {
        set1.insert(encode_id.clone());
    });

    id2.to_encode().iter().for_each(|encode_id| {
        set1.insert(encode_id.clone());
    });

    id3.to_encode().iter().for_each(|encode_id| {
        set1.insert(encode_id.clone());
    });

    id4.to_encode().iter().for_each(|encode_id| {
        set2.insert(encode_id.clone());
    });

    id5.to_encode().iter().for_each(|encode_id| {
        set2.insert(encode_id.clone());
    });

    for ele in set1.iter() {
        writeln!(file1, "{},", ele.decode()).expect("cannot write to file");
    }

    for ele in set2.iter() {
        writeln!(file2, "{},", ele.decode()).expect("cannot write to file");
    }

    let set3 = set1.difference(&set2);

    for ele in set3.iter() {
        writeln!(file3, "{},", ele.decode()).expect("cannot write to file");
    }
}
