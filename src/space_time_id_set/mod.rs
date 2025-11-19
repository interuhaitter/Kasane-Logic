use std::collections::{BTreeMap, HashMap, HashSet};

use crate::bit_vec::BitVec;
pub mod single;

type Index = usize;
pub mod get_all;
pub mod insert;

/// 階層ごとの情報を保持する構造体
#[derive(Debug)]
pub struct LayerInfo {
    //その階層が持つ実際のIDのIndex
    pub index: HashSet<Index>,

    //その階層の下にあるIDの個数
    pub count: usize,
}

/// インデックスから各次元の情報を逆引きするための構造体
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ReverseInfo {
    pub f: BitVec,
    pub x: BitVec,
    pub y: BitVec,
}

/// 時空間IDの集合を効率的に管理するデータ構造
///
/// 重複する範囲を自動的に統合し、階層構造を用いて効率的に格納する。
/// 公開APIは`insert`と`get_all`のみ。
#[derive(Debug)]
pub struct SpaceTimeIdSet {
    //各次元の範囲を保存するためのBTreeMap
    f: BTreeMap<BitVec, LayerInfo>,
    x: BTreeMap<BitVec, LayerInfo>,
    y: BTreeMap<BitVec, LayerInfo>,
    index: usize,
    reverse: HashMap<Index, ReverseInfo>,
}
impl SpaceTimeIdSet {
    /// 新しい空の時空間ID集合を作成
    pub fn new() -> Self {
        Self {
            f: BTreeMap::new(),
            x: BTreeMap::new(),
            y: BTreeMap::new(),
            index: 0,
            reverse: HashMap::new(),
        }
    }
}
