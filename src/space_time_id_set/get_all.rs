use crate::{
    space_time_id::SpaceTimeId,
    space_time_id_set::{
        SpaceTimeIdSet,
        single::{invert_bitvec_f::invert_bitmask_f, invert_bitvec_xy::invert_bitmask_xy},
    },
};

impl SpaceTimeIdSet {
    pub fn get_all(&self) -> Vec<SpaceTimeId> {
        let mut result = vec![];

        for (_, reverse) in &self.reverse {
            //最大のズームレベルで出てくるという前提に基づく

            let (f_z, f_v) = invert_bitmask_f(&reverse.f);
            let (x_z, x_v) = invert_bitmask_xy(&reverse.x);
            let (y_z, y_v) = invert_bitmask_xy(&reverse.y);

            let max_z = f_z.max(x_z).max(y_z);

            let f = if max_z == f_z {
                [Some(f_v), Some(f_v)]
            } else {
                let k = 2_i64.pow((max_z - f_z).into());
                [Some(f_v * k), Some((f_v + 1) * k - 1)]
            };

            let x = if max_z == x_z {
                [Some(x_v), Some(x_v)]
            } else {
                let k = 2_u64.pow((max_z - x_z).into());
                [Some(x_v * k), Some((x_v + 1) * k - 1)]
            };

            let y = if max_z == y_z {
                [Some(y_v), Some(y_v)]
            } else {
                let k = 2_u64.pow((max_z - y_z).into());
                [Some(y_v * k), Some((y_v + 1) * k - 1)]
            };

            result.push(SpaceTimeId::new(max_z, f, x, y, 0, [None, None]).unwrap());
        }
        result
    }
}
