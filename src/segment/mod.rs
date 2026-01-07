use std::fmt::Display;

pub mod encode;

#[derive(Debug, Clone, Copy)]
pub struct Segment<T> {
    pub z: u8,
    pub dimension: T,
}

impl Segment<u64> {
    /// XY向けのセグメント分割
    pub fn new(z: u8, dimension: [u64; 2]) -> Vec<Segment<u64>> {
        let mut target = dimension;
        let mut now_z = z;
        let mut result = Vec::new();

        loop {
            if target[0] == target[1] {
                result.push(Segment {
                    z: now_z,
                    dimension: target[0],
                });
                break;
            }

            if target[0] % 2 != 0 {
                result.push(Segment {
                    z: now_z,
                    dimension: target[0],
                });
                target[0] += 1;
            }

            if target[1] % 2 == 0 {
                result.push(Segment {
                    z: now_z,
                    dimension: target[1],
                });
                target[1] -= 1;
            }

            if target[0] > target[1] {
                break;
            }

            if now_z == 0 {
                break;
            }

            target = [target[0] / 2, target[1] / 2];
            now_z -= 1;
        }

        result
    }

    pub fn as_z(&self) -> u8 {
        self.z
    }

    pub fn as_dimension(&self) -> u64 {
        self.dimension
    }
}

impl Segment<i64> {
    /// F向けのセグメント分割
    pub fn new(z: u8, dimension: [i64; 2]) -> Vec<Segment<i64>> {
        let diff = 2_i64.pow(z.into());
        let mut target = [dimension[0] + diff, dimension[1] + diff];
        let mut now_z = z;
        let mut result = Vec::new();

        loop {
            if target[0] == target[1] {
                result.push(Segment {
                    z: now_z,
                    dimension: target[0] - 2_i64.pow(now_z.into()),
                });
                break;
            }

            if target[0] % 2 != 0 {
                result.push(Segment {
                    z: now_z,
                    dimension: target[0] - 2_i64.pow(now_z.into()),
                });
                target[0] += 1;
            }

            if target[1] % 2 == 0 {
                result.push(Segment {
                    z: now_z,
                    dimension: target[1] - 2_i64.pow(now_z.into()),
                });
                target[1] -= 1;
            }

            if target[0] > target[1] {
                break;
            }

            if now_z == 0 {
                break;
            }

            target = [target[0] / 2, target[1] / 2];
            now_z -= 1;
        }

        result
    }
    pub fn as_z(&self) -> u8 {
        self.z
    }

    pub fn as_dimension(&self) -> i64 {
        self.dimension
    }
}
