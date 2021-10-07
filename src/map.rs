use crate::prelude::*;

const NUM_TILES: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>
}

// pub fn map_idx(x: i32, y: i32) -> usize {
//     ((y * DISPLAY_WIDTH) + x) as usize
// }

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < DISPLAY_WIDTH
            && point.y >= 0 && point.y < DISPLAY_HEIGHT
    }

    // pub fn try_idx(&self, point : Point) -> Option<usize> {
    //     if !self.in_bounds(point) {
    //         None
    //     } else {
    //         Some(map_idx(point.x, point.y))
    //     }
    // }
}
