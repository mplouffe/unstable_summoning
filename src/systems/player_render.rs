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
    draw_batch.target(CURSOR_LAYER);

    renderables
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos,
                render.color,
                render.glyph
            );
        }
    );

    draw_batch.submit(5000).expect("Batch error");
}