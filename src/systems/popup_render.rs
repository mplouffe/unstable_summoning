use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Popup)]
pub fn popup_render(
    ecs: &SubWorld,
) {
    let mut positions = <(Entity, &Point, &Popup)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    
    positions
        .iter(ecs)
        .for_each(|(_entity, pos, _cursor)| {
            let offset_x = (pos.x *4) + 4;
            let position_y = pos.y * 4;

            draw_batch.draw_box(Rect::with_size(offset_x, position_y, 64, 64), ColorPair::new(WHITE, BLACK));        
        });
    draw_batch.submit(10100).expect("Batch error");
}