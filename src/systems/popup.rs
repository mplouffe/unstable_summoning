
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(PopupRequest)]
#[write_component(ClosePopupRequest)]
#[read_component(Popup)]
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
                            width: 12,
                            height: 8,
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
                                width: 30,
                                height: 10,
                                target: None,
                                text: Some(popup_text.clone()),
                            }));
                    }
                }
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