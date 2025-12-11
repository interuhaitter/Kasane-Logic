use std::f64::consts::PI;

// src/id/space_id/helpers.rs
use crate::error::Error;

/// Checked add for i64 with range check; returns Err on overflow or out-of-range.
pub fn checked_add_f_in_range(
    value: i64,
    by: i64,
    min: i64,
    max: i64,
    z: u8,
) -> Result<i64, Error> {
    let new = value.checked_add(by).ok_or(Error::FOutOfRange {
        f: if by >= 0 { i64::MAX } else { i64::MIN },
        z,
    })?;

    if new < min || new > max {
        return Err(Error::FOutOfRange { f: new, z });
    }
    Ok(new)
}

/// Checked sub for i64 with range check; convenience wrapper (subtraction is add of -by).
pub fn checked_sub_f_in_range(
    value: i64,
    by: i64,
    min: i64,
    max: i64,
    z: u8,
) -> Result<i64, Error> {
    checked_add_f_in_range(value, -by, min, max, z)
}

/// Checked add for u64 with range check; returns Err on overflow or out-of-range.
/// `field_name_err` is a closure that creates the appropriate Error variant; we use a generic X/Y variant in caller.
pub fn checked_add_u64_in_range(
    value: u64,
    by: u64,
    max: u64,
    err: impl Fn(u64) -> Error,
) -> Result<u64, Error> {
    let new = value.checked_add(by).ok_or_else(|| err(u64::MAX))?;
    if new > max {
        return Err(err(new));
    }
    Ok(new)
}

/// Checked sub for u64 with range check; returns Err on underflow or out-of-range.
pub fn checked_sub_u64_in_range(
    value: u64,
    by: u64,
    max: u64,
    err: impl Fn(u64) -> Error,
) -> Result<u64, Error> {
    let new = value.checked_sub(by).ok_or_else(|| err(0))?;
    if new > max {
        return Err(err(new));
    }
    Ok(new)
}

/// Wrap-add for u64 in range [0..=max] (ringsize = max + 1)
pub fn wrap_add_u64(value: u64, by: u64, max: u64) -> u64 {
    let ring = max.wrapping_add(1);
    if ring == 0 {
        // max == u64::MAX -> ring == 0 (wrap), fall back to wrapping_add only
        return value.wrapping_add(by);
    }
    (value.wrapping_add(by)) % ring
}

/// Wrap-add for i64 in range [min..=max]
pub fn wrap_add_i64_in_range(value: i64, by: i64, min: i64, max: i64) -> i64 {
    // Convert to offset from min, operate in unsigned modulo, convert back.
    let width = (max - min).wrapping_add(1) as i128; // use wider temp to avoid overflow
    if width <= 0 {
        // degenerate: single-value range
        return min;
    }

    let offset = (value - min) as i128;
    let by_offset = by as i128;

    let new_offset = ((offset + by_offset) % width + width) % width; // canonical positive modulo
    (min as i128 + new_offset) as i64
}

/// Scale an inclusive range `[start, end]` by `scale` for children calculation.
/// For integer types, result is `[start*scale, end*scale + scale - 1]`
pub fn scale_range_i64(start: i64, end: i64, scale: i64) -> [i64; 2] {
    [
        start.saturating_mul(scale),
        end.saturating_mul(scale).saturating_add(scale - 1),
    ]
}

pub fn scale_range_u64(start: u64, end: u64, scale: u64) -> [u64; 2] {
    [
        start.saturating_mul(scale),
        end.saturating_mul(scale).saturating_add(scale - 1),
    ]
}

/// n = 2^z を返す
fn n_from_z(z: u8) -> f64 {
    2_f64.powi(z as i32)
}

/// 経度 (longitude) を返す（実数 x 対応）
///
/// x: 水平方向のタイル/セル座標（連続値）  
/// z: ズームレベル  
///
/// セル番号 x の左端なら x、中心なら x+0.5 を渡せる。
pub fn longitude(x: f64, z: u8) -> f64 {
    let n = n_from_z(z);
    360.0 * (x / n) - 180.0
}

/// 緯度 (latitude) を返す（Web Mercator の逆変換, 実数 y 対応）
///
/// y: 垂直方向のタイル/セル座標（連続値）  
/// z: ズームレベル  
///
/// 公式: lat = atan( sinh( π * (1 - 2*y/n) ) )
pub fn latitude(y: f64, z: u8) -> f64 {
    let n = n_from_z(z);
    let t = PI * (1.0 - 2.0 * (y / n));
    let lat_rad = t.sinh().atan();
    lat_rad.to_degrees()
}

/// 高度 (altitude) を返す（実数 f 対応）
///
/// f: 高度方向 index（連続値）  
/// z: ズームレベル  
///
pub fn altitude(f: f64, z: u8) -> f64 {
    let n = n_from_z(z);
    33_554_432.0 * (f / n)
}
