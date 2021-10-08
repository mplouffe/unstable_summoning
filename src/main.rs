mod components;
mod map;
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
    pub const TILE_SIZE: i32 = 64;
    pub const BG_LAYER: usize = 0;
    pub const SPRITE_LAYER: usize = 1;
    pub const HUD_LAYER: usize = 2;

    pub use crate::turn_state::*;
    pub use crate::map::*;
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
        let map = Map::new();
        let mut rng = RandomNumberGenerator::new();
        
        spawn_player(&mut ecs);
        spawn_cursor(&mut ecs);
        let flask_positions = [ 
            Point::new(2, 8),
            Point::new(4, 8),
            Point::new(6, 8),
            Point::new(8, 8),
            Point::new(11, 8),
            Point::new(13, 8),
            Point::new(15, 8),
            Point::new(17, 8),
        ];
        spawn_flasks(&mut ecs, &mut rng, &flask_positions);

        resources.insert(map);
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
        ctx.set_active_console(HUD_LAYER);
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
        ctx.set_active_console(SPRITE_LAYER);
        ctx.add_sprite(
            Rect::with_size(576, 172, 128, 128),
            1,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            2
        );
        ctx.add_sprite(
            Rect::with_size(566, 148, 128, 128),
            10,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            3
        );

        ctx.set_active_console(HUD_LAYER);
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

    // fn victory(&mut self, ctx: &mut BTerm) {
    //     ctx.set_active_console(HUD_LAYER);
    //     ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
    //     ctx.print_color_centered(4, WHITE, BLACK,
    //         "I don't know how you did it. I haven't coded a win condition yet...");
    //     ctx.print_color_centered(15, WHITE, RED,
    //         "You're clearly a l33t #@XXoR o_O");
    //     ctx.print_color_centered(17, GREEN, BLACK,
    //         "Press 1 to play again.");
        
    //     if let Some(VirtualKeyCode::Key1) = ctx.key {
    //         self.reset_game_state();
    //     }
    // }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let map = Map::new();
        let mut rng = RandomNumberGenerator::new();

        spawn_player(&mut self.ecs);
        spawn_cursor(&mut self.ecs);

        let flask_positions = [ 
            Point::new(2, 8),
            Point::new(4, 8),
            Point::new(6, 8),
            Point::new(8, 8),
            Point::new(11, 8),
            Point::new(13, 8),
            Point::new(15, 8),
            Point::new(17, 8),
        ];
        spawn_flasks(&mut self.ecs, &mut rng, &flask_positions);

        self.resources.insert(map);
        self.resources.insert(TurnState::AwaitingInput);
    }

    // fn advance_level(&mut self) {
    //     let player_entity = *<Entity>::query()
    //         .filter(component::<Player>())
    //         .iter(&mut self.ecs)
    //         .nth(0)
    //         .unwrap();

    //     let mut entities_to_keep = HashSet::new();
    //     entities_to_keep.insert(player_entity);
        
    //     let mut cb = CommandBuffer::new(&mut self.ecs);
    //     for e in Entity::query().iter(&self.ecs) {
    //         if !entities_to_keep.contains(e) {
    //             cb.remove(*e);
    //         }
    //     }
    //     cb.flush(&mut self.ecs);

    //     let mut player_level = 0;
    //     <&mut Player>::query()
    //         .iter_mut(&mut self.ecs)
    //         .for_each(|player| {
    //             player.level += 1;
    //             player_level = player.level;
    //         }
    //     );

    //     self.resources.insert(TurnState::AwaitingInput);
    // }
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
            // TurnState::Victory => self.victory(ctx),
            // TurnState::NextLevel => self.advance_level(),
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
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "unstablefont.png")                 // BG
        .with_sprite_console(DISPLAY_WIDTH*TILE_SIZE, DISPLAY_HEIGHT*TILE_SIZE, 0)      // Sprite Layer
        .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png")          // HUD Layer
        .with_sprite_sheet(
            SpriteSheet::new("resources/sprite_sheet.png")
                .add_sprite(Rect::with_size(0, 224, 32, 32))
                .add_sprite(Rect::with_size(32, 224, 32, 32))
                .add_sprite(Rect::with_size(64, 224, 32, 32))
                .add_sprite(Rect::with_size(96, 224, 32, 32))
                .add_sprite(Rect::with_size(128, 224, 32, 32))
                .add_sprite(Rect::with_size(160, 224, 32, 32))
                .add_sprite(Rect::with_size(192, 224, 32, 32))
                .add_sprite(Rect::with_size(224, 224, 32, 32))

                .add_sprite(Rect::with_size(0, 192, 32, 32))
                .add_sprite(Rect::with_size(32, 192, 32, 32))
                .add_sprite(Rect::with_size(64, 192, 32, 32))
                .add_sprite(Rect::with_size(96, 192, 32, 32))
                .add_sprite(Rect::with_size(128, 192, 32, 32))
                .add_sprite(Rect::with_size(160, 192, 32, 32))
                .add_sprite(Rect::with_size(192, 192, 32, 32))
                .add_sprite(Rect::with_size(224, 192, 32, 32))

                .add_sprite(Rect::with_size(0, 160, 32, 32))
                .add_sprite(Rect::with_size(32, 160, 32, 32))
                .add_sprite(Rect::with_size(64, 160, 32, 32))
                .add_sprite(Rect::with_size(96, 160, 32, 32))
                .add_sprite(Rect::with_size(128, 160, 32, 32))
                .add_sprite(Rect::with_size(160, 160, 32, 32))
                .add_sprite(Rect::with_size(192, 160, 32, 32))
                .add_sprite(Rect::with_size(224, 160, 32, 32))

                .add_sprite(Rect::with_size(0, 128, 32, 32))
                .add_sprite(Rect::with_size(32, 128, 32, 32))
                .add_sprite(Rect::with_size(64, 128, 32, 32))
                .add_sprite(Rect::with_size(96, 128, 32, 32))
                .add_sprite(Rect::with_size(128, 128, 32, 32))
                .add_sprite(Rect::with_size(160, 128, 32, 32))
                .add_sprite(Rect::with_size(192, 128, 32, 32))
                .add_sprite(Rect::with_size(224, 128, 32, 32))
        )
        .with_vsync(false)
        .build()?;
    
    main_loop(context, State::new())
}
