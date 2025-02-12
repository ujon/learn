use crate::components::*;
use crate::constants::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub shape: Shape,
    pub velocity: Velocity,
    pub position: Position,
}

impl BallBundle {
    pub fn new(x: f32, y: f32) -> BallBundle {
        BallBundle {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0.0, 0.0)),
        }
    }
}

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub shape: Shape,
    pub velocity: Velocity,
    pub position: Position,
}

impl PaddleBundle {
    pub fn new(x: f32, y: f32) -> PaddleBundle {
        PaddleBundle {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            velocity: Velocity(Vec2::new(0.0, 0.0)),
            position: Position(Vec2::new(x, y)),
        }
    }
}

#[derive(Bundle)]
pub struct BoundaryBundle {
    pub boundary: Boundary,
    pub shape: Shape,
    pub position: Position,
}

impl BoundaryBundle {
    pub fn new(x: f32, y: f32, width: f32) -> BoundaryBundle {
        BoundaryBundle {
            boundary: Boundary,
            shape: Shape(Vec2::new(width, BOUNDARY_HEIGHT)),
            position: Position(Vec2::new(x, y)),
        }
    }
}

pub fn move_play1_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player1>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            velocity.0.y = 1.0;
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            velocity.0.y = -1.0;
        } else {
            velocity.0.y = 0.0;
        }
    }
}

pub fn move_play2_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Query<&mut Velocity, With<Player2>>,
) {
    if let Ok(mut velocity) = paddle.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.0.y = 1.0;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.0.y = -1.0;
        } else {
            velocity.0.y = 0.0;
        }
    }
}