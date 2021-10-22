use crate::prelude::*;

mod animation;
mod action;
mod computer_render;
mod cursor_render;
mod map_render;
mod end_turn;
mod entity_render;
mod hud;
mod player_input;
mod popup;
mod popup_render;
mod tooltips;


pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(animation::animation_system())
        .add_system(player_input::player_input_system())
        .add_system(action::action_system())
        .add_system(map_render::map_render_system())
        .add_system(animation::animation_system())
        .add_system(popup::popup_system())
        .add_system(popup_render::popup_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(cursor_render::cursor_render_system())
        .add_system(computer_render::computer_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltips_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(animation::animation_system())
        .add_system(map_render::map_render_system())
        .add_system(popup::popup_system())
        .add_system(popup_render::popup_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(cursor_render::cursor_render_system())
        .add_system(computer_render::computer_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(animation::animation_system())
        .add_system(map_render::map_render_system())
        .add_system(popup::popup_system())
        .add_system(popup_render::popup_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(cursor_render::cursor_render_system())
        .add_system(computer_render::computer_render_system())
        .add_system(hud::hud_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_game_start_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(animation::animation_system())
        .add_system(entity_render::entity_render_system())
        .build()
}