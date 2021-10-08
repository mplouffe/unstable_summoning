use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Cursor)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) 
{
    let mut cursors = <(Entity, &Point)>::query()
        .filter(component::<Cursor>());

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0,0),
        };

        let (cursor_entity, destination) = cursors
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();
        
        commands.add_component(cursor_entity, destination);

        *turn_state = TurnState::PlayerTurn;
    }
}
