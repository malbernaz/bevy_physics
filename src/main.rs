use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_editor_pls::prelude::*;

mod physics;
mod player;
mod systems;

const BACKGROUND_COLOR: Color = Color::rgb(1., 1., 1.);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(EditorPlugin::default())
        .add_systems(
            Startup,
            (
                systems::setup_camera,
                systems::spawn_tiles,
                systems::spawn_player,
            ),
        )
        .add_systems(
            Update,
            (
                player::movement,
                player::collision_system,
                physics::collider::update_rect,
            )
                .chain(),
        )
        // .add_systems(FixedUpdate, player::collision_system)
        .run();
}
