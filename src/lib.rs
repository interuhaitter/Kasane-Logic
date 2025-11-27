//! # Kasane-logic
//!
//! 3次元空間+時間軸の時空間IDを扱うライブラリ
//!
//! ## 主要な機能
//!
//! - `SpaceTimeID`: 時空間IDの定義と操作
//! - `EncodeIDSet`: 時空間IDの集合を効率的に管理
//! - `Point`: 座標変換を提供するトレイト（緯度経度高度 ⇔ ECEF）
//! - `function`: 幾何形状（点、線、三角形）から時空間IDを生成
//!
//! ## 使用例
//!
//! ```no_run
//! use kasane_logic::encode_id_set::EncodeIDSet;
//! use kasane_logic::point::{Coordinate, Point};
//!
//! let mut set = EncodeIDSet::new();
//! let point = Coordinate {
//!     latitude: 35.6809,
//!     longitude: 139.7673,
//!     altitude: 0.0,
//! };
//! // IDを挿入
//! // set.insert(point.to_id(25).to_encode().first().unwrap().clone());
//! ```

/// ビット列を用いた階層構造の管理
pub mod bit_vec;

/// エラー型の定義
pub mod error;

/// 幾何形状から時空間IDを生成する関数群
pub mod function;

/// 座標変換を提供するトレイトと座標系の定義
pub mod point;

/// 時空間IDの定義と操作
pub mod space_time_id;

/// 時空間IDの集合を効率的に管理
pub mod encode_id_set;

pub mod encode_id;
pub mod macros;
