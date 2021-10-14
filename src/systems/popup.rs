
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
    let mut close_popup = false;
    
    requests
        .iter(ecs)
        .for_each(|(entity, pos, request)| {
            match request.popup_type {
                PopupType::ActionsInput => {
                    if request.open {
                        commands.push(((),
                            pos.clone(),
                            Popup {
                                options: vec![Actions::Look, Actions::RubberDuck, Actions::Load],
                                width: 12,
                                height: 8,
                            }
                        ));
                    } else {
                        close_popup = true;
                    }
                },
                PopupType::TextOutput => { }
            }

            commands.remove(*entity);        
        });

    let mut popups = <(Entity, &Popup)>::query();

    popups
        .iter(ecs)
        .for_each(|(entity, popup)| {
            if close_popup {
                commands.remove(*entity);
            }
        });
}