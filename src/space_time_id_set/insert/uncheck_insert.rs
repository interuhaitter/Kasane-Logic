use std::collections::{BTreeMap, HashSet};

use crate::{
    bit_vec::BitVec,
    space_time_id_set::{Interval, LayerInfo, ReverseInfo, SpaceTimeIdSet},
};

impl SpaceTimeIdSet {
    pub(crate) fn uncheck_insert(&mut self, f: &BitVec, x: &BitVec, y: &BitVec) {
        let index = self.generate_index();

        Self::update_layer(&mut self.f, f, index);
        Self::update_layer(&mut self.x, x, index);
        Self::update_layer(&mut self.y, y, index);

        self.reverse.insert(
            index,
            ReverseInfo {
                f: f.clone(),
                x: x.clone(),
                y: y.clone(),
            },
        );
    }

    ///上位の階層のcountに+1
    fn update_layer(map: &mut BTreeMap<BitVec, LayerInfo>, key: &BitVec, index: usize) {
        for key_top in key.ancestors() {
            if key_top == *key {
                map.entry(key_top)
                    .and_modify(|v| {
                        v.count += 1;
                        v.index.insert(index);
                    })
                    .or_insert(LayerInfo {
                        index: HashSet::from([index]),
                        count: 1,
                    });
            } else {
                map.entry(key_top)
                    .and_modify(|v| {
                        v.count += 1;
                    })
                    .or_insert(LayerInfo {
                        index: HashSet::from([]),
                        count: 1,
                    });
            }
        }
    }
}
