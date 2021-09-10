use bevy::prelude::*;

use crate::GROUND;

pub fn render(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const THICKNESS: f32 = 1000.;
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.25, 0.2, 0.2).into()),
        transform: Transform::from_xyz(0., GROUND - (THICKNESS / 2.), 0.),
        sprite: Sprite::new(Vec2::new(10_000.0, THICKNESS)),
        ..Default::default()
    });
}
