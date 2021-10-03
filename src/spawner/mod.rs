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
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Health { current: 10, max: 10 },
        )
    );
}
