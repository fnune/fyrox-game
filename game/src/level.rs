use fyrox::{
    asset::manager::ResourceManager,
    core::pool::Handle,
    resource::model::{Model, ModelResourceExtension},
    scene::{node::Node, Scene},
};

pub struct Level {
    root: Handle<Node>,
}

impl Level {
    pub async fn new(resource_manager: ResourceManager, scene: &mut Scene) -> Self {
        let root = resource_manager
            .request::<Model, _>("data/levels/level.rgs")
            .await
            .unwrap()
            .instantiate(scene);
        Self { root }
    }
}
