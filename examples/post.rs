use bevy::prelude::*;

//noinspection DuplicatedCode

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, move_cube_to_the_right_and_rotate_it)
        .run();
}

#[derive(Component)]
struct Cube;

#[derive(Component)]
struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3
}


fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // Spawn cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BEIGE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Cube)
        .insert(Velocity {
            linear: Vec3::new(3.0, 0.0, 0.0),
            angular: Vec3::new(0.0, 90f32.to_radians(), 0.0),
        });


    // Spawn Camera
    let player_camera_y_offset: f32 = 20.0;
    let player_camera_z_offset: f32 = 10.0;

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, player_camera_y_offset, player_camera_z_offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });



    // Spawn platform for reference
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::SEA_GREEN.into()),
        ..default()
    });


    // Add global light
    commands.insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1.0,
    });

}

fn move_cube_to_the_right_and_rotate_it(mut query: Query<(&mut Transform, &Velocity), With<Cube>>, time: Res<Time>) {
    let (mut transform, velocity) = query.single_mut();

    transform.translation += velocity.linear * time.delta_seconds();
    transform.rotation *= Quat::from_scaled_axis(velocity.angular * time.delta_seconds());
}
