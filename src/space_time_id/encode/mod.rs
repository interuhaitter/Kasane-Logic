use crate::{
    encode_id::EncodeID,
    space_time_id::{
        SpaceTimeID,
        encode::segment::{segment_f, segment_xy},
    },
};

pub mod into_bitvec;
pub mod segment;

impl SpaceTimeID {
    fn to_encode(&self) -> Vec<EncodeID> {
        //セグメントに区切る
        let f_splited = segment_f(self.z, self.f);
        let x_splited = segment_xy(self.z, self.x);
        let y_splited = segment_xy(self.z, self.y);

        todo!()
    }
}
