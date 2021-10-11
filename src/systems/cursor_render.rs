use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Cursor)]
pub fn cursor_render(
    ecs: &SubWorld
) {
    let mut renderables = <(&Point, &Render, &Cursor)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(SPRITE_LAYER);

    renderables
        .iter(ecs)
        .for_each(|(pos, render, cursor)| {

            if cursor.is_active {
                draw_batch.set_sprite(
                    Rect::with_size(pos.x*TILE_SIZE, pos.y*TILE_SIZE, 64, 64),
                    render.z_order,
                    render.tint,
                    render.index + 1,
                );
            } else {
                draw_batch.set_sprite(
                    Rect::with_size(pos.x*TILE_SIZE, pos.y*TILE_SIZE, 64, 64),
                    render.z_order,
                    render.tint,
                    render.index,
                );
            }
        }
    );

    draw_batch.submit(5000).expect("Batch error");
}