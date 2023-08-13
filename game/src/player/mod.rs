use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector3, pool::Handle},
    resource::model::{Model, ModelResourceExtension},
    scene::{node::Node, Scene},
};

use self::{camera::create_camera, movement::MovementController};

mod camera;
mod movement;

pub struct Player {
    _model: Handle<Node>,
    pub camera: Handle<Node>,
    pub movement: MovementController,
}

impl Player {
    pub async fn new(resource_manager: ResourceManager, scene: &mut Scene) -> Self {
        let model = resource_manager
            .request::<Model, _>("data/models/paladin/paladin.fbx")
            .await
            .unwrap()
            .instantiate(scene);

        scene.graph[model]
            .local_transform_mut()
            .set_position(Vector3::new(0.0, 0.0, 0.0))
            .set_scale(Vector3::new(0.02, 0.02, 0.02));

        let camera = create_camera(&mut scene.graph, resource_manager).await;

        Self {
            _model: model,
            camera,
            movement: MovementController::new(),
        }
    }

    pub fn update(&mut self, _scene: &mut Scene) {}
}
