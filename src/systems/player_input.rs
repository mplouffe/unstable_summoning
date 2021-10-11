use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Name)]
#[read_component(Cursor)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] mouse_input: &MouseInput,
) 
{
    let mut cursors = <(Entity, &Point)>::query()
        .filter(component::<Cursor>());

    let (cursor_entity, destination) = cursors
        .iter(ecs)
        .find_map(|(entity, _pos)| Some((*entity, mouse_input.mouse_point)))
        .unwrap();
    
    commands.add_component(cursor_entity, destination);

    match mouse_input.left_click {
        ClickState::Released => {
            println!("Released");
            let mut positions = <(Entity, &Point, &Name)>::query()
                .filter(!component::<Cursor>());
        
            positions
                .iter(ecs)
                .filter(|(_, pos, _)| **pos == mouse_input.mouse_point)
                .for_each(|(entity, pos, name)| {
                    println!("collision found");
                    if let Ok(liquid) = ecs.entry_ref(*entity)
                        .unwrap()
                        .get_component::<Liquid>()
                    {
                        println!("clicked Liquid");
                    }
                    
                    if let Ok(player) = ecs.entry_ref(*entity)
                        .unwrap()
                        .get_component::<Player>()
                    {
                        println!("clicked player");
                    }
                });
        },
        _ => {}
    };
}
