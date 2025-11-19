use crate::bit_vec::BitVec;

impl BitVec {
    /// (self, next_prefix)
    pub fn under_prefix(&self) -> (BitVec, BitVec) {
        if self.clone() > self.next_prefix() {
            println!("SELF  :{}", self.clone());
            println!("UNDER :{}", self.next_prefix());
            panic!()
        }
        (self.clone(), self.next_prefix())
    }

    pub fn next_prefix(&self) -> BitVec {
        let mut copyed = self.clone();

        // next_prefix 本体
        for (_byte_index, byte) in copyed.0.iter_mut().enumerate().rev() {
            for i in 0..=3 {
                let mask = 0b00000011 << (i * 2);
                let masked = *byte & mask;

                match masked >> (i * 2) {
                    0b10 => {
                        // 10 -> 11
                        *byte |= 0b01 << (i * 2);
                        // ----- ここで末尾の空バイト削除 -----
                        while let Some(&last) = copyed.0.last() {
                            if last == 0 {
                                copyed.0.pop();
                            } else {
                                break;
                            }
                        }
                        return copyed;
                    }
                    0b11 => {
                        // 11 -> 10
                        *byte ^= 0b11 << (i * 2);
                    }
                    _ => {}
                }
            }
        }

        // ----- ここでも末尾の空バイト削除 -----
        while let Some(&last) = copyed.0.last() {
            if last == 0 {
                copyed.0.pop();
            } else {
                break;
            }
        }

        copyed
    }
}
