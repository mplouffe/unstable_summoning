use crate::prelude::*;

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
        .for_each(|(_entity, pos, _cursor)| {
            
            draw_batch.draw_box(Rect::with_size(pos.x, pos.y, 12, 8), ColorPair::new(WHITE, BLACK));
            
            let in_x_range = mouse_input.mouse_point_hud.x >= pos.x+1 && mouse_input.mouse_point_hud.x <= pos.x+11;
            let in_y_range = mouse_input.mouse_point_hud.y >= pos.y+1 && mouse_input.mouse_point_hud.y <= pos.y+4;
            let mut hovered_index = -1;
            let highlighted = ColorPair::new(RED, BLACK);
            if in_x_range && in_y_range {
                hovered_index = mouse_input.mouse_point_hud.y - (pos.y+1);
            }
            
            if hovered_index == 0 {
                draw_batch.print_color(Point::new(pos.x+1, pos.y+1), "LOOK", highlighted);
            } else {
                draw_batch.print(Point::new(pos.x+1, pos.y+1), "LOOK");
            }
 
            if hovered_index == 1 {
                draw_batch.print_color(Point::new(pos.x+1, pos.y+2), "LOAD", highlighted);
            } else {
                draw_batch.print(Point::new(pos.x+1, pos.y+2), "LOAD");
            }

            if hovered_index == 2 {
                draw_batch.print_color(Point::new(pos.x+1, pos.y+3), "COMPILE", highlighted);
            } else {
                draw_batch.print(Point::new(pos.x+1, pos.y+3), "COMPILE");
            }

            if hovered_index == 3 {
                draw_batch.print_color(Point::new(pos.x+1, pos.y+4), "STACK DUMP", highlighted);
            } else {
                draw_batch.print(Point::new(pos.x+1, pos.y+4), "STACK DUMP");
            }

        });
    draw_batch.submit(10100).expect("Batch error");
}