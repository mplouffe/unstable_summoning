use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Name)]
#[write_component(Cursor)]
#[read_component(Player)]
#[read_component(Disk)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] mouse_input: &MouseInput,
) 
{
    // update cursor position
    let mut cursors = <(Entity, &Cursor)>::query();

    let (cursor_entity, mut cursor) = cursors
        .iter(ecs)
        .find_map(|(entity, cursor)| Some((*entity, *cursor)))
        .unwrap();
    
    if !cursor.is_active {
        commands.add_component(cursor_entity, mouse_input.mouse_point_bg);
    }

    // click on things
    match mouse_input.left_click {
        ClickState::Released => {
            let mut positions = <(Entity, &Point, &Name)>::query()
                .filter(!component::<Cursor>());
        
            let mut cursor_target_updated = false;
            positions
                .iter(ecs)
                .filter(|(_, pos, _)| **pos == mouse_input.mouse_point_bg)
                .for_each(|(entity, _pos, _name)| {
                    let entity_ref = ecs.entry_ref(*entity).unwrap();

                    if let Ok(_disk) = entity_ref.get_component::<Disk>()
                    {
                        cursor.is_active = true;
                        cursor_target_updated = true;
                    }
                    else if let Ok(_player) = entity_ref.get_component::<Player>()
                    {
                        cursor.is_active = true;
                        cursor_target_updated = true;
                    }
                });
            
            if cursor_target_updated {
                commands.add_component(cursor_entity, cursor);
                commands.add_component(cursor_entity, mouse_input.mouse_point_bg);
            } else {
                if cursor.is_active {
                    cursor.is_active = false;
                    commands.add_component(cursor_entity, cursor);
                    commands.add_component(cursor_entity, mouse_input.mouse_point_bg);
                }
            }
        },
        _ => {}
    };
}
