use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{METERS_TO_PIXELS, WINDOW_WIDTH};

pub const GROUND: f32 = -28.;

const GROUND_HEIGHT: f32 = 2.;
const GROUND_WIDTH: f32 = WINDOW_WIDTH;

pub fn spawn_ground(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /* Create the ground. */
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(GROUND_WIDTH / 2., GROUND_HEIGHT / 2.),
            position: [0., GROUND + GROUND_HEIGHT].into(),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.5).into()),
            sprite: Sprite::new(Vec2::new(
                GROUND_WIDTH * METERS_TO_PIXELS,
                GROUND_HEIGHT * METERS_TO_PIXELS,
            )),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    /* Create the ceiling. */
    commands
        .spawn_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(GROUND_WIDTH / 2., GROUND_HEIGHT / 2.),
            position: [0., -1. * (GROUND + GROUND_HEIGHT)].into(),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.2, 0.2, 0.5).into()),
            sprite: Sprite::new(Vec2::new(
                GROUND_WIDTH * METERS_TO_PIXELS,
                GROUND_HEIGHT * METERS_TO_PIXELS,
            )),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
