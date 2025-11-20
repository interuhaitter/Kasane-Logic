use crate::space_time_id_set::{Interval, SpaceTimeIdSet};

///自分が相手に対してどの位置にいるのか
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TimeRelation {
    // 自分が相手を完全に包む
    Top,

    //相手が完全に自分を包む
    Under([Option<(u64, u64)>; 2]),
    Cross(u64, u64),
    Disjoint,
}

impl SpaceTimeIdSet {
    pub(crate) fn time_relation(me: &Interval, target: &Interval) -> TimeRelation {
        let me_start = me.t1;
        let me_end = me.t2;
        let target_start = target.t1;
        let target_end = target.t2;

        // me が target を完全に包む（Top）
        if me_start <= target_start && me_end >= target_end {
            TimeRelation::Top
        }
        // target が me を完全に包む（Under）
        else if target_start <= me_start && target_end >= me_end {
            // target から me を引いた範囲を計算
            let mut diff = [None, None];
            if target_start < me_start {
                diff[0] = Some((target_start, me_start)); // 左側の差分
            }
            if me_end < target_end {
                diff[1] = Some((me_end, target_end)); // 右側の差分
            }
            TimeRelation::Under(diff)
        }
        // 部分的に重なる場合（Cross）
        else if me_end >= target_start && me_start <= target_end {
            let overlap_start = me_start.max(target_start);
            let overlap_end = me_end.min(target_end);
            TimeRelation::Cross(overlap_start, overlap_end)
        }
        // 重ならない場合
        else {
            TimeRelation::Disjoint
        }
    }
}
