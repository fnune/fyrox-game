use fyrox::{
    asset::manager::ResourceManager,
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
    },
    resource::texture::{Texture, TextureWrapMode},
    scene::{
        base::BaseBuilder,
        camera::{CameraBuilder, SkyBox, SkyBoxBuilder},
        graph::Graph,
        node::Node,
        pivot::PivotBuilder,
        transform::TransformBuilder,
    },
};

pub async fn create_camera(graph: &mut Graph, resource_manager: ResourceManager) -> Handle<Node> {
    let camera = CameraBuilder::new(
        BaseBuilder::new().with_local_transform(
            TransformBuilder::new()
                .with_local_position(Vector3::new(0.0, 0.0, -5.5))
                .build(),
        ),
    )
    .with_z_far(48.0)
    .with_skybox(create_skybox(resource_manager).await)
    .build(graph);

    PivotBuilder::new(
        BaseBuilder::new()
            .with_local_transform(
                TransformBuilder::new()
                    .with_local_position(Vector3::new(0.0, 0.0, 0.0))
                    .with_local_rotation(UnitQuaternion::from_axis_angle(
                        &Vector3::x_axis(),
                        70.0f32.to_radians(),
                    ))
                    .build(),
            )
            .with_children(&[camera]),
    )
    .build(graph);

    camera
}

async fn create_skybox(resource_manager: ResourceManager) -> SkyBox {
    let (front, back, left, right, top, bottom) = fyrox::core::futures::join!(
        resource_manager.request::<Texture, _>("data/textures/skybox/front.jpg"),
        resource_manager.request::<Texture, _>("data/textures/skybox/back.jpg"),
        resource_manager.request::<Texture, _>("data/textures/skybox/left.jpg"),
        resource_manager.request::<Texture, _>("data/textures/skybox/right.jpg"),
        resource_manager.request::<Texture, _>("data/textures/skybox/up.jpg"),
        resource_manager.request::<Texture, _>("data/textures/skybox/down.jpg")
    );

    let skybox = SkyBoxBuilder {
        front: Some(front.unwrap()),
        back: Some(back.unwrap()),
        left: Some(left.unwrap()),
        right: Some(right.unwrap()),
        top: Some(top.unwrap()),
        bottom: Some(bottom.unwrap()),
    }
    .build()
    .unwrap();

    let cubemap = skybox.cubemap();
    let mut data = cubemap.as_ref().unwrap().data_ref();
    data.set_s_wrap_mode(TextureWrapMode::ClampToEdge);
    data.set_t_wrap_mode(TextureWrapMode::ClampToEdge);

    skybox
}
