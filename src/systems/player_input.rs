use crate::{prelude::*};

#[system]
#[write_component(Point)]
#[read_component(Name)]
#[write_component(Cursor)]
#[read_component(Player)]
#[read_component(Disk)]
#[read_component(Computer)]
#[write_component(Popup)]
#[read_component(DimensionalButton)]
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

    // click on things
    match mouse_input.left_click {
        ClickState::Released => {
            let mut positions = <(Entity, &Point, &Name)>::query()
                .filter(!component::<Cursor>());
        
            let mut cursor_target_updated = false;
            positions
                .iter(ecs)
                .filter(|(_, pos, _)| **pos == mouse_input.mouse_point_bg)
                .for_each(|(entity, _pos, _name)| {
                    let entity_ref = ecs.entry_ref(*entity).unwrap();
                    if let Ok(_disk) = entity_ref.get_component::<Disk>()
                    {
                        cursor.is_active = true;
                        cursor_target_updated = true;
                        commands.push(((), PopupRequest {
                                popup_type: PopupType::UnloadedDisk,
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
                    else if let Ok(_computer) = entity_ref.get_component::<Computer>()
                    {
                        cursor.is_active = true;
                        cursor_target_updated = true;
                        commands.push(((), PopupRequest {
                            popup_type: PopupType::Computer,
                            target: Some(*entity),
                            text: None,
                            },
                            Point::new(2, 5)
                        ));
                    }
                    else if let Ok(_dimensional_button) = entity_ref.get_component::<DimensionalButton>()
                    {
                        let mut computers = <&Computer>::query();

                        let loaded_computers = computers.iter(ecs).filter(|&computer| computer.computer_state == ComputerState::Running).count();
                        
                        if loaded_computers == 2 {                                
                            commands.add_component(*entity, Render {
                                render: true,
                                z_order: 100,
                                tint: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                                index: 55,
                                scale: (1, 1)
                            });

                            commands.push(((),
                                ActionRequest {
                                    action: Actions::OpenTransDimensionalWarp,
                                    target: None,
                                }));

                        }
                        else
                        {
                            cursor.is_active = true;
                            cursor_target_updated = true;
                            commands.push(((), PopupRequest {
                                popup_type: PopupType::TextOutput,
                                target: Some(*entity),
                                text: Some(vec![String::from("Run a disk"), String::from("in both comps.")]),
                                },
                                Point::new(2, 5)
                            ));
                        }
                    }
                });
            
            if cursor_target_updated {
                commands.add_component(cursor_entity, cursor);
                commands.add_component(cursor_entity, mouse_input.mouse_point_bg);
            } else {
                if cursor.is_active {
                    cursor.is_active = false;
                    commands.add_component(cursor_entity, cursor);
                    commands.push(((), ClosePopupRequest {
                            target: None,
                        }
                    ));
                }
            }
        },
        _ => {}
    };
}
