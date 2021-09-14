use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::TAU;

use crate::METERS_TO_PIXELS;

pub const PLAYER_HEIGHT: f32 = 1.;
pub const PLAYER_WIDTH: f32 = 1.;
pub const VELOCITY: f32 = 30.; // v_x
pub const MAX_VELOCITY: f32 = 3. * VELOCITY;
pub const JUMP_DISTANCE: f32 = 20. * PLAYER_WIDTH; // 2 * x_h
pub const JUMP_HEIGHT: f32 = 4. * PLAYER_HEIGHT; // h

pub const VELOCITY_SQUARED: f32 = VELOCITY * VELOCITY;
pub const JUMP_DISTANCE_HALF: f32 = JUMP_DISTANCE / 2.; // x_h
pub const JUMP_DISTANCE_HALF_SQUARED: f32 =
    JUMP_DISTANCE_HALF * JUMP_DISTANCE_HALF;
pub const JUMP_GRAVITY: f32 =
    (-2. * JUMP_HEIGHT * VELOCITY_SQUARED) / JUMP_DISTANCE_HALF_SQUARED;
pub const GRAVITY: f32 = 5. * JUMP_GRAVITY;
pub const PLAYER_JUMP_VELOCITY: f32 =
    (2. * JUMP_HEIGHT * VELOCITY) / JUMP_DISTANCE_HALF;

pub const BOUNDARY: f32 = 100.;

pub struct Player;

pub struct Physics {
    pub down: Vector<f32>,
    pub left: Vector<f32>,
    pub right: Vector<f32>,
    pub gravity: f32,
}

impl Physics {
    pub fn rotate(mut self: &mut Self, radians: f32) {
        let iso = Isometry::rotation(radians);
        self.down = iso * self.down;
        self.left = iso * self.left;
        self.right = iso * self.right;
    }

    pub fn new() -> Self {
        return Physics {
            down: [0., 1.].into(),
            left: [-1., 0.].into(),
            right: [1., 0.].into(),
            gravity: GRAVITY,
        };
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let physics = Physics::new();
    let rigid_body = RigidBodyBundle {
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        forces: RigidBodyForces {
            gravity_scale: 0.,
            force: physics.gravity * physics.down,
            ..Default::default()
        },
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.),
        material: ColliderMaterial {
            friction: 0.1,
            friction_combine_rule: CoefficientCombineRule::Min.into(),
            restitution: 0.1,
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
        .insert(physics)
        .insert(Player);
}

pub fn link_physics(
    mut query: Query<(&mut RigidBodyForces, &mut Physics), With<Player>>,
) {
    for (mut forces, physics) in query.iter_mut() {
        forces.force = physics.gravity * physics.down;
    }
}

pub fn limit_velocity(mut query: Query<&mut RigidBodyVelocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        if velocity.linvel.magnitude() > MAX_VELOCITY {
            velocity.linvel = velocity.linvel.normalize() * MAX_VELOCITY;
        }
    }
}

pub fn player_jump(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Physics, &mut RigidBodyVelocity), With<Player>>,
) {
    for (mut physics, mut velocity) in query.iter_mut() {
        if kb.just_pressed(KeyCode::Space) {
            physics.gravity = JUMP_GRAVITY;
            velocity.linvel += PLAYER_JUMP_VELOCITY * physics.down;
        }

        if kb.just_released(KeyCode::Space) {
            physics.gravity = GRAVITY;
        }
    }
}

pub fn player_move(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&Physics, &mut RigidBodyVelocity), With<Player>>,
) {
    for (physics, mut velocity) in query.iter_mut() {
        if kb.just_pressed(KeyCode::A) {
            velocity.linvel += physics.left * VELOCITY;
        }
        if kb.just_released(KeyCode::A) {
            velocity.linvel -= physics.left * VELOCITY;
        }
        if kb.just_pressed(KeyCode::D) {
            velocity.linvel += physics.right * VELOCITY;
        }
        if kb.just_released(KeyCode::D) {
            velocity.linvel -= physics.right * VELOCITY;
        }
    }
}

pub fn respawn(mut query: Query<&mut RigidBodyPosition, With<Player>>) {
    for mut position in query.iter_mut() {
        let magnitude = position.position.translation.vector.magnitude();

        if magnitude > BOUNDARY {
            position.position = [0., 0.].into();
        }
    }
}

pub fn rotate(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Physics, With<Player>>,
) {
    for mut physics in query.iter_mut() {
        if kb.just_pressed(KeyCode::Q) {
            physics.rotate(TAU / -4.);
        }

        if kb.just_pressed(KeyCode::E) {
            physics.rotate(TAU / 4.);
        }
    }
}
