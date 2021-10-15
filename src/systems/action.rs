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
        .iter_mut(ecs)
        .for_each(|(entity, action_request)| {
            match action_request.action {
                Actions::Look => {
                    println!("Look action selected and handled");
                },
                Actions::RubberDuck => {
                    println!("RubberDuck action selected and handled");
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
                }
            }

            commands.remove(*entity);
        });
}