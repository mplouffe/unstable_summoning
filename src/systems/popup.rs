
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(PopupRequest)]
#[write_component(ClosePopupRequest)]
#[read_component(Popup)]
#[read_component(Computer)]
pub fn popup(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    let mut create_requests = <(Entity, &Point, &PopupRequest)>::query();
    
    create_requests
        .iter(ecs)
        .for_each(|(entity, pos, request)| {
            match request.popup_type {
                PopupType::UnloadedDisk => {
                    let mut buttons = Vec::new();
                    buttons.push(Button {
                        action: Actions::Look,
                        button_area: Rect::with_size(pos.x+1, pos.y+1, 12, 1),
                        text: "LOOK".to_string(),
                    });
                    buttons.push(Button {
                        action: Actions::Smell,
                        button_area: Rect::with_size(pos.x+1, pos.y+2, 12, 1),
                        text: "SMELL".to_string(),
                    });
                    buttons.push(Button {
                        action: Actions::Load,
                        button_area: Rect::with_size(pos.x+1, pos.y+3, 12, 1),
                        text: "LOAD".to_string(),
                    });
                    commands.push(((),
                        pos.clone(),
                        Popup {
                            popup_type: request.popup_type,
                            options: buttons,
                            bounding_box: Rect::with_size(pos.x, pos.y, 12, 8),
                            target: request.target,
                            text: None,
                        }
                    ));
                },
                PopupType::TextOutput => {
                    let mut buttons = Vec::new();
                    buttons.push(Button {
                        action: Actions::CloseWindow,
                        button_area: Rect::with_size(pos.x+25, pos.y+9, 10, 1),
                        text: "CLOSE".to_string(),
                    });

                    if let Some(popup_text) = &request.text {
                        commands.push(((),
                            pos.clone(),
                            Popup {
                                popup_type: request.popup_type,
                                options: buttons,
                                bounding_box: Rect::with_size(pos.x, pos.y, 30, 10),
                                target: None,
                                text: Some(popup_text.clone()),
                            }));
                    }
                },
                PopupType::Computer => {
                    if let Some(mut target_computer) = &request.target {
                        let computer_ref = ecs.entry_ref(target_computer).unwrap();
                        if let Ok(computer) = computer_ref.get_component::<Computer>()
                        {
                            match computer.computer_state {
                                ComputerState::Unloaded => {
                                    let mut buttons = Vec::new();
                                    buttons.push(Button {
                                        action: Actions::CloseWindow,
                                        button_area: Rect::with_size(pos.x+25, pos.y+9, 10, 1),
                                        text: "CLOSE".to_string(),
                                    });

                                    commands.push(((),
                                    pos.clone(),
                                    Popup {
                                        popup_type: PopupType::TextOutput,
                                        options: buttons,
                                        bounding_box: Rect::with_size(pos.x, pos.y, 30, 10),
                                        target: None,
                                        text: Some(vec!["Load a disk to use computer.".to_string()]),
                                    }));
                                },
                                ComputerState::Loaded => {
                                    let mut buttons = Vec::new();
                                    buttons.push(Button {
                                        action: Actions::Compile,
                                        button_area: Rect::with_size(pos.x+1, pos.y+1, 12, 1),
                                        text: "COMPILE".to_string(),
                                    });
                                    buttons.push(Button {
                                        action: Actions::StackDump,
                                        button_area: Rect::with_size(pos.x+1, pos.y+2, 12, 1),
                                        text: "STACK DUMP".to_string(),
                                    });
                                    buttons.push(Button {
                                        action: Actions::Run,
                                        button_area: Rect::with_size(pos.x+1, pos.y+3, 12, 1),
                                        text: "RUN".to_string(),
                                    });
                                    buttons.push(Button {
                                        action: Actions::Unload,
                                        button_area: Rect::with_size(pos.x+1, pos.y+4, 12, 1),
                                        text: "UNLOAD".to_string(),
                                    });
                
                                    commands.push(((),
                                        pos.clone(),
                                        Popup {
                                            popup_type: request.popup_type,
                                            options: buttons,
                                            bounding_box: Rect::with_size(pos.x, pos.y, 12, 8),
                                            target: request.target,
                                            text: None,
                                        }
                                    ));
                                },
                                ComputerState::Compiling => { },
                                ComputerState::Running => {
                                    let mut buttons = Vec::new();
                                    buttons.push(Button {
                                        action: Actions::StopRun,
                                        button_area: Rect::with_size(pos.x+1, pos.y+1, 12, 1),
                                        text: "STOP RUN".to_string(),
                                    });
                
                                    commands.push(((),
                                        pos.clone(),
                                        Popup {
                                            popup_type: request.popup_type,
                                            options: buttons,
                                            bounding_box: Rect::with_size(pos.x, pos.y, 12, 8),
                                            target: request.target,
                                            text: None,
                                        }
                                    ));
                                }
                            }
                        }
                    }
                },
                PopupType::EndGame => {
                    let mut buttons = Vec::new();
                    buttons.push(Button {
                        action: Actions::EndGame,
                        button_area: Rect::with_size(pos.x+20, pos.y+11, 10, 1),
                        text: "END GAME".to_string(),
                    });

                    commands.push(((),
                    pos.clone(),
                    Popup {
                        popup_type: PopupType::TextOutput,
                        options: buttons,
                        bounding_box: Rect::with_size(pos.x, pos.y, 30, 12),
                        target: None,
                        text: Some(vec![String::from("Something is wrong!"),
                                        String::from(""),
                                        String::from("The hack made a monster!"),
                                        String::from(""),
                                        String::from("It smells of decay"),
                                        String::from("and social media..."),
                                        String::from(""),
                                        String::from("It attacks you and you die!")
                                    ]),
                    }));
                },
            }

            commands.remove(*entity);        
        });
    
    let mut close_requests = <(Entity, &ClosePopupRequest)>::query();

    close_requests
        .iter(ecs)
        .for_each(|(entity, close_popup_request)| {
            if let Some(close_target) = close_popup_request.target {
                commands.remove(close_target);
            }
            else
            {
                let mut popups = <(Entity, &Popup)>::query();
                popups
                    .iter(ecs)
                    .for_each(|(open_popup, _)| {
                        commands.remove(*open_popup);
                    });
            }
            commands.remove(*entity);
        });
}