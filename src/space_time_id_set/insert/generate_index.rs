use crate::space_time_id_set::{Index, SpaceTimeIdSet};

impl SpaceTimeIdSet {
    pub(crate) fn generate_index(&mut self) -> Index {
        self.index = self.index + 1;
        self.index.clone()
    }
}
