pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub tint: RGBA,
    pub z_order: i32,
    pub index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub level: u32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, PartialEq)]
pub struct Description(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Flask {
    pub color: ColorPair,
}
