use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Popup)]
pub fn popup_render(
    ecs: &SubWorld
) {
    let mut positions = <(Entity, &Point, &Popup)>::query();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(POPUP_LAYER);
    positions
        .iter(ecs)
        .for_each(|(_entity, pos, popup)| {
            draw_batch.draw_box(popup.bounding_box, ColorPair::new(GREEN, BLACK));

            match popup.popup_type {
                PopupType::TextOutput => {
                    if let Some(popup_text_lines) = &popup.text {
                        let mut y_offset = 1;
                        popup_text_lines.iter()
                            .for_each(|popup_text| {
                                draw_batch.print_color(Point::new(pos.x +1, pos.y+y_offset), popup_text.clone(), ColorPair::new(GREEN, BLACK));
                                y_offset += 1;
                            }
                        );                   
                    }
                },
                _ => {}
            }
        });

    draw_batch.submit(10100).expect("Batch error");
}