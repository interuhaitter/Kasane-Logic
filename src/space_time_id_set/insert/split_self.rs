use crate::{
    bit_vec::BitVec,
    space_time_id_set::{Index, SpaceTimeIDSet, insert::select_dimensions::DimensionSelect},
};
#[derive(Debug)]
pub struct RangesCollect {
    pub main: Vec<BitVec>,
    pub a: Vec<BitVec>,
    pub b: Vec<BitVec>,
}

impl SpaceTimeIDSet {
    ///下位,下位,上位の場合に自身を切断する
    pub(crate) fn split_self(
        &self,
        divison_collect: &mut RangesCollect,
        target_index: Index,
        target_dim_select: &DimensionSelect,
    ) {
        let reverse = self
            .reverse
            .get(&target_index)
            .expect("Internal error: reverse index not found in under_under_top");

        match target_dim_select {
            DimensionSelect::F => {
                divison_collect.main.push(reverse.f.clone());
            }
            DimensionSelect::X => {
                divison_collect.main.push(reverse.x.clone());
            }
            DimensionSelect::Y => {
                divison_collect.main.push(reverse.y.clone());
            }
        }
    }
}
