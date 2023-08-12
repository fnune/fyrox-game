use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector3, pool::Handle},
    resource::model::{Model, ModelResourceExtension},
    scene::{node::Node, Scene},
};

use self::camera::create_camera;

mod camera;

pub struct Player {
    model: Handle<Node>,
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

        create_camera(&mut scene.graph, resource_manager).await;

        Self { model }
    }

    pub fn update(&mut self, _scene: &mut Scene) {}
}
