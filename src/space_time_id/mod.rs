use serde::Serialize;
pub mod format;
pub mod random;
pub mod z_range;

use crate::{
    error::Error,
    space_time_id::z_range::{F_MAX, F_MIN, XY_MAX},
};

/// 時空間ID（4次元：3次元空間+時間）
///
/// 各次元は範囲として表現され、ズームレベルによって解像度が決まる
/// - z: ズームレベル (0-60)
/// - f: 高度方向の範囲 (符号付き整数)
/// - x: 経度方向の範囲 (符号なし整数)
/// - y: 緯度方向の範囲 (符号なし整数)
/// - i: インデックス
/// - t: 時間範囲
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpaceTimeId {
    pub z: u8,
    pub f: [i64; 2],
    pub x: [u64; 2],
    pub y: [u64; 2],
    pub i: u32,
    pub t: [u64; 2],
}

impl SpaceTimeId {
    /// 新しい時空間IDを作成
    ///
    /// 各次元の値の範囲を確認・正規化する。
    /// None を指定すると、その次元の最小値または最大値が使用される。
    pub fn new(
        z: u8,
        f: [Option<i64>; 2],
        x: [Option<u64>; 2],
        y: [Option<u64>; 2],
        i: u32,
        t: [Option<u32>; 2],
    ) -> Result<Self, Error> {
        if z > 60 {
            return Err(Error::ZoomLevelOutOfRange { zoom_level: z });
        }

        let f_max = F_MAX[z as usize];
        let f_min = F_MIN[z as usize];
        let xy_max = XY_MAX[z as usize];

        // 空間の次元を全て値に変換
        let new_f = normalize_dimension(f, f_min, f_max, valid_range_f, z)?;
        let new_x = normalize_dimension(x, 0, xy_max, valid_range_x, z)?;
        let new_y = normalize_dimension(y, 0, xy_max, valid_range_y, z)?;

        //時間軸の順番を入れ替え
        //終点はu64::MAXを用いて表現
        let new_t = if i == 0 {
            [0, u64::MAX]
        } else {
            match t {
                [None, None] => [0, u64::MAX],
                [None, Some(e)] => [0, (e as u64) * (i as u64) - 1],
                [Some(s), None] => [(s as u64) * (i as u64), u64::MAX],
                [Some(s), Some(e)] => {
                    let start = s.min(e) as u64;
                    let end = s.max(e) as u64;
                    [start * (i as u64), end * (i as u64) - 1]
                }
            }
        };

        Ok(SpaceTimeId {
            z,
            f: new_f,
            x: new_x,
            y: new_y,
            i,
            t: new_t,
        })
    }
}

///次元の値が正しいかを判定するパッチ関数
fn normalize_dimension<T>(
    dim: [Option<T>; 2],
    min: T,
    max: T,
    validate: impl Fn(T, T, T, u8) -> Result<(), Error>,
    z: u8,
) -> Result<[T; 2], Error>
where
    T: PartialOrd + Copy,
{
    //値が範囲内なのかをチェックする
    if let Some(s) = dim[0] {
        validate(s, min, max, z)?;
    }
    if let Some(e) = dim[1] {
        validate(e, min, max, z)?;
    }

    //値を変換して代入する
    let start = match dim[0] {
        Some(v) => v,
        None => min,
    };

    let end = match dim[1] {
        Some(v) => v,
        None => max,
    };

    //順序を正しくする
    if end > start {
        Ok([start, end])
    } else {
        Ok([end, start])
    }
}

///Fの範囲が正しいかを確認する
fn valid_range_f(num: i64, min: i64, max: i64, z: u8) -> Result<(), Error> {
    if (min..=max).contains(&num) {
        Ok(())
    } else {
        Err(Error::FOutOfRange { f: num, z })
    }
}

///Xの範囲が正しいかを確認する
fn valid_range_x(num: u64, min: u64, max: u64, z: u8) -> Result<(), Error> {
    if (min..=max).contains(&num) {
        Ok(())
    } else {
        Err(Error::XOutOfRange { x: num, z })
    }
}

///Yの範囲が正しいかを確認する
fn valid_range_y(num: u64, min: u64, max: u64, z: u8) -> Result<(), Error> {
    if (min..=max).contains(&num) {
        Ok(())
    } else {
        Err(Error::YOutOfRange { y: num, z })
    }
}
