use crate::space_time_id_set::{SpaceTimeIDSet, insert::select_dimensions::DimensionSelect};

impl SpaceTimeIDSet {
    /// 二つのSpaceTimeIDSetを結合する
    pub fn union(&self, other: &SpaceTimeIDSet) -> SpaceTimeIDSet {
        // IDの個数が少ないほうを small、多いほうを large にする
        let (small, large) = if self.iter().len() <= other.iter().len() {
            (self, other)
        } else {
            (other, self)
        };

        // large をコピーして small の内容を挿入する
        let mut result = large.clone();
        for (_, reverse) in small.reverse.clone() {
            result.uncheck_insert(&reverse.f, &reverse.x, &reverse.y, &DimensionSelect::F);
        }

        result
    }
}
