use std::f32::consts::{PI, TAU};

// use bevy::{math::IVec3, utils::HashSet};
use crate::typical::*;

use super::{BOARD_SIZE_X, BOARD_SIZE_Y};

// https://www.redblobgames.com/grids/circle-drawing/
// https://www.redblobgames.com/coordinates/axes-and-angles/#angles
// https://www.redblobgames.com/grids/line-drawing/

pub fn circle(centre: IVec3, radius: f32) -> HashSet<[i32; 3]> {
    let mut circle = HashSet::new();

    let [x, y, z] = centre.to_array();
    let [fx, fy, r] = [x as f32, y as f32, radius]; // r = x.5 looks better
    let r2 = r * r;

    let top = fy - r;
    let bot = fy + r;

    for y in f32::round(top) as i32..f32::round(bot) as i32 {
        let dy = y as f32 - fy;
        let dx = f32::sqrt(r2 - dy * dy);
        let left = f32::ceil(fx - dx);
        let right = f32::floor(fx + dx);

        for x in f32::round(left) as i32..f32::round(right) as i32 {
            if x >= 0 && y >= 0 && x <= BOARD_SIZE_X && y <= BOARD_SIZE_Y {
                circle.insert([x, y, z]);
            }
        }
    }
    circle
}

pub fn take_sector(
    angle: f32,
    width: f32,
    centre: &IVec3,
    circle: HashSet<[i32; 3]>,
    // permissiveness: f32,
) -> HashSet<[i32; 3]> {
    circle
        .into_iter()
        .filter(|v| {
            let angle_to_point = angle_between_2d(centre, &IVec3::from_array(*v));
            warn!("{:?}", angle_to_point);
            return angle_to_point >= (angle - width / 2.0)
                && angle_to_point <= (angle + width / 2.0);
        })
        .collect::<HashSet<_>>()
}

pub fn sector_facing(
    facing: Direction,
    centre: &IVec3,
    circle: HashSet<[i32; 3]>,
) -> HashSet<[i32; 3]> {
    let angle = crate::board::direction::COMPASS_DEGREES[facing as usize];
    // let angle = crate::board::direction::DIRECTION_RADIANS[facing as usize];
    warn!(angle);
    take_sector(angle, 90., centre, circle)
}

fn mod360(value: f32) -> f32 {
    (value + 360.) % 360.
}

fn modl(value: f32, modulo: f32) -> f32 {
    return ((value * modulo) + modulo) % modulo;
}

pub fn degrees_left(start_deg: f32, end_deg: f32) -> f32 {
    modl(end_deg - start_deg, 360.0)
}

pub fn degrees_right(start_deg: f32, end_deg: f32) -> f32 {
    modl(start_deg - end_deg, 360.0)
}

pub fn degrees_apart(start_deg: f32, end_deg: f32) -> f32 {
    f32::min(
        degrees_left(start_deg, end_deg),
        degrees_right(start_deg, end_deg),
    )
}

pub fn angle_between_2d(a: &IVec3, b: &IVec3) -> f32 {
    let (x1, y1, x2, y2) = (a.x as f32, a.y as f32, b.x as f32, b.y as f32);
    f32::atan2(x2 - x1, y2 - y1) * (180. / PI)
}

#[test]
fn sanity() {
    let o = IVec3::new(0, 0, 0);
    // assert_eq!(angle_between_2d(&o, &Direction::North.offset()), 0.);
    assert_eq!(angle_between_2d(&o, &Direction::East.offset()), 90.);
    // assert_eq!(angle_between_2d(&o, &Direction::South.offset()), 180.);
    assert_eq!(angle_between_2d(&o, &Direction::West.offset()), 270.);
}

// https://www.redblobgames.com/grids/line-drawing/
pub fn line(p0: IVec3, p1: IVec3) -> Vec<IVec3> {
    let mut points = vec![];
    let n = distance_between_2d(p0, p1);
    for step in 0..n {
        let t: f32 = if n == 0 { 0.0 } else { step as f32 / n as f32 };
        let p = p0.as_vec3().lerp(p1.as_vec3(), t).as_ivec3();
        points.push(p);
    }
    points
}

fn distance_between_2d(p0: IVec3, p1: IVec3) -> i32 {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;
    return i32::max(i32::abs(dx), i32::abs(dy));
}
