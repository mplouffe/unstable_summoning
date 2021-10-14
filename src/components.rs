pub use crate::prelude::*;

use strum_macros::{EnumIter, AsStaticStr};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub render: bool,
    pub tint: RGBA,
    pub z_order: i32,
    pub index: usize,
    pub scale: (i32, i32),
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
pub struct Cursor {
    pub is_active: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PopupType {
    ActionsInput,
    TextOutput,
}

#[derive(Clone, Copy, PartialEq)]
pub struct PopupRequest {
    pub popup_type: PopupType,
    pub open: bool,
}

#[derive(Clone, PartialEq)]
pub struct Popup {
    pub options: Vec<Actions>,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct MouseInput {
    pub mouse_point_bg: Point,
    pub mouse_point_hud: Point,
    pub left_click: ClickState,  
}

#[derive(Clone, Copy, PartialEq)]
pub struct Computer {
    pub computer_state: ComputerState
}

#[derive(Clone, Copy, PartialEq)]
pub enum ComputerState {
    Unloaded,
    Loaded,
    Compiling,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Disk {
    pub color: DiskColor,
    pub disk_state: DiskState,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DiskState {
    Untouched,
    Grabbed,
    Loaded,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ClickState {
    Unclicked,
    Clicked,
    Held,
    Released
}

#[derive(Clone, Copy, PartialEq, Debug, EnumIter, AsStaticStr)]
pub enum Actions {
    Look,
    RubberDuck,
    Load,   // all actions post here require DiskState == Grabbed
    Compile,
    StackDump,
    Run,
}

#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
pub enum DiskColor {
    Blue,
    Yellow,
    Green,
    Red,
    Orange,
    Purple,
    Aqua,
    NeonGreen,
    Black,
    Silver,
    Copper,
    NeonPink,
    Crimson,
    BlueGreen,
    Magenta,
    Gold
}

#[derive(Clone, Copy)]
pub struct TimerInfo {
    pub frame: usize,
    pub timer: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Animation {
    pub state: AnimationState,
    pub starting_frame: usize,
    pub loop_play: bool,
    pub animation_index: usize,
    pub final_frame: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AnimationState {
    Start,
    Playing,
    Stopping,
    Stopped,
}