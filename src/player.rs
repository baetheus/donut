use bevy::prelude::*;
use bevy_rapier2d::na::Rotation2;
use bevy_rapier2d::prelude::*;
use std::f32::consts::TAU;

use crate::METERS_TO_PIXELS;

// Constants for physics (Units are meters and seconds)
pub const PLAYER_HEIGHT: f32 = 1.5;
pub const PLAYER_WIDTH: f32 = 1.;
pub const VELOCITY: f32 = 20.; // v_x
pub const MAX_VELOCITY: f32 = 3. * VELOCITY;
pub const JUMP_DISTANCE: f32 = 7.; // 2 * x_h
pub const JUMP_HEIGHT: f32 = 5.; // h

// Derived constants for physics
pub const VELOCITY_SQUARED: f32 = VELOCITY * VELOCITY;
pub const JUMP_DISTANCE_HALF: f32 = JUMP_DISTANCE / 2.; // x_h
pub const JUMP_DISTANCE_HALF_SQUARED: f32 =
    JUMP_DISTANCE_HALF * JUMP_DISTANCE_HALF;
pub const JUMP_GRAVITY: f32 =
    (-2. * JUMP_HEIGHT * VELOCITY_SQUARED) / JUMP_DISTANCE_HALF_SQUARED;
pub const GRAVITY: f32 = 3. * JUMP_GRAVITY;
pub const JUMP_VELOCITY: f32 =
    (2. * JUMP_HEIGHT * VELOCITY) / JUMP_DISTANCE_HALF;

pub const BOUNDARY: f32 = 100.;

pub struct Player;

pub struct Physics {
    // Basis has the columns [right, up]
    pub basis: Matrix<f32>,
    pub gravity_scalar: f32,
}

impl Physics {
    pub fn new() -> Self {
        return Physics {
            basis: Matrix::new(1., 0., 0., 1.),
            gravity_scalar: GRAVITY,
        };
    }

    pub fn rotate(mut self: &mut Self, radians: f32) {
        let rotation = Rotation2::new(radians);
        self.basis = self.basis * rotation;
    }

    pub fn get_gravity(&self) -> Vector<f32> {
        let scale: Vector<f32> = [0., self.gravity_scalar].into();
        self.basis * scale
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
            force: physics.get_gravity(),
            ..Default::default()
        },
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(PLAYER_WIDTH / 2., PLAYER_HEIGHT / 2.),
        material: ColliderMaterial {
            friction: 0.3,
            friction_combine_rule: CoefficientCombineRule::Min.into(),
            restitution: 0.3,
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
        forces.force = physics.get_gravity();
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
            let decomposition = physics.basis.lu();
            let x = decomposition.solve(&velocity.linvel).expect("Ooops!");
            if x.y < JUMP_VELOCITY {
                let movement: Vector<f32> = [x.x, JUMP_VELOCITY].into();
                physics.gravity_scalar = JUMP_GRAVITY;
                velocity.linvel = physics.basis * movement;
            }
        }

        if kb.just_released(KeyCode::Space) {
            physics.gravity_scalar = GRAVITY;
        }
    }
}

pub fn player_move(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&Physics, &mut RigidBodyVelocity), With<Player>>,
) {
    for (physics, mut velocity) in query.iter_mut() {
        let decomposition = physics.basis.lu();
        let x = decomposition.solve(&velocity.linvel).expect("Ooops!");
        if kb.pressed(KeyCode::A) {
            if x.x > -1. * MAX_VELOCITY {
                let movement: Vector<f32> = [-1. * VELOCITY, x.y].into();
                velocity.linvel = physics.basis * movement;
            }
        }

        if kb.pressed(KeyCode::D) {
            if x.x < MAX_VELOCITY {
                let movement: Vector<f32> = [VELOCITY, x.y].into();
                velocity.linvel = physics.basis * movement;
            }
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
