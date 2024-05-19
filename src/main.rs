use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod physics;
mod player;
mod systems;

fn main() {
    App::new()
        .insert_resource(LevelSelection::index(0))
        .insert_resource(Time::<Fixed>::from_hz(120.0))
        .register_ldtk_int_cell::<systems::TileBundle>(1)
        .register_type::<player::Player>()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::new(),
            LdtkPlugin,
            FrameTimeDiagnosticsPlugin,
            physics::PhysicsPlugin,
            physics::PhysicsDebugPlugin,
        ))
        .add_systems(Startup, systems::setup)
        .add_systems(
            Update,
            (
                systems::spawn_tile_collisions,
                systems::spawn_player,
                player::handle_input,
                player::handle_collision,
            ),
        )
        .run();
}
