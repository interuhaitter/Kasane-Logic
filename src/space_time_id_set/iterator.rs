use crate::{
    space_time_id::SpaceTimeID,
    space_time_id_set::{
        ReverseInfo, SpaceTimeIDSet,
        single::{invert_bitvec_f::invert_bitmask_f, invert_bitvec_xy::invert_bitmask_xy},
    },
};

pub struct SpaceTimeIDSetIter<'a> {
    reverse_iter: std::collections::hash_map::Iter<'a, usize, ReverseInfo>,
}

impl SpaceTimeIDSet {
    pub fn iter(&'_ self) -> SpaceTimeIDSetIter<'_> {
        SpaceTimeIDSetIter {
            reverse_iter: self.reverse.iter(),
        }
    }
}

impl<'a> Iterator for SpaceTimeIDSetIter<'a> {
    type Item = SpaceTimeID;

    fn next(&mut self) -> Option<Self::Item> {
        let (_index, reverse) = self.reverse_iter.next()?; // <-- ここが(usize, ReverseInfo)

        let (f_z, f_v) = invert_bitmask_f(&reverse.f);
        let (x_z, x_v) = invert_bitmask_xy(&reverse.x);
        let (y_z, y_v) = invert_bitmask_xy(&reverse.y);

        let max_z = f_z.max(x_z).max(y_z);

        let f = if max_z == f_z {
            [f_v, f_v]
        } else {
            let k = 2_i64.pow((max_z - f_z) as u32);
            [f_v * k, (f_v + 1) * k - 1]
        };

        let x = if max_z == x_z {
            [x_v, x_v]
        } else {
            let k = 2_u64.pow((max_z - x_z) as u32);
            [x_v * k, (x_v + 1) * k - 1]
        };

        let y = if max_z == y_z {
            [y_v, y_v]
        } else {
            let k = 2_u64.pow((max_z - y_z) as u32);
            [y_v * k, (y_v + 1) * k - 1]
        };

        Some(SpaceTimeID {
            z: max_z,
            f,
            x,
            y,
            i: 0,
            t: [0, u64::MAX],
        })
    }
}

impl<'a> IntoIterator for &'a SpaceTimeIDSet {
    type Item = SpaceTimeID;
    type IntoIter = SpaceTimeIDSetIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> ExactSizeIterator for SpaceTimeIDSetIter<'a> {
    fn len(&self) -> usize {
        self.reverse_iter.len()
    }
}
