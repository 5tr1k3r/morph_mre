use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>
) {
    // works
    // let mesh_handle = assets.load("movement_indicator.glb#Scene0");
    // commands.spawn(SceneBundle {
    //     scene: mesh_handle.clone(),
    //     ..default()
    // });

    // does not work (regardless of the material)
    let mesh_handle = assets.load("movement_indicator.glb#Mesh0/Primitive0");
    // let material_handle = assets.load("movement_indicator.glb#Material0");
    commands.spawn(PbrBundle {
        mesh: mesh_handle.clone(),
        // material: material_handle.clone(),
        ..default()
    });


    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
