use std::collections::HashSet;

use crate::{
    bit_vec::BitVec,
    space_time_id_set::{Index, SpaceTimeIdSet, insert::insert_main_dim::DimensionSelect},
};

impl SpaceTimeIdSet {
    /// 与えられた次元において、上位の範囲を収集する
    pub(crate) fn collect_top(
        &self,
        main_bit: &BitVec,
        main_dim_select: &DimensionSelect,
    ) -> Vec<Index> {
        let dims = self.select_dimensions(&main_dim_select);

        let mut result = HashSet::new();

        for top in main_bit.ancestors() {
            if let Some(v) = dims.main.get(&top) {
                result.extend(v.index.iter().copied());
            }
        }

        Vec::from_iter(result)
    }
}
