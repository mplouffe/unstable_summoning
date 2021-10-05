use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Player)]
pub fn player_render(
    ecs: &SubWorld
) {
    let mut renderables = <(&Point, &Render)>::query().filter(component::<Player>());

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(SPRITE_LAYER);

    renderables
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set_sprite(
                Rect::with_size(pos.x*TILE_SIZE, pos.y*TILE_SIZE, 64, 64),
                400 - 100,
                RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                0
            );
        }
    );

    draw_batch.submit(5000).expect("Batch error");
}