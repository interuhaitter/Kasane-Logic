use crate::{id::space_id::constants::MAX_ZOOM_LEVEL, segment::Segment};

#[derive(Debug, Clone, PartialEq)]
pub struct EncodeSegment([u8; EncodeSegment::ARRAY_LENGTH]);

impl EncodeSegment {
    //固定長配列の場合に必要な長さを定義
    const ARRAY_LENGTH: usize = (MAX_ZOOM_LEVEL * 2).div_ceil(8);

    pub fn new(segment: Segment<u64>) -> Self {
        let mut result = vec![0u8; EncodeSegment::ARRAY_LENGTH];

        let bit_count = (z + 1) as u32;
        let mask = if bit_count >= 64 {
            u64::MAX
        } else {
            (1u64 << bit_count) - 1
        };
        let uxy = xy & mask;

        for now_z in (0..=z).rev() {
            let index = ((now_z) * 2 / 8) as usize;
            let in_index = now_z % 4;

            result[index] |= 1 << (7 - in_index * 2);

            let bit_position = z - now_z;
            if (uxy >> bit_position) & 1 != 0 {
                result[index] |= 1 << (6 - in_index * 2);
            }
        }

        BitVec::from_vec(result)
    }
}
