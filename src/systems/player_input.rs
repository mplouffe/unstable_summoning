use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Cursor)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] mouse_pos: &Point,
    #[resource] key: &Option<VirtualKeyCode>,
) 
{
    let mut cursors = <(Entity, &Point)>::query()
        .filter(component::<Cursor>());

    let (cursor_entity, destination) = cursors
        .iter(ecs)
        .find_map(|(entity, _pos)| Some((*entity, *mouse_pos)))
        .unwrap();
    
    commands.add_component(cursor_entity, destination);
}
