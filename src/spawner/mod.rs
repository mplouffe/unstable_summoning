use crate::prelude::*;

use strum::IntoEnumIterator;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

pub fn spawn_player(ecs: &mut World) {
    ecs.push(
        (
            Player {
                level: 0
            },
            Name("Player".to_string()),
            Point::new(DISPLAY_WIDTH - 2, 1),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 6,
                scale: (1, 1),
            },
            Health { current: 10, max: 10 },
        )
    );
}

pub fn spawn_cursor(ecs: &mut World) {
    ecs.push(
        (
            Cursor { 
                is_active: false,
                popup_open: false,
            },
            Name("Cursor".to_string()),
            Point::new(DISPLAY_WIDTH/2, DISPLAY_HEIGHT/2),
            Render {
                render: false,
                z_order: 1000,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 0,
                scale: (1, 1),
            },
        )
    );
}

pub fn spawn_disks(ecs: &mut World, rng: &mut ThreadRng, disk_positions: &[Point]) {    
    let random_colors = DiskColor::iter().choose_multiple(rng, disk_positions.len());
    disk_positions
        .iter()
        .zip(random_colors.into_iter())
        .for_each(|(pos, disk_color)| {
            let disk = Disk { 
                color: disk_color,
                disk_state: DiskState::Untouched,
                disk_label: "This is the label description".to_string(),
                original_pos: pos.clone(),
            };
            ecs.push(
                (
                    disk,
                    Name("Disk".to_string()),
                    pos.clone(),
                    Render {
                        render: true,
                        z_order: 100,
                        tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        index: disk_color as usize + 8,
                        scale: (1, 1),
                    },
                )
            );
        });
}

pub fn spawn_computers(ecs: &mut World, computer_positions: &[Point]) {   
    computer_positions
        .iter()
        .for_each(|pos| {
            let computer = Computer {
                computer_state: ComputerState::Unloaded,
                loaded_disk: None,
            };
            ecs.push(
                (
                    computer,
                    Name("Computer".to_string()),
                    pos.clone(),
                    Render {
                        render: true,
                        z_order: 100,
                        tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        index: 24,
                        scale: (1, 1),
                    }
                )
            );
        });
}

pub fn spawn_infrastructure(ecs: &mut World) {
    // the platform
    ecs.push(
        (
            Point::new(9, 4),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 29,
                scale: (1, 1),
            }
        )
    );
    ecs.push(
        (
            Point::new(8, 4),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 28,
                scale: (1, 1),
            }
        )
    );
    ecs.push(
        (
            Point::new(10, 4),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 30,
                scale: (1, 1),
            }
        )
    );

    // pipes
    ecs.push(
        (
            Point::new(10, 5),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 53,
                scale: (1, 1),
            }
        )
    );
    ecs.push(
        (
            Point::new(9, 5),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 52,
                scale: (1, 1),
            }
        )
    );
    ecs.push(
        (
            Point::new(8, 5),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 53,
                scale: (1, 1),
            }
        )
    );
}

pub fn spawn_title_screen(ecs: &mut World) {
    // spawn computers
    ecs.push(
        (
            Point::new(7, 2),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 24,
                scale: (2, 2),
            }
        )
    );
    ecs.push(
        (
            Point::new(11, 2),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 24,
                scale: (2, 2),
            }
        )
    );

    ecs.push(
        (
            Point::new(9, 2),
            Render {
                render: true,
                z_order: 1000,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 33,
                scale: (2, 2),
            }
        )
    );

    // spawn platform
    ecs.push(
        (
            Name("Platform".to_string()),
            Point::new(9, 3),
            Render {
                render: true,
                z_order: 500,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 2,
                scale: (2, 2),
            }
        )
    );

    // spawn fire
    ecs.push(
        (
            Point::new(9, 2),
            Render {
                render: true,
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 0,
                scale: (2, 2),
            },
            Animation {
                state: AnimationState::Start,
                starting_frame: 0,
                loop_play: true,
                animation_index: 0,
                final_frame: 2,
            },
        )
    );
}
