use crate::{
    bit_vec::BitVec,
    space_id::{range::RangeID, segment::Segment, single::SingleID},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct EncodeID {
    pub(crate) f: Vec<BitVec>,
    pub(crate) x: Vec<BitVec>,
    pub(crate) y: Vec<BitVec>,
}

impl EncodeID {
    pub fn to_range_id(&self) -> RangeID {
        todo!()
    }

    pub fn to_single_id(&self) {
        todo!()
    }
}
