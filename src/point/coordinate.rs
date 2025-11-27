#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-rs")]
use ts_rs::TS;

use crate::{error::Error, space_time_id::SpaceTimeID};

use super::{constants::{WGS84_A, WGS84_INV_F}, ecef::ECEF, Point};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "ts-rs", derive(TS))]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

impl Point for Coordinate {
    fn to_coordinate(&self) -> Coordinate {
        *self
    }

    fn to_ecef(&self) -> ECEF {
        let f = 1.0 / WGS84_INV_F;
        let b = WGS84_A * (1.0 - f);
        let e2 = 1.0 - (b * b) / (WGS84_A * WGS84_A);

        let lat = self.latitude.to_radians();
        let lon = self.longitude.to_radians();
        let h = self.altitude;

        let sin_lat = lat.sin();
        let cos_lat = lat.cos();
        let cos_lon = lon.cos();
        let sin_lon = lon.sin();

        let n = WGS84_A / (1.0 - e2 * sin_lat * sin_lat).sqrt();

        let x_f64 = (n + h) * cos_lat * cos_lon;
        let y_f64 = (n + h) * cos_lat * sin_lon;
        let z_f64 = (n * (1.0 - e2) + h) * sin_lat;

        ECEF {
            x: x_f64,
            y: y_f64,
            z: z_f64,
        }
    }

    fn to_id(&self, z: u8) -> SpaceTimeID {
        let lat = self.latitude;
        let lon = self.longitude;
        let alt = self.altitude;

        // ---- 高度 h -> f (Python の h_to_f を Rust に移植) ----
        let factor = 2_f64.powi(z as i32 - 25); // 2^(z-25)
        let f_id = (factor * alt).floor() as i64;

        // ---- 経度 lon -> x ----
        let n = 2u64.pow(z as u32) as f64;
        let x_id = ((lon + 180.0) / 360.0 * n).floor() as u64;

        // ---- 緯度 lat -> y (Web Mercator) ----
        let lat_rad = lat.to_radians();
        let y_id = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0
            * n)
            .floor() as u64;

        SpaceTimeID {
            z,
            f: [f_id, f_id],
            x: [x_id, x_id],
            y: [y_id, y_id],
        }
    }
}

impl Coordinate {
    /// 新しい座標を作成（範囲チェック付き）
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Result<Self, Error> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(Error::LatitudeOutOfRange { latitude });
        }

        if !(-180.0..=180.0).contains(&longitude) {
            return Err(Error::LongitudeOutOfRange { longitude });
        }

        if !(-33_554_432.0..=33_554_432.0).contains(&altitude) {
            return Err(Error::AltitudeOutOfRange { altitude });
        }

        Ok(Self {
            latitude,
            longitude,
            altitude,
        })
    }
}
