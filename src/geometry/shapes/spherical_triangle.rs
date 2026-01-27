use std::collections::HashSet;

use crate::geometry::coordinate::Coordinate;
use crate::spatial_id::single::SingleId;

/// Webメルカトル平面上の点
#[derive(Debug, Clone, Copy)]
struct Point2 {
    x: f64,
    y: f64,
}

/// 小数点切り捨てしない
fn project(coord: &Coordinate, z: u8) -> Point2 {
    let n = 2f64.powi(z as i32);

    let x = (coord.as_longitude() + 180.0) / 360.0 * n;

    let lat_rad = coord.as_latitude().to_radians();
    let y = (1.0
        - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln()
            / std::f64::consts::PI)
        / 2.0
        * n;

    Point2 { x, y }
}

/// 2D三角形
#[derive(Debug)]
struct Triangle2 {
    a: Point2,
    b: Point2,
    c: Point2,
}

impl Triangle2 {
    fn bounding_box(&self) -> (i32, i32, i32, i32) {
        let min_x = self.a.x.min(self.b.x).min(self.c.x).floor() as i32;
        let max_x = self.a.x.max(self.b.x).max(self.c.x).ceil() as i32;
        let min_y = self.a.y.min(self.b.y).min(self.c.y).floor() as i32;
        let max_y = self.a.y.max(self.b.y).max(self.c.y).ceil() as i32;
        (min_x, max_x, min_y, max_y)
    }
}

/// 点が三角形内にあるか
fn point_in_triangle(p: Point2, t: &Triangle2) -> bool {
    fn sign(p1: Point2, p2: Point2, p3: Point2) -> f64 {
        (p1.x - p3.x) * (p2.y - p3.y)
            - (p2.x - p3.x) * (p1.y - p3.y)
    }

    let d1 = sign(p, t.a, t.b);
    let d2 = sign(p, t.b, t.c);
    let d3 = sign(p, t.c, t.a);

    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;

    !(has_neg && has_pos)
}

fn segments_intersect(
    a1: Point2,
    a2: Point2,
    b1: Point2,
    b2: Point2,
) -> bool {
    fn orient(a: Point2, b: Point2, c: Point2) -> f64 {
        (b.x - a.x) * (c.y - a.y)
            - (b.y - a.y) * (c.x - a.x)
    }

    fn on_segment(a: Point2, b: Point2, p: Point2) -> bool {
        p.x >= a.x.min(b.x)
            && p.x <= a.x.max(b.x)
            && p.y >= a.y.min(b.y)
            && p.y <= a.y.max(b.y)
    }

    let o1 = orient(a1, a2, b1);
    let o2 = orient(a1, a2, b2);
    let o3 = orient(b1, b2, a1);
    let o4 = orient(b1, b2, a2);

    if o1 * o2 < 0.0 && o3 * o4 < 0.0 {
        return true;
    }

    if o1 == 0.0 && on_segment(a1, a2, b1) { return true; }
    if o2 == 0.0 && on_segment(a1, a2, b2) { return true; }
    if o3 == 0.0 && on_segment(b1, b2, a1) { return true; }
    if o4 == 0.0 && on_segment(b1, b2, a2) { return true; }

    false
}

/// タイル（1×1正方形）が三角形と交差するか
fn tile_intersects_triangle(tx: i32, ty: i32, tri: &Triangle2) -> bool {
    let tile = [
        Point2 { x: tx as f64, y: ty as f64 },
        Point2 { x: tx as f64 + 1.0, y: ty as f64 },
        Point2 { x: tx as f64 + 1.0, y: ty as f64 + 1.0 },
        Point2 { x: tx as f64, y: ty as f64 + 1.0 },
    ];

    // 1. タイル頂点が三角形内
    for &p in &tile {
        if point_in_triangle(p, tri) {
            return true;
        }
    }

    // 2. 三角形頂点がタイル内
    let in_tile = |p: Point2| {
        p.x >= tile[0].x && p.x <= tile[2].x &&
        p.y >= tile[0].y && p.y <= tile[2].y
    };

    if in_tile(tri.a) || in_tile(tri.b) || in_tile(tri.c) {
        return true;
    }

    // 3. 辺交差判定
    let tri_edges = [
        (tri.a, tri.b),
        (tri.b, tri.c),
        (tri.c, tri.a),
    ];

    let tile_edges = [
        (tile[0], tile[1]),
        (tile[1], tile[2]),
        (tile[2], tile[3]),
        (tile[3], tile[0]),
    ];

    for (ta, tb) in tri_edges {
        for (sa, sb) in tile_edges {
            if segments_intersect(ta, tb, sa, sb) {
                return true;
            }
        }
    }

    false
}

/// Iterator 本体
pub struct CoverSphericalTriangleIter {
    z: u8,
    tri: Triangle2,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    cur_x: i32,
    cur_y: i32,
}

impl Iterator for CoverSphericalTriangleIter {
    type Item = SingleId;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur_y <= self.max_y {
            while self.cur_x <= self.max_x {
                let x = self.cur_x;
                let y = self.cur_y;
                self.cur_x += 1;

                if tile_intersects_triangle(x, y, &self.tri) {
                    return Some(unsafe {
                        SingleId::uncheck_new(
                            self.z,
                            0,
                            x as u32,
                            y as u32,
                        )
                    });
                }
            }
            self.cur_x = self.min_x;
            self.cur_y += 1;
        }
        None
    }
}

/// 球面三角形で覆われるSingleIdをIteratorとして返す**
pub fn spherical_triangle(
    a: Coordinate,
    b: Coordinate,
    c: Coordinate,
    z: u8,
) -> CoverSphericalTriangleIter {
    let pa = project(&a, z);
    let pb = project(&b, z);
    let pc = project(&c, z);

    let tri = Triangle2 { a: pa, b: pb, c: pc };

    let (min_x, max_x, min_y, max_y) = tri.bounding_box();

    CoverSphericalTriangleIter {
        z,
        tri,
        min_x,
        max_x,
        min_y,
        max_y,
        cur_x: min_x,
        cur_y: min_y,
    }
}
