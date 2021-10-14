
use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(PopupRequest)]
#[read_component(Popup)]
pub fn popup(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    let mut requests = <(Entity, &Point, &PopupRequest)>::query();
    let mut popups_to_close = Vec::new();
    
    requests
        .iter(ecs)
        .for_each(|(entity, pos, request)| {
            if !request.open {
                popups_to_close.push(request.popup_type);
            }
            else 
            {
                match request.popup_type {
                    PopupType::UnloadedDisk => {
                            let mut buttons = Vec::new();
                            buttons.push(Button {
                                action: Actions::Look,
                                button_area: Rect::with_size(pos.x+1, pos.y+1, 12, 1),
                                text: "LOOK".to_string(),
                            });
                            buttons.push(Button {
                                action: Actions::RubberDuck,
                                button_area: Rect::with_size(pos.x+1, pos.y+2, 12, 1),
                                text: "RUBBER DUCK".to_string(),
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
                                    width: 12,
                                    height: 8,
                                    target: request.target,
                                }
                            ));
                    },
                    PopupType::TextOutput => { }
                }
            }

            commands.remove(*entity);        
        });

    let mut popups = <(Entity, &Popup)>::query();

    popups
        .iter(ecs)
        .for_each(|(entity, popup)| {
            if popups_to_close.iter().any(|pops_to_close|  *pops_to_close == popup.popup_type) {
                commands.remove(*entity);
            }
        });
}