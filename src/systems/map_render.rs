use crate::prelude::*;

#[system]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(BG_LAYER);
    for y in 0..DISPLAY_HEIGHT {
        for x in 0..DISPLAY_WIDTH {
            let pt = Point::new(x, y);
            let idx = map_idx(x, y);
            if map.in_bounds(pt) {
                draw_batch.set(
                    pt,
                    ColorPair::new(
                        WHITE,
                        BLACK
                    ),
                    to_cp437('.')
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}