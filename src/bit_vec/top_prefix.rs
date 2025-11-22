use crate::bit_vec::BitVec;

impl BitVec {
    /// self の全ての上位階層を列挙して返す
    ///
    /// 返される Vec は、上位階層の BitVecを順番に格納
    /// 例えば self = [10 11 10 00] の場合、返り値は [[10 00], [10 11 00],[10 11 10 00]] となる。
    pub fn ancestors(&self) -> Vec<BitVec> {
        let mut result: Vec<BitVec> = vec![];
        for byte in &self.0 {
            for i in 0..4 {
                let masked: u8 = byte & (0b11000000 >> 2 * i);

                if masked == 0 {
                    break;
                }

                match result.last() {
                    Some(v) => {
                        let mut copy = v.clone();
                        if i == 0 {
                            copy.0.push(masked);
                        } else if let Some(last) = copy.0.last_mut() {
                            *last |= masked;
                        }
                        result.push(copy);
                    }
                    None => result.push(BitVec(vec![masked])),
                }
            }
        }
        result
    }
}
