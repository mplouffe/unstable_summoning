use crate::{prelude::*};

#[system]
#[write_component(Cursor)]
#[write_component(Popup)]
pub fn popup_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] mouse_input: &mut MouseInput,
) 
{
    let mut cursors = <(Entity, &Cursor)>::query();

    let (cursor_entity, mut cursor) = cursors
        .iter(ecs)
        .find_map(|(entity, cursor)| Some((*entity, *cursor)))
        .unwrap();

    let mut left_click_state = mouse_input.left_click;
    match mouse_input.left_click {
        ClickState::Released => {
            let mut popups = <(Entity, &Popup)>::query();
            popups
                .iter(ecs)
                .filter(|(_, popup)| popup.bounding_box.point_in_rect(mouse_input.mouse_point_hud))
                .for_each(|(entity, popup)| {
                    left_click_state = ClickState::Unclicked;
                    popup.options.iter()
                        .filter(|option| option.button_area.point_in_rect(mouse_input.mouse_point_hud))
                        .for_each(|option| {
                            match option.action {
                                Actions::CloseWindow => {
                                    commands.push(((),
                                        ActionRequest {
                                            target: Some(*entity),
                                            action: option.action,
                                    }));
                                    cursor.is_active = false;
                                    commands.add_component(cursor_entity, cursor);
                                },
                                Actions::Load => {
                                    commands.push(((),
                                        ActionRequest {
                                            target: Some(*entity),
                                            action: Actions::CloseWindow,
                                        }
                                    ));
                                    commands.push(((),
                                        ActionRequest {
                                            target: popup.target,
                                            action: option.action
                                        }
                                    ));
                                    cursor.is_active = false;
                                    commands.add_component(cursor_entity, cursor);
                                },
                                _ => {
                                    commands.push(((),
                                        ActionRequest {
                                            target: Some(*entity),
                                            action: Actions::CloseWindow,
                                        }
                                    ));
                                    commands.push(((),
                                        ActionRequest {
                                            target: popup.target,
                                            action: option.action,
                                    }));
                                }
                            }
                        }
                    );
                }
            );
        },
        _ => {}
    };
    mouse_input.left_click = left_click_state;
}
