use std::collections::HashSet;

mod components;
mod systems;
mod turn_state;
mod spawner;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub const SCREEN_WIDTH: i32 = 40;
    pub const SCREEN_HEIGHT: i32 = 20;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::turn_state::*;
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use crate::spawner::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        
        spawn_player(&mut ecs);

        resources.insert(TurnState::GameStart);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(20, RED, BLACK, "Your experiment has ended.");
        ctx.print_color_centered(25, WHITE, BLACK,
            "Slain by a monster, your hero's journey has come to a \
            premature end.");
        ctx.print_color_centered(26, WHITE, BLACK,
            "The purpose of your experiments will never be known \
            as your notes were destroyed in the tragedy that killed you.");
        ctx.print_color_centered(30, YELLOW, BLACK,
            "Don't worry, you can always try again.");
        ctx.print_color_centered(35, GREEN, BLACK,
            "Press f to pay respects.");
        
        if let Some(VirtualKeyCode::F) = ctx.key {
            self.reset_game_state();
        }
    }

    fn game_start(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(20, GREEN, BLACK, "ld49: Unstable Summoning.");
        ctx.print_color_centered(25, WHITE, BLACK,
            "Welcome to the lab.");
        ctx.print_color_centered(26, WHITE, BLACK,
            "Combine elements and toss them into the Sciencefier!.");
        ctx.print_color_centered(30, RED, BLACK,
            "Don't worry, it's totally not powered by the dark arts.");
        ctx.print_color_centered(35, GREEN, BLACK,
            "Press space to play.");
        
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(4, WHITE, BLACK,
            "I don't know how you did it. I haven't coded a win condition yet...");
        ctx.print_color_centered(15, WHITE, RED,
            "You're clearly a l33t #@XXoR o_O");
        ctx.print_color_centered(17, GREEN, BLACK,
            "Press 1 to play again.");
        
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        spawn_player(&mut self.ecs);

        self.resources.insert(TurnState::AwaitingInput);
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();

        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        
        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs);

        let mut rng = RandomNumberGenerator::new();

        let mut player_level = 0;
        <(&mut Player)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player)| {
                player.level += 1;
                player_level = player.level;
            }
        );

        self.resources.insert(TurnState::AwaitingInput);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self.monster_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::GameStart => self.game_start(ctx),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("ld49: Unstable Summoning")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(64, 64)
        .with_resource_path("resources/")
        .with_font("unstablefont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "unstablefont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "unstablefont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png")
        .build()?;
    
    main_loop(context, State::new())
}