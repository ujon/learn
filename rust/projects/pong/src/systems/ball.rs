use crate::bundles::BallBundle;
use crate::components::*;
use crate::constants::*;
use bevy::ecs::query::QuerySingleError;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::WindowResized;
use rand::Rng;
use std::slice::Windows;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Mesh::from(Circle::new(BALL_SIZE));
    let color = ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0));

    let mesh_handle = meshes.add(shape);
    let material_handle = materials.add(color);
    commands.spawn((
        BallBundle::new(1.0, 0.0),
        Mesh2d {
            0: mesh_handle.into(),
        },
        MeshMaterial2d { 0: material_handle },
    ));
}

pub fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

pub fn update_entity_position(mut ball: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in ball.iter_mut() {
        transform.translation = position.0.extend(0.0);
    }
}

pub fn detect_scoring(
    mut ball_query: Query<&mut Position, With<Ball>>,
    window_query: Query<&Window>,
    mut score_event_write: EventWriter<Scored>,
) {
    let window = match window_query.get_single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let half_window_width = window.resolution.width() / 2.0;
    let ball_position = match ball_query.get_single_mut() {
        Ok(bp) => bp,
        Err(_) => return,
    };

    if ball_position.0.x > half_window_width {
        score_event_write.send(Scored(Scorer::Player2));
    } else if ball_position.0.x < -half_window_width {
        score_event_write.send(Scored(Scorer::Player1));
    }
}

pub fn respawn_ball(
    mut ball_query: Query<(&mut Position, &mut Velocity), With<Ball>>,
    mut events: EventReader<Scored>,
) {
    if let Ok((mut position, mut velocity)) = ball_query.get_single_mut() {
        for event in events.read() {
            position.0 = Vec2::new(0.0, 0.0);

            let mut rng = rand::rng();
            let angle: f32 = match event.0 {
                Scorer::Player1 => {
                    rng.random_range(-std::f32::consts::PI / 4.0..std::f32::consts::PI / 4.0)
                }
                Scorer::Player2 => rng.random_range(
                    3.0 * std::f32::consts::PI / 4.0..5.0 * std::f32::consts::PI / 4.0,
                ),
            };

            let speed: f32 = 1.0;
            velocity.0 = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        }
    }
}
