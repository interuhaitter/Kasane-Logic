//! 座標系の定義と変換
//!
//! - `Point`: 座標変換を提供するトレイト
//! - `Coordinate`: 緯度経度高度座標
//! - `ECEF`: 地球中心座標系

mod constants;
pub mod coordinate;
pub mod ecef;

pub use coordinate::Coordinate;
pub use ecef::ECEF;

use crate::space_time_id::SpaceTimeID;

/// 座標変換を提供するトレイト
///
/// 様々な座標系をこのトレイトで統一的に扱うことができます。
pub trait Point: Clone + Copy {
    /// Coordinate形式に変換
    fn to_coordinate(&self) -> Coordinate;

    /// ECEF形式に変換
    fn to_ecef(&self) -> ECEF;

    /// 指定されたズームレベルで時空間IDに変換
    fn to_id(&self, z: u8) -> SpaceTimeID {
        self.to_coordinate().to_id(z)
    }
}
