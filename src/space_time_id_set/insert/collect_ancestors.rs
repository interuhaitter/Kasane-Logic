use std::collections::HashSet;

use crate::{
    bit_vec::BitVec,
    space_time_id_set::{Index, SpaceTimeIDSet, insert::select_dimensions::DimensionSelect},
};

impl SpaceTimeIDSet {
    /// 指定された次元において、自分を含む祖先のインデックスを収集する
    pub(crate) fn collect_ancestors(
        &self,
        main_bit: &BitVec,
        main_dim_select: &DimensionSelect,
    ) -> Vec<Index> {
        let dims = self.dims_btree(main_dim_select);

        let mut result = HashSet::new();

        for top in main_bit.ancestors() {
            if let Some(v) = dims.main.get(&top) {
                result.extend(v.index.iter().copied());
            }
        }

        Vec::from_iter(result)
    }
}
