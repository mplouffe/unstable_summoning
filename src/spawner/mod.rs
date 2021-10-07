use crate::prelude::*;

pub fn spawn_player(ecs: &mut World) {
    ecs.push(
        (
            Player {
                level: 0
            },
            Name("Player".to_string()),
            Point::new(DISPLAY_WIDTH/2, DISPLAY_HEIGHT/2),
            Render {
                z_order: 100,
                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                index: 0
            },
            Health { current: 10, max: 10 },
        )
    );
}

pub fn spawn_flasks(ecs: &mut World, flask_positions: &[Point]) {    
    flask_positions
        .iter()
        .for_each(|pos| {
            ecs.push(
                (
                    Flask {
                        color: ColorPair::new(WHITE, BLACK)
                    },
                    Name("Flask".to_string()),
                    pos.clone(),
                    Render {
                        z_order: 100,
                        tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        index: 2
                    },
                )
            );
        });
}
