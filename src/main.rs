use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_ldtk::prelude::*;

mod physics;
mod player;
mod systems;

fn main() {
    App::new()
        .insert_resource(LevelSelection::index(0))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, systems::setup)
        .add_systems(
            Update,
            (
                systems::spawn_tile_collisions,
                systems::spawn_player,
                player::movement,
                (player::collision_system, physics::update_rect).chain(),
            ),
        )
        .register_ldtk_int_cell::<systems::TileBundle>(1)
        .run();
}
