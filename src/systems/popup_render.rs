use crate::prelude::*;

use strum::AsStaticRef;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Popup)]
pub fn popup_render(
    ecs: &SubWorld,
    #[resource] mouse_input: &MouseInput
) {
    let mut positions = <(Entity, &Point, &Popup)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_LAYER);
    positions
        .iter(ecs)
        .for_each(|(_entity, pos, popup)| {
            
            draw_batch.draw_box(Rect::with_size(pos.x, pos.y, popup.width, popup.height), ColorPair::new(GREEN, BLACK));
            
            let num_options: i32 = popup.options.len() as i32;
            let in_x_range = mouse_input.mouse_point_hud.x >= pos.x+1 && mouse_input.mouse_point_hud.x <= pos.x+11;
            let in_y_range = mouse_input.mouse_point_hud.y >= pos.y+1 && mouse_input.mouse_point_hud.y <= pos.y + num_options;
            let mut hovered_index = -1;

            if in_x_range && in_y_range {
                hovered_index = mouse_input.mouse_point_hud.y - (pos.y+1);
            }

            for i in 0..num_options {
                let color = if i == hovered_index { ColorPair::new(RED, BLACK) } else { ColorPair::new(GREEN, BLACK) };
                draw_batch.print_color(Point::new(pos.x+1, pos.y+(i+1)), popup.options[i as usize].as_static(),color);
            }
        });
    draw_batch.submit(10100).expect("Batch error");
}