use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Popup)]
#[read_component(Button)]
pub fn button_render(
    ecs: &SubWorld,
    #[resource] mouse_input: &MouseInput
) {
    let mut positions = <(Entity, &Point, &Popup)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(POPUP_LAYER);
    positions
        .iter(ecs)
        .for_each(|(_entity, pos, popup)| {            
            popup.options.iter()
                .for_each(|option| {
                    let color;
                    if option.button_area.point_in_rect(mouse_input.mouse_point_hud) {
                        color = ColorPair::new(RED, BLACK);
                    } else {
                        color = ColorPair::new(GREEN, BLACK);
                    }
                    draw_batch.print_color(Point::new(option.button_area.x1, option.button_area.y1), option.text.clone(), color);
                });
        });

    draw_batch.submit(10200).expect("Batch error");
}