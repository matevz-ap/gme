//! Shows how to render simple primitive shapes with a single color.

use std::time::Duration;

use bevy::{prelude::*, winit::{WinitSettings, UpdateMode}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource( WinitSettings {
            focused_mode: UpdateMode::Reactive {
                wait: Duration::from_secs(10),
            },
            unfocused_mode: UpdateMode::ReactiveLowPower {
                wait: Duration::from_secs(10),
            },
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_circle, sync_player_camera))
        .run();
}
#[derive(Component)]
struct Player;

#[derive(Component)]
struct MyGameCamera;


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere{
            radius: 4.0,
            subdivisions: 10,
        }).unwrap()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere{
            radius: 0.5,
            subdivisions: 10,
        }).unwrap()),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(4.0, 4.0, 0.0),
        ..default()
    }).insert(Player);
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, MyGameCamera));
}

fn move_circle(time: Res<Time>, mut query: Query<(&Player, &mut Transform)>, keyboard_input: ResMut<Input<KeyCode>>) {
    for (_, mut transform) in query.iter_mut() {
        // let angle = time.delta_seconds() * speed;
        let angle: f32 = 0.2;
        if keyboard_input.pressed(KeyCode::W) {
            let center_x: f32 = 0.;
            let center_y: f32 = 0.;
            let curr_x = transform.translation.x.clone();
            let curr_y = transform.translation.y.clone();
            transform.translation.x = center_x + (curr_x - center_x) * angle.cos() - (curr_y - center_y) * angle.sin();
            transform.translation.y = center_y + (curr_x - center_x) * angle.sin() + (curr_y - center_y) * angle.cos();
        }

        if keyboard_input.pressed(KeyCode::S) {
            let center_x: f32 = 0.;
            let center_y: f32 = 0.;
            let curr_x = transform.translation.x.clone();
            let curr_y = transform.translation.y.clone();
            transform.translation.x = center_x + (curr_x - center_x) * angle.cos() + (curr_y - center_y) * angle.sin();
            transform.translation.y = center_y + (curr_x - center_x) * angle.sin() - (curr_y - center_y) * angle.cos();
        } 

        if keyboard_input.pressed(KeyCode::A) {
            let center_x: f32 = 0.;
            let center_z: f32 = 0.;
            let curr_x = transform.translation.x.clone();
            let curr_z = transform.translation.z.clone();
            transform.translation.x = center_x + (curr_x - center_x) * angle.cos() - (curr_z - center_z) * angle.sin();
            transform.translation.z = center_z + (curr_x - center_x) * angle.sin() + (curr_z - center_z) * angle.cos();
        }
        
    }
}

fn sync_player_camera(
    mut player: Query<(&Player, &mut Transform), Without<MyGameCamera>>,
    mut camera: Query<(&MyGameCamera, &mut Transform), With<MyGameCamera>>,
) {
    for (_, mut player_transform) in player.iter_mut() {
        for (_, mut camera_transform) in camera.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y + 4.5;
            camera_transform.translation.z = player_transform.translation.z + 9.0;
        }
    }
}