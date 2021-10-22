use crate::prelude::*;

#[system]
#[write_component(ActionRequest)]
#[write_component(Computer)]
#[write_component(Render)]
#[write_component(Point)]
#[read_component(Disk)]
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
                    if let Some(target_entity) = &action_request.target {
                        let entity_ref = ecs.entry_ref(*target_entity).unwrap();
                        if let Ok(disk) = entity_ref.get_component::<Disk>()
                        {
                            commands.push(((),
                                Point::new(10, 5),
                                PopupRequest {
                                    popup_type: PopupType::TextOutput,
                                    target: None,
                                    text: Some(vec![disk.disk_label.clone()]),
                                }
                            ));
                        }
                    }
                },
                Actions::Smell => {
                    println!("Smell action selected and handled");
                },
                Actions::Load => {
                    let mut computers = <(Entity, &Computer)>::query()
                        .iter(ecs)
                        .filter(|(_, computer)| computer.computer_state != ComputerState::Loaded)
                        .nth(0);

                    if let Some((computer_entity, computer)) = computers {
                        if let Some(mut target_disk) = &action_request.target {
                            let disk_ref = ecs.entry_ref(target_disk).unwrap();
                            if let Ok(render) = disk_ref.get_component::<Render>()
                            {
                                let mut disk_render = render.clone();
                                disk_render.render = false;
                                commands.add_component(target_disk, disk_render);
                            }
                            if let Ok(point) = disk_ref.get_component::<Point>()
                            {
                                let mut disk_point = point.clone();
                                disk_point.x = -1;
                                disk_point.y = -1;
                                commands.add_component(target_disk, disk_point);
                            }
                            let mut computer = computer.clone();
                            computer.computer_state = ComputerState::Loaded;
                            computer.loaded_disk = Some(target_disk);
                            commands.add_component(*computer_entity, computer);
                        }
                    }
                    else
                    {
                        commands.push(((),
                            Point::new(10, 5),
                            PopupRequest {
                                popup_type: PopupType::TextOutput,
                                target: None,
                                text: Some(vec!["There are no available".to_string(), "computers to load the disk.".to_string()]),
                            }
                        ));
                    }
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
                    if let Some(popup_entity) = action_request.target {
                        commands.push(((),
                            ClosePopupRequest {
                                target: Some(popup_entity)
                            }
                        ));
                    }
                }
            }

            commands.remove(*entity);
        });
}