use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                jump_system,
                shoot,
                flying_bullets,
                flying_objects_despawn,
                rotate_jumping_object,
            ),
        )
        .run();
}

#[derive(Component)]
struct Enemy {
    hp: u8,

}

#[derive(Component)]
struct JumpingThing {
    jump_init: f32,
    is_jump: bool,
}

#[derive(Component)]
struct FlyingObject {
    velocity: Vec3,
}

#[derive(Component)]
struct MyGroundPlane;

#[derive(Component)]
struct MyGameCamera;

const JUMP_HEIGHT: f32 = 1.;
const JUMP_SPEED: f32 = 10.;
const INITIAL_VEC: Vec3 = Vec3::new(0., 0.5, 0.);
const CAMERA_INITIAL: Vec3 = Vec3::new(-1.5, 4.5, 7.0);
const ROTATION_SPEED: f32 = 0.9;
const BULLET_SPEED: f32 = 5.;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Circle::new(16.0)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
        MyGroundPlane,
    ));
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(CAMERA_INITIAL).looking_at(INITIAL_VEC, Vec3::Y),
            ..default()
        },
        MyGameCamera,
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_translation(INITIAL_VEC),
            ..default()
        },
        JumpingThing {
            jump_init: 0.,
            is_jump: false,
        },
    ));
}

fn jump_system(
    time: Res<Time>,
    mut sprite_position: Query<(&mut JumpingThing, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mut jumping, mut transform) in &mut sprite_position {
        if keyboard.just_pressed(KeyCode::Space) & !jumping.is_jump {
            jumping.is_jump = true;
            jumping.jump_init = time.elapsed_seconds();
        }

        if jumping.is_jump {
            let time_dif = time.elapsed_seconds() - jumping.jump_init;

            // Check if the time deference is more than a period of sinus - end jump
            if time_dif <= (PI / JUMP_SPEED) {
                let temp_sin = (time_dif * JUMP_SPEED).sin();
                transform.translation.y = INITIAL_VEC.y + JUMP_HEIGHT * temp_sin;
            } else {
                transform.translation.y = INITIAL_VEC.y;
                jumping.is_jump = false;
            }
        }
    }
}

fn rotate_jumping_object(
    time: Res<Time>,
    mut sprite_position: Query<(&mut JumpingThing, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::ArrowRight) {
        for (_, mut transform) in &mut sprite_position {
            transform.rotate_y(-ROTATION_SPEED * TAU * time.delta_seconds());
        }
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        let angle = ROTATION_SPEED * TAU * time.delta_seconds();
        for (_, mut transform) in &mut sprite_position {
            transform.rotate_y(angle);
        }
    }
}

fn shoot(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sprite_position: Query<(&mut JumpingThing, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        for (_, transform) in &mut sprite_position {
            let mut trans = transform.clone();
            trans.rotate_axis(transform.right().normalize(), 0.25 * TAU);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Capsule3d::new(0.05, 0.3)),
                    material: materials.add(Color::WHITE),
                    transform: trans,
                    ..default()
                },
                FlyingObject {
                    velocity: transform.forward().normalize() * BULLET_SPEED,
                },
            ));
        }
    }
}

fn flying_objects_despawn(
    flying_obj: Query<(Entity, &FlyingObject, &Transform)>,
    mut commands: Commands,
) {
    for (entity, _, transform) in flying_obj.iter() {
        if transform.translation.x.abs() > 16.
            || transform.translation.z.abs() > 16.
        {
            commands.entity(entity).despawn();
        }
    }
}

fn flying_bullets(time: Res<Time>, mut flying_obj: Query<(&FlyingObject, &mut Transform)>) {
    for (flying, mut transform) in &mut flying_obj {
        transform.translation += flying.velocity * time.delta_seconds();
    }
}
