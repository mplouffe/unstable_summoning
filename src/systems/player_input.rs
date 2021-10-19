use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Name)]
#[write_component(Cursor)]
#[read_component(Player)]
#[read_component(Disk)]
#[write_component(Popup)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] mouse_input: &MouseInput,
) 
{
    // update cursor position
    let mut cursors = <(Entity, &Cursor)>::query();

    let (cursor_entity, mut cursor) = cursors
        .iter(ecs)
        .find_map(|(entity, cursor)| Some((*entity, *cursor)))
        .unwrap();
    
    if !cursor.is_active {
        commands.add_component(cursor_entity, mouse_input.mouse_point_bg);
    }

    // click on things
    match mouse_input.left_click {
        ClickState::Released => {
            let mut click_consumed = false;
            if cursor.popup_open {
                let mut popups = <&Popup>::query();

                popups
                    .iter(ecs)
                    .for_each(|popup| {
                        popup.options.iter()
                            .for_each(|option| {
                                if option.button_area.point_in_rect(mouse_input.mouse_point_hud) {
                                    click_consumed = true;
                                    cursor.is_active = false;
                                    cursor.popup_open = false;

                                    commands.add_component(cursor_entity, cursor);
                                    commands.push(((),
                                        PopupRequest {
                                            popup_type: popup.popup_type,
                                            open: false,
                                            target: None,
                                            text: None,
                                        },
                                        Point::zero()
                                    ));
                                    commands.push(((),
                                        ActionRequest {
                                            target: popup.target,
                                            action: option.action,
                                    }));
                                }
                            });
                    });
            }

            if !click_consumed {
                let mut positions = <(Entity, &Point, &Name)>::query()
                    .filter(!component::<Cursor>());
            
                let mut cursor_target_updated = false;
                positions
                    .iter(ecs)
                    .filter(|(entity, pos, _)| **pos == mouse_input.mouse_point_bg)
                    .for_each(|(entity, _pos, _name)| {
                        let entity_ref = ecs.entry_ref(*entity).unwrap();

                        if let Ok(_disk) = entity_ref.get_component::<Disk>()
                        {
                            cursor.is_active = true;
                            cursor.popup_open = true;
                            cursor_target_updated = true;
                            commands.push(((), PopupRequest {
                                    popup_type: PopupType::UnloadedDisk,
                                    open: true,
                                    target: Some(*entity),
                                    text: None,
                                },
                                Point::new(2, 5)
                            ));
                        }
                        else if let Ok(_player) = entity_ref.get_component::<Player>()
                        {
                            cursor.is_active = true;
                            cursor_target_updated = true;
                        }
                    });
                
                if cursor_target_updated {
                    commands.add_component(cursor_entity, cursor);
                    commands.add_component(cursor_entity, mouse_input.mouse_point_bg);
                } else {
                    if cursor.is_active {
                        cursor.is_active = false;
                        cursor.popup_open = false;
                        commands.add_component(cursor_entity, cursor);
                        commands.add_component(cursor_entity, mouse_input.mouse_point_bg);
                        commands.push(((), PopupRequest {
                                popup_type: PopupType::UnloadedDisk,
                                open: false,
                                target: None,
                                text: None,
                            },
                            Point::zero()
                        ));
                    }
                }
            }
        },
        _ => {}
    };
}
