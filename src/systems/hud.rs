use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .nth(0)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1,
        "Science the Chemicals, Cursor keys to pick item, space to interact.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH*2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK)
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ",
            player_health.current,
            player_health.max
        ),
        ColorPair::new(WHITE, RED)
    );

    let (player, level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.level)))
        .unwrap();
    
    draw_batch.print_color_right(
        Point::new(SCREEN_WIDTH*2, (SCREEN_HEIGHT*2)-1),
        format!("Science Level: {}", level+1),
        ColorPair::new(YELLOW, BLACK)
    );

    draw_batch.submit(10000).expect("Batch error");
}