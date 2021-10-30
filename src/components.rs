pub use crate::prelude::*;

use strum_macros::EnumIter;

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
    pub popup_open: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PopupType {
    UnloadedDisk,
    TextOutput,
    Computer,
    DimensionalButton,
}

#[derive(Clone, PartialEq)]
pub struct PopupRequest {
    pub popup_type: PopupType,
    pub target: Option<Entity>,
    pub text: Option<Vec<String>>,
}

#[derive(Clone, PartialEq)]
pub struct ClosePopupRequest {
    pub target: Option<Entity>
}

#[derive(Clone, PartialEq)]
pub struct Popup {
    pub popup_type: PopupType,
    pub options: Vec<Button>,
    pub bounding_box: Rect,
    pub width: i32,
    pub height: i32,
    pub target: Option<Entity>,
    pub text: Option<Vec<String>>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct MouseInput {
    pub mouse_point_bg: Point,
    pub mouse_point_hud: Point,
    pub left_click: ClickState,  
}

#[derive(Clone, Copy, PartialEq)]
pub enum ClickState {
    Unclicked,
    Clicked,
    Held,
    Released
}

#[derive(Clone, Copy, PartialEq)]
pub struct Computer {
    pub computer_state: ComputerState,
    pub loaded_disk: Option<Entity>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ComputerState {
    Unloaded,
    Loaded,
    Compiling,
    Running,
}

#[derive(Clone, PartialEq)]
pub struct Disk {
    pub color: DiskColor,
    pub disk_state: DiskState,
    pub disk_label: String,
    pub original_pos: Point,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DiskState {
    Untouched,
    Loaded,
}

#[derive(Clone, Copy, PartialEq)]
pub struct DimensionalButton { }


#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
pub enum Actions {
    // actions for Disk
    Look,
    Smell,
    Load,
    Unload,
    
    // actions for Computer
    Compile,
    StackDump,
    Run,
    StopRun,

    CloseWindow,

    OpenTransDimensionalWarp,
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

#[derive(Clone, PartialEq)]
pub struct Button {
    pub action: Actions,
    pub button_area: Rect,
    pub text: String,
}

#[derive(Clone)]
pub struct ActionRequest {
    pub action: Actions,
    pub target: Option<Entity>,
}