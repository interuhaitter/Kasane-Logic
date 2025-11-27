use std::collections::HashSet;

use itertools::iproduct;

use crate::{
    bit_vec::relation::BitVecRelation,
    encode_id::EncodeID,
    encode_id_set::{
        EncodeIDSet, Index, insert::split_self::RangesCollect,
        utils::select_dimensions::DimensionSelect,
    },
};

impl EncodeIDSet {
    ///EncodeIDSetから指定されたEncodeIDを削除する。
    /// 既存の範囲と重複がある場合は削除時に調整が行われ、削除されたIDが返される。
    pub fn remove(&mut self, encode_id: EncodeID) -> Vec<EncodeID> {
        let mut result = vec![];

        //下位IDの個数が最小の次元を選択する
        let f_descendants_count = match self.f.get(&encode_id.f) {
            Some(info) => info.count,
            None => 0,
        };

        let x_descendants_count = match self.x.get(&encode_id.x) {
            Some(info) => info.count,
            None => 0,
        };

        let y_descendants_count = match self.y.get(&encode_id.y) {
            Some(info) => info.count,
            None => 0,
        };

        let min_count = f_descendants_count.min(x_descendants_count.min(y_descendants_count));

        //代表の次元を選出する
        let main_dim;
        let a_dim;
        let b_dim;
        let main;
        let a;
        let b;

        if min_count == f_descendants_count {
            main_dim = DimensionSelect::F;
            a_dim = DimensionSelect::X;
            b_dim = DimensionSelect::Y;
            main = &encode_id.f;
            a = &encode_id.x;
            b = &encode_id.y;
        } else if min_count == x_descendants_count {
            main_dim = DimensionSelect::X;
            a_dim = DimensionSelect::F;
            b_dim = DimensionSelect::Y;
            main = &encode_id.x;
            a = &encode_id.f;
            b = &encode_id.y;
        } else {
            main_dim = DimensionSelect::Y;
            a_dim = DimensionSelect::F;
            b_dim = DimensionSelect::X;
            main = &encode_id.y;
            a = &encode_id.f;
            b = &encode_id.x;
        }

        //Main次元の祖先を探索する
        let main_ancestors: Vec<Index> = Self::collect_ancestors(&self, main, &main_dim);

        //Main次元において、祖先にも子孫にも重なる範囲が存在しなければ何も削除しない
        if main_ancestors.is_empty() && min_count == 0 {
            return result;
        }

        //Main次元における祖先のIndexを取得する
        let mut main_ancestors_reverse = vec![];

        //Main次元における祖先を逆引き情報で取得する
        for ancestor_index in &main_ancestors {
            main_ancestors_reverse.push(
                self.reverse
                    .get(&*ancestor_index)
                    .expect("Internal error: reverse index not found for under"),
            );
        }

        //Main次元における子孫のIndexを取得する
        let main_descendants: Vec<Index> = self.collect_descendants(main, &main_dim);

        //Main次元における子孫を逆引き情報で取得する
        let mut main_descendants_reverse = vec![];
        for descendant_index in &main_descendants {
            main_descendants_reverse.push(
                self.reverse
                    .get(&descendant_index)
                    .expect("Internal error: reverse index not found for top"),
            );
        }

        let a_relation = match Self::collect_other_dimension(
            a,
            &main_ancestors_reverse,
            &main_descendants_reverse,
            &a_dim,
        ) {
            Some(v) => v,
            None => {
                return result;
            }
        };

        let b_relation = match Self::collect_other_dimension(
            b,
            &main_ancestors_reverse,
            &main_descendants_reverse,
            &b_dim,
        ) {
            Some(v) => v,
            None => {
                return result;
            }
        };

        let mut need_delete: HashSet<Index> = HashSet::new();
        let mut need_insert: HashSet<EncodeID> = HashSet::new();

        let mut collect_divison_ranges = RangesCollect {
            f: vec![],
            x: vec![],
            y: vec![],
        };

        //Main次元における祖先の範囲を調べる
        for (i, (a_rel, b_rel)) in a_relation.0.iter().zip(b_relation.0.iter()).enumerate() {
            match (a_rel, b_rel) {
                (
                    BitVecRelation::Descendant | BitVecRelation::Equal,
                    BitVecRelation::Descendant | BitVecRelation::Equal,
                ) => {
                    //全ての次元において既存IDが削除対象を含むため、既存IDを分割して削除対象以外を残す
                    self.split_other(
                        &main_ancestors[i],
                        main_ancestors_reverse[i],
                        main,
                        &main_dim,
                        &mut need_delete,
                        &mut need_insert,
                    );
                }
                (BitVecRelation::Descendant | BitVecRelation::Equal, BitVecRelation::Ancestor) => {
                    //A次元で既存IDが削除対象を含み、B次元で削除対象が既存IDを含む
                    self.split_other(
                        &main_ancestors[i],
                        main_ancestors_reverse[i],
                        b,
                        &b_dim,
                        &mut need_delete,
                        &mut need_insert,
                    );
                }
                (BitVecRelation::Ancestor, BitVecRelation::Descendant | BitVecRelation::Equal) => {
                    //A次元で削除対象が既存IDを含み、B次元で既存IDが削除対象を含む
                    self.split_other(
                        &main_ancestors[i],
                        main_ancestors_reverse[i],
                        a,
                        &a_dim,
                        &mut need_delete,
                        &mut need_insert,
                    );
                }
                (BitVecRelation::Ancestor, BitVecRelation::Ancestor) => {
                    //全ての次元において祖先のIDが存在するため、削除対象全体が既存IDに含まれる
                    //既存IDを分割して削除対象を除外する
                    need_delete.insert(main_ancestors[i]);

                    //削除対象を除外した部分を追加する
                    self.split_self(
                        main_ancestors_reverse[i],
                        &mut collect_divison_ranges,
                        &main_dim,
                    );
                    self.split_self(
                        main_ancestors_reverse[i],
                        &mut collect_divison_ranges,
                        &a_dim,
                    );
                    self.split_self(
                        main_ancestors_reverse[i],
                        &mut collect_divison_ranges,
                        &b_dim,
                    );

                    result.push(encode_id.clone());

                    //分割後の範囲を挿入する
                    let f_splited = main_ancestors_reverse[i]
                        .f
                        .subtract_ranges(&collect_divison_ranges.f);
                    let x_splited = main_ancestors_reverse[i]
                        .x
                        .subtract_ranges(&collect_divison_ranges.x);
                    let y_splited = main_ancestors_reverse[i]
                        .y
                        .subtract_ranges(&collect_divison_ranges.y);

                    for (f, x, y) in iproduct!(f_splited, x_splited, y_splited) {
                        need_insert.insert(EncodeID { f, x, y });
                    }

                    //既存IDのうち、削除すべきものを削除する
                    for need_delete_index in need_delete {
                        self.uncheck_delete(&need_delete_index);
                    }

                    //分割後の範囲を挿入する
                    for id in need_insert {
                        self.uncheck_insert(id);
                    }

                    return result;
                }
                _ => {}
            }
        }

        //Main次元における子孫の範囲について調べる
        for (i, (a_rel, b_rel)) in a_relation.1.iter().zip(b_relation.1.iter()).enumerate() {
            match (a_rel, b_rel) {
                (
                    BitVecRelation::Descendant | BitVecRelation::Equal,
                    BitVecRelation::Descendant | BitVecRelation::Equal,
                ) => {
                    //全ての次元において子孫のIDが存在するため完全削除
                    need_delete.insert(main_descendants[i]);
                    result.push(
                        self.reverse
                            .get(&main_descendants[i])
                            .expect("Internal error: reverse index not found")
                            .clone(),
                    );
                }
                (BitVecRelation::Descendant | BitVecRelation::Equal, BitVecRelation::Ancestor) => {
                    //A次元で既存IDが削除対象を含み、B次元で削除対象が既存IDを含む
                    self.split_other(
                        &main_descendants[i],
                        main_descendants_reverse[i],
                        b,
                        &b_dim,
                        &mut need_delete,
                        &mut need_insert,
                    );
                }
                (BitVecRelation::Ancestor, BitVecRelation::Descendant | BitVecRelation::Equal) => {
                    //A次元で削除対象が既存IDを含み、B次元で既存IDが削除対象を含む
                    self.split_other(
                        &main_descendants[i],
                        main_descendants_reverse[i],
                        a,
                        &a_dim,
                        &mut need_delete,
                        &mut need_insert,
                    );
                }
                (BitVecRelation::Ancestor, BitVecRelation::Ancestor) => {
                    //全ての次元において削除対象が既存IDを含む - 完全削除
                    need_delete.insert(main_descendants[i]);
                    result.push(
                        self.reverse
                            .get(&main_descendants[i])
                            .expect("Internal error: reverse index not found")
                            .clone(),
                    );
                }
                _ => {}
            }
        }

        //既存IDのうち、削除すべきものを削除する
        for need_delete_index in need_delete {
            self.uncheck_delete(&need_delete_index);
        }

        //既存IDを分割した場合に、分割後の範囲を挿入する
        for id in need_insert {
            self.uncheck_insert(id);
        }

        result
    }
}
