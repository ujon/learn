use crate::constants::WINDOW_HEIGHT;
use bevy::prelude::*;
use std::slice::Windows;

pub fn create_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: String::from("Ping Pong"),
            ..default()
        }),
        ..default()
    }
}

pub fn spawn_dotted_line(mut commands: Commands) {
    let dot_color = Color::srgb(1.0, 1.0, 1.0);
    let dot_size = Vec3::new(5.0, 20.0, 1.0);
    let gap_size = 10.0;
    let num_dots = (WINDOW_HEIGHT / (dot_size.y + gap_size)) as i32;

    for i in 0..num_dots {
        commands.spawn((
            Sprite {
                color: dot_color,
                ..Default::default()
            },
            Transform {
                translation: Vec3::new(
                    0.0,
                    i as f32 * (dot_size.y + gap_size) - WINDOW_HEIGHT / 2.0,
                    0.0,
                ),
                scale: dot_size,
                ..Default::default()
            },
        ));
    }
}
