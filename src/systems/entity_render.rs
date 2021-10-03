use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Player)]
pub fn entity_render(
    ecs: &SubWorld
) {
    let mut renderables = <(&Point, &Render)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

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