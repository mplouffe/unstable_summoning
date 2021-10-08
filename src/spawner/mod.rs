use crate::prelude::*;

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
    flask_positions
        .iter()
        .for_each(|pos| {
            let substance_color = match rng.range(0, 23) {
                0 => SubstanceColor::Aqua,
                1 => SubstanceColor::Black,
                2 => SubstanceColor::Blue,
                3 => SubstanceColor::BlueGreen,
                4 => SubstanceColor::Bronze,
                5 => SubstanceColor::Brown,
                6 => SubstanceColor::BubbleGreen,
                7 => SubstanceColor::BubblePink,
                8 => SubstanceColor::BubbleWhite,
                9 => SubstanceColor::BubbleYellow,
                10 => SubstanceColor::Crimson,
                11 => SubstanceColor::DarkBlue,
                12 => SubstanceColor::Gold,
                13 => SubstanceColor::Green,
                14 => SubstanceColor::Grey,
                15 => SubstanceColor::Indigo,
                16 => SubstanceColor::Orange,
                17 => SubstanceColor::Pink,
                18 => SubstanceColor::Purple,
                19 => SubstanceColor::Red,
                20 => SubstanceColor::RedPink,
                21 => SubstanceColor::White,
                _ => SubstanceColor::Yellow
            };
            let substance = Substance { color: substance_color };
            ecs.push(
                (
                    Flask {
                        color: ColorPair::new(WHITE, BLACK)
                    },
                    substance,
                    Name("Flask".to_string()),
                    pos.clone(),
                    Render {
                        z_order: 100,
                        tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                        index: substance_color as usize + 8
                    },
                )
            );
        });
}
