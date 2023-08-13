use fyrox::{
    asset::manager::ResourceManager,
    core::{algebra::Vector3, arrayvec::ArrayVec, log::Log, math::ray::Ray},
    resource::model::{Model, ModelResourceExtension},
    scene::{
        graph::physics::{Intersection, RayCastOptions},
        Scene,
    },
};

pub struct MovementController {
    _target: Option<Vector3<f32>>,
}

impl MovementController {
    pub fn new() -> Self {
        return Self { _target: None };
    }

    pub async fn handle_move_command(&self, ray: Ray, scene: &mut Scene, resource_manager: ResourceManager) {
        let mut intersections = ArrayVec::<Intersection, 1>::new();

        scene.graph.physics.cast_ray(
            RayCastOptions {
                ray_origin: ray.origin.into(),
                ray_direction: ray.dir,
                max_len: ray.dir.norm(),
                // FIXME: this intersects with anything. I need it to intersect with the ground.
                groups: Default::default(),
                sort_results: true,
            },
            &mut intersections,
        );

        if let Some(intersection) = intersections.first() {
            Log::info(format!("Intersection: {:#?}", intersection));

            {
                // TODO: this block just here for debugging that the click happened, and where.
                // Remove it once actual movement is implemented.
                let model = resource_manager
                    .request::<Model, _>("data/models/paladin/paladin.fbx")
                    .await
                    .unwrap()
                    .instantiate(scene);

                scene.graph[model]
                    .local_transform_mut()
                    .set_position(Vector3::new(
                        intersection.position.x,
                        intersection.position.y,
                        intersection.position.z,
                    ))
                    .set_scale(Vector3::new(0.01, 0.01, 0.01));
            }
        }
    }
}
