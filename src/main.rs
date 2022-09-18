use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
    .add_startup_system(setup_camera)
    .add_startup_system(spawn_plane)
    .add_system(controls)
    .add_system(physics)
    .add_system(set_position_from_heading)
    .add_system(set_translation_from_position)
    .add_system(set_rotation_from_heading)
    .add_system(set_flip_from_direction)
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

const PLAYER_SPRITE: &str = "green_plane.png";

#[derive(Component)]
struct Aeroplane;

#[derive(Component)]
struct Heading {
    pitch: f32,
    direction: f32,
    speed: f32,
}

#[derive(Component)]
struct Controlable;

#[derive(Component)]
struct Position(Vec3);

const BASE_SPEED: f32 = 3.0;

fn spawn_plane(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 5.9)),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 0.0),
                rotation: Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Aeroplane)
        .insert(Controlable)
        .insert(Position(Vec3 { x: 0.0, y: 0.0, z: 0.0 }))
        .insert(Heading { pitch: 0.0, direction: 1.0, speed: BASE_SPEED});
}

const PITCH_DELTA: f32 = 0.05;

fn controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut movement: Query<(&mut Heading, &Controlable)>,
) {
    
    for (mut heading, _) in movement.iter_mut() {

        if keyboard_input.pressed(KeyCode::Up) {
            heading.pitch += PITCH_DELTA;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            heading.pitch -= PITCH_DELTA;
        }
        
        if keyboard_input.pressed(KeyCode::Left) {
            if heading.direction > 0.0 {
                heading.direction *= -1.0;
                heading.speed = BASE_SPEED;
            }
        }
        
        if keyboard_input.pressed(KeyCode::Right) {
            if heading.direction < 0.0 {
                heading.direction *= -1.0;
                heading.speed = BASE_SPEED;
            }
        }
        
        if heading.pitch > 0.0 {
            heading.pitch = heading.pitch.min(1.0);
        } else {
            heading.pitch = heading.pitch.max(-1.0);
        }
    }
}

fn set_position_from_heading(
    mut movement: Query<(&mut Position, &Heading)>
) {
    for (mut position, heading) in movement.iter_mut() {
        position.0.x += (heading.pitch * PI / 2.0).cos() * heading.speed * heading.direction;
        position.0.y += (heading.pitch * PI / 2.0).sin() * heading.speed;
    }
}

fn set_translation_from_position(
    mut movement: Query<(&mut Transform, &Position)>
) {
    for (mut transform, position) in movement.iter_mut() {
        transform.translation = position.0
    }
}

fn set_rotation_from_heading(
    mut movement: Query<(&mut Transform, &Heading)>
) {
    for (mut transform, heading) in movement.iter_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), heading.pitch * PI / 2.0 * heading.direction)
    }
}

fn set_flip_from_direction(
    mut movement: Query<(&mut Transform, &Heading)>
) {
    for (mut transform, heading) in movement.iter_mut() {
        if heading.direction < 0.0 && transform.scale.x > 0.0 {
            transform.scale.x *= -1.0
        } else if heading.direction > 0.0 && transform.scale.x < 0.0 {
            transform.scale.x *= -1.0
        }
    }
}

const GRAVITY_FACTOR: f32 = 0.1;
const TERMINAL_VELOCITY_FACTOR: f32 = 2.0;
const ACCELERATION: f32 = 0.005;

fn physics(
    mut movement: Query<&mut Heading>
) {
    for mut heading in movement.iter_mut() {
        heading.speed = (BASE_SPEED * (1.0 + heading.pitch.powf(3.0) * -1.0)).min(heading.speed + ACCELERATION);
    }
}