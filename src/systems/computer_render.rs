use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Computer)]
pub fn computer_render(
    ecs: &SubWorld
) {
    let mut renderables = <(&Point, &Render, &Computer)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(SPRITE_LAYER);

    renderables
        .iter(ecs)
        .filter(|(_, render, _)| render.render)
        .for_each(|(pos, render, computer)| {
            draw_batch.set_sprite(
                Rect::with_size(pos.x*TILE_SIZE, pos.y*TILE_SIZE, render.scale.0*TILE_SIZE, render.scale.1*TILE_SIZE),
                render.z_order,
                render.tint,
                render.index
            );

            match computer.computer_state {
                ComputerState::Loaded => {
                    draw_batch.set_sprite(
                        Rect::with_size(pos.x*TILE_SIZE, pos.y*TILE_SIZE, render.scale.0*TILE_SIZE, render.scale.1*TILE_SIZE),
                        render.z_order,
                        render.tint,
                        render.index + 2,
                    );
                },
                ComputerState::Running => {
                    draw_batch.set_sprite(
                        Rect::with_size(pos.x*TILE_SIZE, pos.y*TILE_SIZE, render.scale.0*TILE_SIZE, render.scale.1*TILE_SIZE),
                        render.z_order + 10,
                        render.tint,
                        render.index + 2,
                    );
                    draw_batch.set_sprite(
                        Rect::with_size(pos.x*TILE_SIZE, pos.y*TILE_SIZE, render.scale.0*TILE_SIZE, render.scale.1*TILE_SIZE),
                        render.z_order + 20,
                        render.tint,
                        render.index + 1,
                    );
                }
                _ => { }
            }
        }
    );

    draw_batch.submit(5000).expect("Batch error");
}