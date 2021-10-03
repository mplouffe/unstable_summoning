use crate::prelude::*;

pub fn spawn_player(ecs: &mut World) {
    ecs.push(
        (
            Player {
                level: 0
            },
            Point::new(40, 25),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Health { current: 10, max: 10 },
        )
    );
}
