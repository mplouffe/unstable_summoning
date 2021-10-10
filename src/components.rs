pub use crate::prelude::*;

use strum_macros::EnumIter;

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

#[derive(Clone, Copy, PartialEq)]
pub struct Cursor;

#[derive(Clone, Copy, PartialEq)]
pub struct Liquid {
    pub color: LiquidColor,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Sludge {
    pub color: SludgeColor,
}

#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
pub enum LiquidColor {
    Orange,
    Pink,
    Red,
    Grey,
    White,
    Yellow,
    Purple,
    Indigo,
    BubbleGreen,
    BubbleWhite,
    BubblePink,
    BubbleYellow,
}

#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
pub enum SludgeColor {
    Gold,
    Bronze,
    Crimson,
    RedPink,
    Black,
    Blue,
    Brown,
    BlueGreen,
    Aqua,
    DarkBlue,
    Green,
}
