//! Game project.
mod player;

use fyrox::{
    core::{color::Color, futures::executor::block_on, pool::Handle},
    event_loop::ControlFlow,
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::Scene,
};
use level::Level;
use player::Player;

mod level;

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, _context: PluginRegistrationContext) {
        // Register your scripts here.
    }

    fn create_instance(
        &self,
        _override_scene: Handle<Scene>,
        context: PluginContext,
    ) -> Box<dyn Plugin> {
        Box::new(Game::new(context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    level: Level,
    player: Player,
}

impl Game {
    pub fn new(context: PluginContext) -> Self {
        let mut scene = Scene::new();
        scene.ambient_lighting_color = Color::opaque(150, 150, 150);

        let player = block_on(Player::new(context.resource_manager.clone(), &mut scene));
        let level = block_on(Level::new(context.resource_manager.clone(), &mut scene));
        Self {
            scene: context.scenes.add(scene),
            player,
            level,
        }
    }
}

impl Plugin for Game {
    fn update(&mut self, context: &mut PluginContext, _control_flow: &mut ControlFlow) {
        let scene = &mut context.scenes[self.scene];
        self.player.update(scene);
    }
}
