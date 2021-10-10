use crate::prelude::*;

use strum::IntoEnumIterator;
use rand::thread_rng;
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
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 6
            },
            Health { current: 10, max: 10 },
        )
    );
}

pub fn spawn_cursor(ecs: &mut World) {
    ecs.push(
        (
            Cursor { },
            Name("Cursor".to_string()),
            Point::new(DISPLAY_WIDTH/2, DISPLAY_HEIGHT/2),
            Render {
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 0
            },
        )
    );
}

pub fn spawn_flasks(ecs: &mut World, rng: &mut RandomNumberGenerator, flask_positions: &[Point]) {    
    
    let mut rng = thread_rng();

    let random_colors = LiquidColor::iter().choose_multiple(&mut rng, flask_positions.len());
    flask_positions
        .iter()
        .zip(random_colors.into_iter())
        .for_each(|(pos, liquid_color)| {
            let liquid = Liquid { color: liquid_color };
            ecs.push(
                (
                    liquid,
                    Name("Flask".to_string()),
                    pos.clone(),
                    Render {
                        z_order: 100,
                        tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        index: liquid_color as usize + 8
                    },
                )
            );
        });
}
