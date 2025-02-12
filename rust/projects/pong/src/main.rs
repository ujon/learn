mod bundles;
mod components;
mod constants;
mod systems;

use crate::bundles::{move_play1_paddle, move_play2_paddle};
use crate::components::{Score, Scored};
use bevy::prelude::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(create_window()))
        .init_resource::<Score>()
        .add_event::<Scored>()
        .add_systems(
            Startup,
            (
                spawn_dotted_line,
                spawn_ball,
                spawn_paddle,
                spawn_camera,
                spawn_scoreboard,
                spawn_boundary,
            ),
        )
        .add_systems(
            Update,
            (
                move_ball,
                move_play1_paddle,
                detect_scoring,
                move_play2_paddle,
                respawn_ball.after(detect_scoring),
                update_score.after(detect_scoring),
                update_scoreboard.after(detect_scoring),
                update_entity_position.after(move_ball),
                move_paddles.after(move_play1_paddle),
                handle_collision.after(move_ball),
            ),
        )
        .run();
}
