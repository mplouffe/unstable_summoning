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
        .for_each(|(_entity, pos, popup)| {
            draw_batch.draw_box(Rect::with_size(pos.x, pos.y, popup.width, popup.height), ColorPair::new(GREEN, BLACK));
            
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
            
            match popup.popup_type {
                PopupType::TextOutput => {
                    if let Some(popup_text) = &popup.text {
                        draw_batch.print_color(Point::new(pos.x +1, pos.y+1), popup_text.clone(), ColorPair::new(GREEN, BLACK));
                    }
                },
                _ => {}
            }
        });
    draw_batch.submit(10100).expect("Batch error");
}