//! Shows how to render simple primitive shapes with a single color.

use std::time::Duration;
use bevy::input::mouse::MouseMotion;
use bevy::{prelude::*, winit::{WinitSettings, UpdateMode}};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MyGameCamera;

#[derive(Component)]
struct Snowball;


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
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere{
            radius: 0.5,
            subdivisions: 10,
        }).unwrap()),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.0, 4.5).looking_at(Vec3 { x: 0.0, y: 4.5, z: 4.5 }, Vec3::Y),
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
        transform: Transform::from_xyz(0.0, 0.0, 14.0).looking_at(Vec3::Y, Vec3::Y),
        ..default()
    }, MyGameCamera));
}

fn move_circle(
    time: Res<Time>, 
    mut query: Query<(&Player, &mut Transform)>, 
    keyboard_input: ResMut<Input<KeyCode>>, 
    mut motion_evr: EventReader<MouseMotion>,
) {
    for (_, mut transform) in query.iter_mut() {
        let angle:f32 = 0.2;

        if keyboard_input.pressed(KeyCode::W) {
            transform.translate_around(Vec3::ZERO, Quat::from_rotation_x(-angle));
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translate_around(Vec3::ZERO, Quat::from_rotation_x(angle));
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translate_around(Vec3::ZERO, Quat::from_rotation_y(-angle));
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translate_around(Vec3::ZERO, Quat::from_rotation_y(angle));
        }
        
        for ev in motion_evr.read() {
            transform.rotation *= Quat::from_rotation_y(ev.delta.x * 0.01);
            transform.rotation *= Quat::from_rotation_x(ev.delta.y * 0.01);
        }
    }   
}

fn sync_player_camera(
    mut player: Query<(&Player, &mut Transform), Without<MyGameCamera>>,
    mut camera: Query<(&MyGameCamera, &mut Transform), With<MyGameCamera>>,
) {
    for (_, player_transform) in player.iter_mut() {
        for (_, mut camera_transform) in camera.iter_mut() {
            camera_transform.translation = player_transform.translation + player_transform.forward() * 4.0;
            camera_transform.translation.clamp(Vec3::splat(5.0), Vec3::splat(10.0));
            camera_transform.look_at(player_transform.translation, Vec3::Y)
        }
    }
}

fn throw_snowball(
    mut player: Query<(&Player, &mut Transform), Without<MyGameCamera>>,
    keyboard_input: ResMut<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (_, player_transform) in player.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            let snowball = player_transform.translation + player_transform.forward() * 1.0;
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::try_from(shape::Icosphere{
                    radius: 0.2,
                    subdivisions: 10,
                }).unwrap()),
                transform: Transform::from_translation(snowball),
                ..default()
            });
        }
    }

}

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
        .add_systems(Update, (move_circle, sync_player_camera, throw_snowball))
        .run();
}