use crate::{
    bit_vec::BitVec,
    space_time_id_set::{Interval, SpaceTimeIdSet, insert::insert_main_dim::DimensionSelect},
};

impl SpaceTimeIdSet {
    pub(crate) fn uncheck_insert_dim(
        &mut self,
        dim_select: DimensionSelect,
        main: &BitVec,
        a: &BitVec,
        b: &BitVec,
        t: &Interval,
    ) {
        match dim_select {
            DimensionSelect::F => {
                self.uncheck_insert(main, a, b, t);
            }
            DimensionSelect::X => {
                self.uncheck_insert(a, main, b, t);
            }
            DimensionSelect::Y => {
                self.uncheck_insert(a, b, main, t);
            }
        }
    }
}
