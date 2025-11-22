use crate::bit_vec::BitVec;

/// - `Ancestor`  
///     - `self` が `other` を包含する上位世代である
///
/// - `Equal`  
///     - `self` と `other` が同じ世代・範囲である
///
/// - `Descendant`  
///     - `self` が `other` の下位世代である
///
/// - `Unrelated`  
///     - `self` と `other` に世代的な包含関係がない
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitVecRelation {
    /// self が other を包含する上位世代
    Ancestor,

    /// self と other が同じ世代・範囲
    Equal,

    /// self が other の下位世代
    Descendant,

    /// 世代的に無関係
    Unrelated,
}

impl BitVec {
    /// self と other の世代（ancestor/descendant）関係を返す
    pub fn relation(&self, other: &Self) -> BitVecRelation {
        let self_upper = self.upper_bound();
        let other_upper = other.upper_bound();

        // Same: 完全一致
        if self == other {
            return BitVecRelation::Equal;
        }

        // Ancestor: self が other を包含
        if self < other && other < &self_upper {
            return BitVecRelation::Ancestor;
        }

        // Descendant: self が other の下位
        if other < self && self < &other_upper {
            return BitVecRelation::Descendant;
        }

        // それ以外は無関係
        BitVecRelation::Unrelated
    }
}
