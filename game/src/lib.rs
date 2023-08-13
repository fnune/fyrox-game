//! Game project.
mod player;

use fyrox::{
    core::{algebra::Vector2, color::Color, futures::executor::block_on, pool::Handle},
    event::{DeviceEvent, ElementState, Event, WindowEvent},
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

    fn create_instance(&self, _override_scene: Handle<Scene>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
    _level: Level,
    player: Player,
    cursor: Vector2<f32>,
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
            _level: level,
            cursor: Vector2::new(0.0, 0.0),
        }
    }
}

impl Plugin for Game {
    fn update(&mut self, context: &mut PluginContext, _control_flow: &mut ControlFlow) {
        let scene = &mut context.scenes[self.scene];
        self.player.update(scene);
    }

    fn on_os_event(&mut self, event: &Event<()>, context: PluginContext, _control_flow: &mut ControlFlow) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::CursorMoved { position, .. } = event {
                // TODO: do I need a LogicalPosition? This is a PhysicalPosition.
                self.cursor = Vector2::new(position.x as f32, position.y as f32)
            };
        };

        if let Event::DeviceEvent { event, .. } = event {
            if let DeviceEvent::Button {
                state: ElementState::Pressed,
                button: 3,
            } = event
            {
                let scene = &mut context.scenes[self.scene];
                let camera = scene.graph[self.player.camera].as_camera();
                let screen_size = context
                    .graphics_context
                    .as_initialized_mut()
                    .renderer
                    .get_frame_bounds();

                let ray = camera.make_ray(self.cursor, screen_size);
                block_on(self.player.movement.handle_move_command(
                    ray,
                    scene,
                    context.resource_manager.clone(),
                ));
            }
        }
    }
}
