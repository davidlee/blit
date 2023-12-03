// #![allow(dead_code)]

use crate::typical::*;
#[allow(unused_imports)]
use crate::{BOARD_SIZE_X, BOARD_SIZE_Y};

// https://www.roguebasin.com/index.php?title=FOV_using_recursive_shadowcasting
// https://www.roguebasin.com/index.php?title=Comparative_study_of_field_of_view_algorithms_for_2D_grid_based_worlds

// https://www.albertford.com/shadowcasting/
// - https://github.com/370417/symmetric-shadowcasting/

type DepthColVec = IVec2;
type XyVec = IVec2;

// TODO add bounds
pub fn compute_fov_2d_recursive<'a>(
    origin: [i32; 2],
    walls: &'a HashSet<[i32; 2]>,
) -> Vec<[i32; 2]> {
    let mut visible = vec![];
    visible.push(origin);

    for cardinal in CARDINALS {
        let quadrant: Quadrant = Quadrant::new(cardinal, &IVec2::from_array(origin));
        let first_row = Row::new(1, -1.0, 1.0);
        visible.append(&mut scan_row_recur(first_row, None, &quadrant, walls));
    }
    visible
}

fn out_of_bounds(x: i32, y: i32) -> bool {
    i32::min(x, y) < 0 || x > BOARD_SIZE_X || y > BOARD_SIZE_Y
}

fn scan_row_recur(
    mut row: Row,
    prev_tile: Option<DepthColVec>,
    quadrant: &Quadrant,
    walls: &HashSet<[i32; 2]>,
) -> Vec<[i32; 2]> {
    let mut prev_tile = prev_tile;
    let mut visible = Vec::new();
    let is_wall = |x, y| walls.contains(&[x, y]) || out_of_bounds(x, y);
    let is_floor = |x, y| !walls.contains(&[x, y]);

    for tile in row.tiles().iter() {
        let (x, y) = quadrant.transform(tile);

        if is_wall(x, y) || is_symmetric(&row, tile) {
            visible.push([x, y]);
        }

        if prev_tile.is_some() {
            let (px, py) = quadrant.transform(&prev_tile.unwrap());

            if is_wall(px, py) && is_floor(x, y) {
                row.start_slope = slope(tile);
            }

            if is_floor(px, py) & is_wall(x, y) {
                if let Some(mut next_row) = row.next() {
                    next_row.end_slope = slope(tile);
                    visible.append(&mut scan_row_recur(next_row, prev_tile, quadrant, walls));
                }
            }
        }
        prev_tile = Some(*tile);
    }

    if prev_tile.is_some() {
        let (px, py) = quadrant.transform(&prev_tile.unwrap());

        if is_floor(px, py) {
            let next_row = row.next();
            if next_row.is_some() {
                visible.append(&mut scan_row_recur(
                    next_row.unwrap(),
                    prev_tile,
                    quadrant,
                    walls,
                ));
            }
        }
    }
    visible
}

// is_symmetric: checks if a given floor tile can be seen symmetrically from the origin. It returns
// true if the central point of the tile is in the sector swept out by the row’s start and end slopes.
// Otherwise, it returns false.
fn is_symmetric(row: &Row, tile: &DepthColVec) -> bool {
    let col = tile.to_array()[1] as f32;
    let depth = row.depth as f32;
    col >= depth * row.start_slope && col <= depth * row.end_slope
}

fn slope(tile: &DepthColVec) -> f32 {
    let [row_depth, col] = tile.to_array();
    (2.0 * col as f32 - 1.0) / (2.0 * row_depth as f32)
}

// round_ties_up and round_ties_down round n to the nearest integer. If n ends in .5, round_ties_up
// rounds up and round_ties_down rounds down. Note: round_ties_up is not the same as Python’s round.
// Python’s round will round away from 0, resulting in unwanted behavior for negative numbers.
// TODO: verify this is necessary with Rust
fn round_ties_up(n: f32) -> i32 {
    f32::floor(n + 0.5) as i32
}

fn round_ties_down(n: f32) -> i32 {
    f32::ceil(n - 0.5) as i32
}

struct Quadrant {
    pub cardinal: Cardinal,
    pub ox: i32,
    pub oy: i32,
}

impl Quadrant {
    // map rows & columns to global board position (x,y)
    pub fn transform(&self, tile: &DepthColVec) -> (i32, i32) {
        let [row, col] = tile.to_array();
        match self.cardinal {
            North => return (self.ox + col, self.oy - row),
            South => return (self.ox + col, self.oy + row),
            East => return (self.ox + row, self.oy + col),
            West => return (self.ox - row, self.oy + col),
        }
    }

    pub fn new(cardinal: Cardinal, origin: &XyVec) -> Self {
        let [ox, oy] = origin.to_array();
        Quadrant { cardinal, ox, oy }
    }
}

#[derive(Debug, Clone)]
struct Row {
    depth: i32,
    start_slope: f32,
    end_slope: f32,
}

impl Row {
    fn new(depth: i32, start_slope: f32, end_slope: f32) -> Self {
        Row {
            depth,
            start_slope,
            end_slope,
        }
    }

    fn tiles(&self) -> Vec<DepthColVec> {
        let mut ts = vec![];
        let min_col = round_ties_up(self.depth as f32 * self.start_slope);
        let max_col = round_ties_down(self.depth as f32 * self.end_slope);

        for col in min_col..=max_col {
            ts.push(IVec2::new(self.depth, col));
        }
        ts
    }

    fn next(&self) -> Option<Self> {
        // FIXME check board bounds
        if self.depth < 15 {
            Some(Row {
                depth: self.depth + 1,
                start_slope: self.start_slope,
                end_slope: self.end_slope,
            })
        } else {
            None
        }
    }
}

enum Cardinal {
    North,
    East,
    South,
    West,
}
use Cardinal::*;

const CARDINALS: [Cardinal; 4] = [North, East, South, West];
