use bevy::prelude::*;

use crate::GROUND;

pub const PLAYER_HEIGHT: f32 = 40.0;
pub const PLAYER_WIDTH: f32 = 30.0;
pub const PLAYER_BOTTOM: f32 = GROUND + (PLAYER_HEIGHT / 2.);

pub const MAX_VELOCITY: f32 = 10.; // v_x
pub const ACCELERATION: f32 = 0.4;

pub const JUMPS: u8 = 3;
pub const JUMP_DISTANCE: f32 = PLAYER_WIDTH * 10.; // 2 * x_h
pub const JUMP_DISTANCE_HALF: f32 = JUMP_DISTANCE / 2.; // x_h
pub const JUMP_HEIGHT: f32 = PLAYER_HEIGHT * 4.; // h

pub fn player_gravity() -> f32 {
    (-2. * JUMP_HEIGHT * MAX_VELOCITY.powi(2)) / JUMP_DISTANCE_HALF.powi(2)
}
pub const PLAYER_JUMP_VELOCITY: f32 =
    (2. * JUMP_HEIGHT * MAX_VELOCITY) / JUMP_DISTANCE_HALF;

#[derive(Debug)]
pub struct Physics {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub jumps: u8,
}

pub struct Player;

impl Default for Physics {
    fn default() -> Physics {
        return Physics {
            position: [0., PLAYER_BOTTOM].into(),
            velocity: [0., 0.].into(),
            acceleration: [0., 0.].into(),
            jumps: JUMPS,
        };
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.4, 0.4, 0.5).into()),
            transform: Transform::from_xyz(0., PLAYER_BOTTOM, 0.),
            sprite: Sprite::new(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..Default::default()
        })
        .insert(Physics {
            ..Default::default()
        })
        .insert(Player);
}

pub fn player_jump(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Physics, With<Player>>,
) {
    if kb.just_pressed(KeyCode::Space) {
        for mut physics in query.iter_mut() {
            if physics.jumps > 0 {
                physics.jumps -= 1;
                physics.velocity.y = PLAYER_JUMP_VELOCITY;
                physics.acceleration.y = player_gravity();
                physics.position.y += 1.;
            }
        }
    }
}

pub fn player_move(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Physics, With<Player>>,
) {
    let mut direction = 0.;

    if kb.pressed(KeyCode::A) {
        direction += -1.;
    }

    if kb.pressed(KeyCode::D) {
        direction += 1.;
    }

    for mut physics in query.iter_mut() {
        physics.acceleration.x = direction * ACCELERATION;
        if direction == 0. {
            physics.velocity.x = 0.;
        }
    }
}

pub fn physics(time: Res<Time>, mut query: Query<&mut Physics, With<Player>>) {
    for mut physics in query.iter_mut() {
        let delta = time.delta().as_secs_f32() * 100.;
        let delta_squared = delta.powi(2);

        // Moving
        physics.position.x += physics.velocity.x * delta
            + 0.5 * physics.acceleration.x * delta_squared;
        physics.velocity.x += physics.acceleration.x * delta;

        // Set speed limit
        if physics.velocity.x > 0. && physics.velocity.x > MAX_VELOCITY {
            physics.velocity.x = MAX_VELOCITY;
        }

        if physics.velocity.x < 0. && physics.velocity.x < (-1. * MAX_VELOCITY)
        {
            physics.velocity.x = -1. * MAX_VELOCITY;
        }

        // Jumping
        if physics.position.y <= PLAYER_BOTTOM {
            physics.position.y = PLAYER_BOTTOM;
            physics.velocity.y = 0.;
            physics.acceleration.y = 0.;
            physics.jumps = JUMPS;
        } else {
            physics.position.y += physics.velocity.y * delta
                + 0.5 * physics.acceleration.y * delta_squared;
            physics.velocity.y += physics.acceleration.y * delta;

            // Double check that we don't go too low..
            if physics.position.y <= PLAYER_BOTTOM {
                physics.position.y = PLAYER_BOTTOM;
            }
        }
    }
}

pub fn link_sprite(
    mut query: Query<(&mut Physics, &mut Transform), With<Player>>,
) {
    for (physics, mut transform) in query.iter_mut() {
        transform.translation =
            [physics.position.x, physics.position.y, 0.].into();
    }
}
