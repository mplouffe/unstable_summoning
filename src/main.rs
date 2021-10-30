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

    pub const DISPLAY_WIDTH: i32 = 21;
    pub const DISPLAY_HEIGHT: i32 = 11;
    pub const TILE_SIZE: i32 = 64;
    pub const BG_LAYER: usize = 0;
    pub const SPRITE_LAYER: usize = 1;
    pub const HUD_LAYER: usize = 2;
    pub const POPUP_LAYER: usize = 3;

    pub use crate::turn_state::*;
    pub use crate::map::*;
    pub use crate::components::*;
    pub use crate::systems::*;
    pub use crate::spawner::*;
}

use prelude::*;
use rand::thread_rng;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    game_start_systems: Schedule,
    frame: usize,
    timer: f32,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let map = Map::new();

        let animations: Vec<Vec<usize>> = vec![vec![3, 4, 5], vec![56,57,58,59]];
        resources.insert(animations);

        spawn_title_screen(&mut ecs);

        resources.insert(map);
        resources.insert(TurnState::GameStart);
        let time_info = TimerInfo {
            timer: 0.0,
            frame: 0,
        };

        resources.insert(time_info);  

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            game_start_systems: build_game_start_scheduler(),
            frame: 0,
            timer: 0.0,
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
        ctx.set_active_console(HUD_LAYER);
        ctx.print_color_centered(20, GREEN, BLACK, "ld49: Unstable Trans-dimensional Hacking.");
        ctx.print_color_centered(25, WHITE, BLACK,
            "Three years ago your Uncle Bill mysteriously disappeared...");
        ctx.print_color_centered(26, WHITE, BLACK,
            "In his will he left you the keys to his 'lab' and a note...");
        ctx.print_color_centered(30, RED, BLACK,
            "'Only you can save us now!'");
        ctx.print_color_centered(31, RED, BLACK,
            "'Compile the codes. Perform the rituals. Hack the planet!!!'");
        ctx.print_color_centered(32, RED, BLACK,
            "'Don't worry. It's totally not powered by the Dark Arts...'");

        ctx.print_color_centered(36, GREEN, BLACK,
            "Press space to play.");
        
        self.game_start_systems.execute(&mut self.ecs, &mut self.resources);

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
        let mut thread_rng = thread_rng();
        spawn_player(&mut self.ecs);
        spawn_cursor(&mut self.ecs);

        let animations: Vec<Vec<usize>> = vec![vec![3, 4, 5], vec![56,57,58,59]];
        self.resources.insert(animations);

        let disk_positions = [ 
            Point::new(2, 8),
            Point::new(4, 8),
            Point::new(6, 8),
            Point::new(8, 8),
            Point::new(12, 8),
            Point::new(14, 8),
            Point::new(16, 8),
            Point::new(18, 8),
        ];
        let computer_positions = [
            ( Point::new(5, 5), PartName::LeftComputer ),
            ( Point::new(15, 5), PartName::RightComputer ),
        ];

        let piping = [
            ( Point::new(11,5), 51, PartName::Platform ),
            ( Point::new(9, 5), 51, PartName::Platform ),
            ( Point::new(11, 6), 53, PartName::DimensionalButton ),
            ( Point::new(10, 6), 52, PartName::DimensionalButton ),
            ( Point::new(9, 6), 53, PartName::DimensionalButton ),
            ( Point::new(10, 7), 51, PartName::DimensionalButton ),
            ( Point::new(8, 6), 50, PartName::LeftComputer ),
            ( Point::new(12, 6), 50, PartName::RightComputer ),
            ( Point::new(7, 6), 50, PartName::LeftComputer ),
            ( Point::new(13, 6), 50, PartName::RightComputer ),
            ( Point::new(6, 6), 50, PartName::LeftComputer ),
            ( Point::new(14, 6), 50, PartName::RightComputer ),
            ( Point::new(7, 6), 50, PartName::LeftComputer ),
            ( Point::new(13, 6), 50, PartName::RightComputer ),
            ( Point::new(5, 6), 48, PartName::LeftComputer ),
            ( Point::new(15, 6), 49, PartName::RightComputer ),
        ];

        spawn_disks(&mut self.ecs, &mut thread_rng, &disk_positions);
        spawn_computers(&mut self.ecs, &computer_positions);
        spawn_infrastructure(&mut self.ecs);
        spawn_pipes(&mut self.ecs, &piping);

        self.resources.insert(map);
        self.resources.insert(TurnState::AwaitingInput);
        self.timer = 0.0;
        self.frame = 0;
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
        ctx.set_active_console(3);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);

        let mouse_pos_bg_layer = ctx.mouse_pos();
        ctx.set_active_console(2);
        let mouse_pos_hud_layer = ctx.mouse_pos();

        let mut mouse_input = MouseInput {
            mouse_point_bg: Point::from_tuple(mouse_pos_bg_layer),
            mouse_point_hud: Point::from_tuple(mouse_pos_hud_layer),
            left_click: ClickState::Unclicked
        };

        if let Some(old_input) = self.resources.get_mut::<MouseInput>() {
            mouse_input.left_click = match old_input.left_click {
                ClickState::Clicked => ClickState::Held,
                ClickState::Released => ClickState::Unclicked,
                _ => old_input.left_click
            };
        }

        if ctx.left_click {
            let old_state = mouse_input.left_click;
            mouse_input.left_click = match old_state {
                ClickState::Held => ClickState::Released,
                ClickState::Unclicked => ClickState::Clicked,
                _ => ClickState::Unclicked
            };
        }
        self.resources.insert(mouse_input);

        self.timer += ctx.frame_time_ms;
        if self.timer > 66.0 {
            self.timer = 0.0;
            self.frame += 1;
        }
        let time_info = TimerInfo {
            timer: self.timer,
            frame: self.frame,
        };

        self.resources.insert(time_info);     
        
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

    let mut sprites = Vec::new();
    for y in 0..12 {
        for x in 0..12 {
            sprites.push(Sprite::new(Rect::with_size(x*32, 352-(y*32), 32, 32)));
        }
    }

    let context = BTermBuilder::new()
        .with_title("ld49: Unstable Summoning")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(64, 64)
        .with_resource_path("resources/")
        .with_font("unstablefont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "unstablefont.png")                 // BG
        .with_sprite_console(DISPLAY_WIDTH*TILE_SIZE, DISPLAY_HEIGHT*TILE_SIZE, 0)       // Sprite Layer
        .with_simple_console_no_bg(DISPLAY_WIDTH*4, DISPLAY_HEIGHT*4, "terminal8x8.png")          // HUD Layer
        .with_sparse_console(DISPLAY_WIDTH*4, DISPLAY_HEIGHT*4, "terminal8x8.png")                // Popup Layer
        .with_sprite_sheet(SpriteSheet {
            filename: "resources/sprite_sheet.png".to_string(),
            sprites,
            backing: None
        })
        .with_vsync(false)
        .build()?;
    
    main_loop(context, State::new())
}
