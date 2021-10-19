use crate::prelude::*;

#[system]
#[write_component(ActionRequest)]
#[write_component(Computer)]
#[write_component(Render)]
pub fn action(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    let mut action_requests = <(Entity, &ActionRequest)>::query();

    action_requests
        .iter(ecs)
        .for_each(|(entity, action_request)| {
            match action_request.action {
                Actions::Look => {
                    println!("Look selected and handled");
                    if let Some(target_entity) = &action_request.target {
                        println!("found target entity");
                        let entity_ref = ecs.entry_ref(*target_entity).unwrap();
                        
                        if let Ok(disk) = entity_ref.get_component::<Disk>()
                        {
                            println!("got disk, creating text popup");
                            commands.push(((),
                                Point::new(10, 5),
                                PopupRequest {
                                    popup_type: PopupType::TextOutput,
                                    open: true,
                                    target: None,
                                    text: Some(disk.disk_label.clone()),
                                }
                            ));
                        }
                    }
                },
                Actions::Smell => {
                    println!("Smell action selected and handled");
                },
                Actions::Load => {
                    println!("Load action selected and handled");
                },
                Actions::Compile => {
                    println!("Compile action selected and handled");
                },
                Actions::StackDump => {
                    println!("StackDump action selected and handled");
                },
                Actions::Run => {
                    println!("Run action selected and handled");
                },
                Actions::CloseWindow => {
                    commands.push(((),
                        PopupRequest {
                            popup_type: PopupType::TextOutput,
                            open: false,
                            target: None,
                            text: None,
                        }));
                }
            }

            commands.remove(*entity);
        });
}