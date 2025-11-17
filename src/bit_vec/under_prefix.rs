use crate::bit_vec::BitVec;

impl BitVec {
    /// (self, next_prefix)
    pub fn under_prefix(&self) -> (BitVec, BitVec) {
        (self.clone(), self.next_prefix())
    }

    pub fn next_prefix(&self) -> BitVec {
        let mut copyed = self.clone();
        let len = copyed.0.len();

        // 全ての分岐Bitが 11 の場合のみ true のまま残る
        let mut all_one = true;

        // まず "全ての分岐Bitが 11 かどうか" を判定
        for (byte_index, byte) in self.0.iter().enumerate().rev() {
            for i in 0..=3 {
                let mask = 0b00000011 << (i * 2);

                // 最後の2bit(i == 0) もここでは判定に含める
                if (byte & mask) >> (i * 2) != 0b11 {
                    all_one = false;
                    break;
                }
            }
            if !all_one {
                break;
            }
        }

        // ここから next_prefix 本体
        for (byte_index, byte) in copyed.0.iter_mut().enumerate().rev() {
            for i in 0..=3 {
                // 最後の2bit（i == 0）だけ特別処理
                if byte_index == len - 1 && i == 0 {
                    if all_one {
                        // 全て 11 のときだけ 11 -> 10 に変える
                        *byte = (*byte & !(0b11)) | 0b10;
                    }
                    continue;
                }

                let mask = 0b00000011 << (i * 2);
                let masked = *byte & mask;

                match masked >> (i * 2) {
                    0b10 => {
                        // 10 -> 11
                        *byte |= 0b01 << (i * 2);
                        return copyed;
                    }
                    0b11 => {
                        // 11 -> 10
                        *byte ^= 0b11 << (i * 2);
                        // → 続行して上位で処理させる
                    }
                    _ => {}
                }
            }
        }

        copyed
    }
}
