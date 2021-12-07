use bevy::prelude::*;
use bevy_rapier2d::na::Rotation2;
use bevy_rapier2d::prelude::*;
use std::f32::consts::TAU;

use crate::{MainCamera, METERS_TO_PIXELS};

// Constants for physics (Units are meters and seconds)
pub const PLAYER_HEIGHT: f32 = 1.5;
pub const PLAYER_WIDTH: f32 = 1.5;

pub const BOUNDARY: f32 = 100.;

pub struct Player;

pub struct Physics {
    // Basis has the columns [right, up]
    pub basis: Matrix<f32>,

    pub velocity: f32,
    pub jump_distance: f32,
    pub jump_height: f32,
    pub heavy_scalar: f32,
    pub jump_velocity: f32,
    pub jump_gravity: f32,
    pub gravity: f32,
}

impl Physics {
    pub fn new(
        velocity: f32,
        jump_distance: f32,
        jump_height: f32,
        heavy_scalar: f32,
    ) -> Self {
        return Physics {
            // Definition
            velocity,
            jump_distance,
            jump_height,
            heavy_scalar,

            // Derived
            basis: Matrix::new(1., 0., 0., 1.),
            jump_velocity: (2. * jump_height * velocity) / (jump_distance / 2.),
            jump_gravity: (-2. * jump_height * velocity * velocity)
                / ((jump_distance / 2.) * (jump_distance / 2.)),
            gravity: (-2. * jump_height * velocity * velocity)
                / ((jump_distance / 2.) * (jump_distance / 2.))
                * heavy_scalar,
        };
    }

    pub fn rotate(mut self: &mut Self, radians: f32) {
        let rotation = Rotation2::new(radians);
        self.basis = self.basis * rotation;
    }

    pub fn get_gravity(&self) -> Vector<f32> {
        let scale: Vector<f32> = [0., self.gravity].into();
        self.basis * scale
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let physics = Physics::new(50., 20., 20., 2.);
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
        shape: ColliderShape::ball(PLAYER_HEIGHT / 2.),
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
        transform: Transform::from_xyz(0., 0., 5.),
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

// pub fn limit_velocity(mut query: Query<&mut RigidBodyVelocity, With<Player>>) {
//     for mut velocity in query.iter_mut() {
//         if velocity.linvel.magnitude() > MAX_VELOCITY {
//             velocity.linvel = velocity.linvel.normalize() * MAX_VELOCITY;
//         }
//     }
// }

pub fn player_jump(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Physics, &mut RigidBodyVelocity), With<Player>>,
) {
    for (mut physics, mut velocity) in query.iter_mut() {
        if kb.just_pressed(KeyCode::Space) {
            let decomposition = physics.basis.lu();
            let x = decomposition.solve(&velocity.linvel).expect("Ooops!");
            if x.y < physics.jump_velocity {
                let movement: Vector<f32> = [x.x, physics.jump_velocity].into();
                physics.gravity = physics.jump_gravity;
                velocity.linvel = physics.basis * movement;
            }
        }

        if kb.just_released(KeyCode::Space) {
            physics.gravity = physics.jump_gravity * physics.heavy_scalar;
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
            if x.x > -1. * physics.velocity {
                let movement: Vector<f32> =
                    [-1. * physics.velocity, x.y].into();
                velocity.linvel = physics.basis * movement;
            }
        }

        if kb.pressed(KeyCode::D) {
            if x.x < physics.velocity {
                let movement: Vector<f32> = [physics.velocity, x.y].into();
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

pub fn follow_player(
    mut query: QuerySet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<MainCamera>>,
    )>,
) {
    let mut translation: Vec3 = Vec3::ZERO;

    if let Ok(transform) = query.q0().single() {
        translation = transform.translation.to_owned();
    }

    if let Ok(mut transform) = query.q1_mut().single_mut() {
        translation.z = transform.translation.z.to_owned();
        transform.translation = translation;
    }
}
