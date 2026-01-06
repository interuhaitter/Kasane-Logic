use crate::bit_vec::BitVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Segment<T> {
    pub z: u8,
    pub dim: T,
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
                    dim: target[0],
                });
                break;
            }

            if target[0] % 2 != 0 {
                result.push(Segment {
                    z: now_z,
                    dim: target[0],
                });
                target[0] += 1;
            }

            if target[1] % 2 == 0 {
                result.push(Segment {
                    z: now_z,
                    dim: target[1],
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
                    dim: target[0] - 2_i64.pow(now_z.into()),
                });
                break;
            }

            if target[0] % 2 != 0 {
                result.push(Segment {
                    z: now_z,
                    dim: target[0] - 2_i64.pow(now_z.into()),
                });
                target[0] += 1;
            }

            if target[1] % 2 == 0 {
                result.push(Segment {
                    z: now_z,
                    dim: target[1] - 2_i64.pow(now_z.into()),
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
}

impl From<Segment<i64>> for BitVec {
    fn from(segment: Segment<i64>) -> Self {
        let z = segment.z;
        let f = segment.dim;

        if f >= 0 {
            return Segment { z, dim: f as u64 }.into();
        }

        let u = (f.abs() - 1) as u64;
        let mut converted: BitVec = Segment { z, dim: u }.into();

        converted.0[0] |= 0b11000000;
        converted
    }
}

impl From<Segment<u64>> for BitVec {
    fn from(segment: Segment<u64>) -> Self {
        let z = segment.z;
        let xy = segment.dim;

        let length = ((z * 2 / 8) + 1).max(1) as usize;
        let mut result = vec![0u8; length];

        let bit_count = (z + 1) as u32;
        let mask = if bit_count >= 64 {
            u64::MAX
        } else {
            (1u64 << bit_count) - 1
        };
        let uxy = xy & mask;

        for now_z in (0..=z).rev() {
            let index = ((now_z) * 2 / 8) as usize;
            let in_index = now_z % 4;

            result[index] |= 1 << (7 - in_index * 2);

            let bit_position = z - now_z;
            if (uxy >> bit_position) & 1 != 0 {
                result[index] |= 1 << (6 - in_index * 2);
            }
        }

        BitVec::from_vec(result)
    }
}







struct SegmentIter {
    z: u8,
    l: i64,
    r: i64,
    cur_z: u8,
}

impl Iterator for SegmentIter {
    type Item = Segment<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        // 区間が空になったら終了
        if self.l > self.r {
            return None;
        }

        let z = self.cur_z;

        // 単一点
        if self.l == self.r {
            let v = self.l;
            self.l += 1;
            return Some(Segment { z, dim: v });
        }

        // 左端が奇数 → 親セルにまとめられない
        if self.l & 1 == 1 {
            let v = self.l;
            self.l += 1;
            return Some(Segment { z, dim: v });
        }

        // 右端が偶数 → 親セルにまとめられない
        if self.r & 1 == 0 {
            let v = self.r;
            self.r -= 1;
            return Some(Segment { z, dim: v });
        }

        // これ以上解像度を下げられない
        if self.cur_z == 0 {
            return None;
        }

        // 親レベルへ昇る
        self.l >>= 1;
        self.r >>= 1;
        self.cur_z -= 1;

        self.next()
    }
}


impl Segment<u64> {
    /// XY インデックス範囲を Segment 列に分割する
    ///
    /// # 概要
    /// 非負整数の連続区間 `[l, r]` を、
    /// 空間IDに変換可能な最小単位の Segment に分解する。
    ///
    /// # 戻り値
    /// `Iterator<Item = Segment<u64>>`
    pub fn iter_xy(z: u8, dimension: [u64; 2]) -> impl Iterator<Item = Segment<u64>> {
        let [l, r] = dimension;

        SegmentIter {
            z,
            l: l as i64,
            r: r as i64,
            cur_z: z,
        }
        .map(|seg| Segment {
            z: seg.z,
            dim: seg.dim as u64,
        })
    }
}

impl Segment<i64> {
    /// F インデックス範囲を Segment 列に分割する
    ///
    /// # 概要
    /// 負数を含む区間を一度 `2^z` だけ平行移動し、
    /// 非負空間で分解したあと元の座標系へ戻す。
    pub fn iter_f(z: u8, dimension: [i64; 2]) -> impl Iterator<Item = Segment<i64>> {
        let diff = 1i64 << z;
        let [l, r] = dimension;

        SegmentIter {
            z,
            l: l + diff,
            r: r + diff,
            cur_z: z,
        }
        .map(move |seg| Segment {
            z: seg.z,
            dim: seg.dim - (1i64 << seg.z),
        })
    }
}




#[test]
fn bitvec_equivalence_xy() {
    let z = 6;
    let range = [3u64, 57];

    let old: Vec<_> = Segment::<u64>::new(z, range)
        .into_iter()
        .map(BitVec::from)
        .collect();

    let new: Vec<_> = Segment::<u64>::iter_xy(z, range)
        .map(BitVec::from)
        .collect();

    assert_eq!(old, new);
}



#[cfg(test)]
mod tests {
    use super::*;

    /* ---------- XY (u64) ---------- */

    #[test]
    fn test_iter_xy_equals_old_new_simple() {
        let z = 3;
        let range = [2, 13];

        let old = Segment::<u64>::new(z, range);
        let new: Vec<_> = Segment::<u64>::iter_xy(z, range).collect();

        assert_eq!(old, new);
    }

    #[test]
    fn test_iter_xy_edge_cases() {
        let cases = [
            (0, [0, 0]),
            (1, [0, 1]),
            (4, [7, 7]),
            (5, [0, 31]),
            (6, [10, 10]),
        ];

        for (z, range) in cases {
            let old = Segment::<u64>::new(z, range);
            let new: Vec<_> = Segment::<u64>::iter_xy(z, range).collect();
            assert_eq!(old, new, "z={z}, range={range:?}");
        }
    }

    /* ---------- F (i64) ---------- */

    #[test]
    fn test_iter_f_equals_old_new_simple() {
        let z = 3;
        let range = [-5, 6];

        let old = Segment::<i64>::new(z, range);
        let new: Vec<_> = Segment::<i64>::iter_f(z, range).collect();

        assert_eq!(old, new);
    }

    #[test]
    fn test_iter_f_cross_zero() {
        let z = 4;
        let range = [-8, 8];

        let old = Segment::<i64>::new(z, range);
        let new: Vec<_> = Segment::<i64>::iter_f(z, range).collect();

        assert_eq!(old, new);
    }

    #[test]
    fn test_iter_f_edge_cases() {
        let cases = [
            (0, [0, 0]),
            (1, [-1, 0]),
            (2, [-2, -2]),
            (5, [-10, 3]),
        ];

        for (z, range) in cases {
            let old = Segment::<i64>::new(z, range);
            let new: Vec<_> = Segment::<i64>::iter_f(z, range).collect();
            assert_eq!(old, new, "z={z}, range={range:?}");
        }
    }
}