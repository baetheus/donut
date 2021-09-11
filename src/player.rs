use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::METERS_TO_PIXELS;

pub const PLAYER_HEIGHT: f32 = 2.;
pub const PLAYER_WIDTH: f32 = 1.;

pub const MAX_VELOCITY: f32 = 30.; // v_x
pub const MAX_VELOCITY_SQUARED: f32 = MAX_VELOCITY * MAX_VELOCITY;

pub const JUMPS: u8 = 3;

pub const JUMP_DISTANCE: f32 = 20. * PLAYER_WIDTH; // 2 * x_h
pub const JUMP_DISTANCE_HALF: f32 = JUMP_DISTANCE / 2.; // x_h
pub const JUMP_DISTANCE_HALF_SQUARED: f32 =
    JUMP_DISTANCE_HALF * JUMP_DISTANCE_HALF;
pub const JUMP_HEIGHT: f32 = 4. * PLAYER_HEIGHT; // h

pub const JUMP_GRAVITY: f32 =
    (-2. * JUMP_HEIGHT * MAX_VELOCITY_SQUARED) / JUMP_DISTANCE_HALF_SQUARED;
pub const GRAVITY: f32 = 5. * JUMP_GRAVITY;

pub const PLAYER_JUMP_VELOCITY: f32 =
    (2. * JUMP_HEIGHT * MAX_VELOCITY) / JUMP_DISTANCE_HALF;

pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBundle {
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        forces: RigidBodyForces {
            gravity_scale: GRAVITY,
            ..Default::default()
        },
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.),
        material: ColliderMaterial {
            friction: 0.,
            friction_combine_rule: CoefficientCombineRule::Min.into(),
            restitution: 0.,
            restitution_combine_rule: CoefficientCombineRule::Min.into(),
            ..Default::default()
        },
        ..Default::default()
    };
    let sprite = SpriteBundle {
        material: materials.add(Color::rgb(0.4, 0.4, 0.5).into()),
        sprite: Sprite::new(Vec2::new(
            PLAYER_WIDTH * METERS_TO_PIXELS,
            PLAYER_HEIGHT * METERS_TO_PIXELS,
        )),
        ..Default::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        .insert_bundle(sprite)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player);
}

pub fn player_jump(
    kb: Res<Input<KeyCode>>,
    mut query: Query<
        (&mut RigidBodyVelocity, &mut RigidBodyForces),
        With<Player>,
    >,
) {
    for (mut velocity, mut forces) in query.iter_mut() {
        if kb.just_pressed(KeyCode::Space) {
            velocity.linvel.y = PLAYER_JUMP_VELOCITY;
            forces.gravity_scale = JUMP_GRAVITY;
        }

        if kb.just_released(KeyCode::Space) {
            forces.gravity_scale = GRAVITY;
        }
    }
}

pub fn player_move(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut RigidBodyVelocity, With<Player>>,
) {
    let mut direction = 0.;
    if kb.pressed(KeyCode::A) {
        direction += -1.;
    }
    if kb.pressed(KeyCode::D) {
        direction += 1.;
    }
    for mut velocity in query.iter_mut() {
        velocity.linvel.x = direction * MAX_VELOCITY;
    }
}
