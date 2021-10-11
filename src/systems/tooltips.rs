use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Cursor)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
) {
    let mut positions = <(Entity, &Point, &Name)>::query()
        .filter(!component::<Cursor>());

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == *mouse_pos)
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 4;
            let display = if let Ok(desc) = ecs.entry_ref(*entity)
                .unwrap()
                .get_component::<Description>()
            {
                format!("{} : {}", &name.0, &desc.0)
            } else {
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}