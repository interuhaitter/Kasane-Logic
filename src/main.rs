use kasane_logic::segment::{Segment, encode::EncodeSegment};

fn main() {
    let segment = Segment {
        z: 30,
        dimension: 5u64,
    };

    let result = EncodeSegment::from(segment);

    println!("{}", result);
}
