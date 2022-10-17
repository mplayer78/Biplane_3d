// //! Loads and renders a glTF file as a scene.

use std::{f32::consts::PI};

use bevy::{prelude::*, transform};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 4.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(p1_control)
        .run();
}

const INITIAL_ROTATION: Quat = Quat::from_xyzw(0.0, 0.0, 0.0, 1.0);

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    let translation = Vec3::new(50.0, 50.0, 50.0);

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(translation)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    
    commands.spawn_bundle(SpotLightBundle {
        transform: Transform::from_xyz(0.0, 0.0, 20.0)
            .looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
        spot_light: SpotLight {
            intensity: 16000.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::GREEN,
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.8,
            ..default()
        },
        ..default()
    });
    
    commands.spawn()
        .insert_bundle(SceneBundle {
            scene: asset_server.load("sopwith_camel.gltf#Scene0"),
            transform: Transform {
                rotation: INITIAL_ROTATION,
                translation: Vec3::new(0.0, 5.6 / 2.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Heading(INITIAL_ROTATION))
        .insert(Roll(INITIAL_ROTATION))
        .insert(Pitch(INITIAL_ROTATION))
        .insert(Player1);
}

#[derive(Component)]
struct Player1;

#[derive(Component)]
struct Heading(Quat);

#[derive(Component)]
struct Roll(Quat);

#[derive(Component)]
struct Pitch(Quat);

const TURN_SPEED: f32 = 0.1;
const PITCH_SPEED: f32 = 0.1;

fn p1_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut movement: Query<(&mut Transform, &mut Heading, &mut Roll, &mut Pitch), With<Player1>>,
) {
        for (mut transform, mut heading, mut roll, mut pitch) in movement.iter_mut() {
            let ( axis, angle ) = roll.0.to_axis_angle();
            if keyboard_input.pressed(KeyCode::Left) {
                if angle * axis.z < 1.0 {
                    roll.0 *= Quat::from_axis_angle(Vec3::Z, PITCH_SPEED );
                }
            }   
            if keyboard_input.pressed(KeyCode::Right) {
                println!("Right");
                if angle * axis.z > -1.0 {
                    roll.0 *= Quat::from_axis_angle(Vec3::Z, -PITCH_SPEED);
                } 
            }            
            if !keyboard_input.any_pressed([KeyCode::Left, KeyCode::Right]) {
                if angle * axis.z > 0.0 {
                    roll.0 *= Quat::from_axis_angle(Vec3::Z, -PITCH_SPEED);
                } else if angle * axis.z < 0.0 {
                    roll.0 *= Quat::from_axis_angle(Vec3::Z, PITCH_SPEED );
                }
            }
            if keyboard_input.pressed(KeyCode::Up) {
                pitch.0 *= Quat::from_axis_angle(Vec3::X, PITCH_SPEED );
            }
            if keyboard_input.pressed(KeyCode::Down) {
                pitch.0 *= Quat::from_axis_angle(Vec3::X, -PITCH_SPEED );
            }
            heading.0 *= Quat::from_axis_angle(Vec3::Y, angle * axis.z * -TURN_SPEED);
            transform.rotation = heading.0.mul_quat(roll.0).mul_quat(pitch.0);
        }
}