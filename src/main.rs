use kasane_logic::geometry::{
    coordinate::Coordinate,
    shapes::line::{line, line_dda},
};
use rand::prelude::*;
use std::fs::File;
use std::io::Write;

const MIN_LAT: f64 = 20.0;
const MAX_LAT: f64 = 46.0;
const MIN_LON: f64 = 122.0;
const MAX_LON: f64 = 154.0;
const MIN_ALT: f64 = 0.0;
const MAX_ALT: f64 = 1000.0;
fn rondom_point(rng: &mut impl Rng) -> Coordinate {
    Coordinate::new(
        rng.random_range(MIN_LAT..MAX_LAT),
        rng.random_range(MIN_LON..MAX_LON),
        rng.random_range(MIN_ALT..MAX_ALT),
    )
    .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("output.txt")?;
    let tokyo = Coordinate::new(35.681382, 139.76608399999998, 0.0)?;
    let nagoya = Coordinate::new(35.1706431, 136.8816945, 100.0)?;
    let yokohama = Coordinate::new(35.4660694, 139.6226196, 100.0)?;
    let (z, start, goal) = (25, nagoya, tokyo);
    let iter = line_dda(z, start, goal)?;
    for id in iter {
        // SingleIDの内容を一行ずつ書き込む
        writeln!(file, "{},", id)?;
    }
    println!("{},{}", start.to_id(z), goal.to_id(z));
    Ok(println!("結果を output.txt に保存しました 。"))
}
