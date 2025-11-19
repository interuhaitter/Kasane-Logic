use std::collections::BTreeMap;

use crate::{
    bit_vec::BitVec,
    space_time_id_set::{LayerInfo, SpaceTimeIdSet, insert::insert_main_dim::DimensionSelect},
};

pub struct DimensionRefs<'a> {
    pub main: &'a BTreeMap<BitVec, LayerInfo>,
    pub others: [&'a BTreeMap<BitVec, LayerInfo>; 2],
}

pub struct DimensionReverseRefs<'a> {
    pub main: &'a BTreeMap<BitVec, LayerInfo>,
    pub others: [&'a BTreeMap<BitVec, LayerInfo>; 2],
}

impl SpaceTimeIdSet {
    /// メイン次元とその他の次元の参照を選択
    pub(crate) fn select_dimensions(&self, dim: &DimensionSelect) -> DimensionRefs<'_> {
        match dim {
            DimensionSelect::F => DimensionRefs {
                main: &self.f,
                others: [&self.x, &self.y],
            },
            DimensionSelect::X => DimensionRefs {
                main: &self.x,
                others: [&self.f, &self.y],
            },
            DimensionSelect::Y => DimensionRefs {
                main: &self.y,
                others: [&self.f, &self.x],
            },
        }
    }
}
