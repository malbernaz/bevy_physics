use bevy::{prelude::*, render::camera::ScalingMode::FixedHorizontal};
use bevy_ecs_ldtk::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::physics::*;
use crate::player::PlayerBundle;

#[derive(Component)]
pub struct CameraMarker;

pub const WIDTH: f32 = 320.;
pub const HEIGHT: f32 = 190.;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                scaling_mode: FixedHorizontal(WIDTH),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(WIDTH / 2., HEIGHT / 2., 0.)),
            ..default()
        },
        CameraMarker,
    ));

    let ldtk_handle = asset_server.load("level.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: ldtk_handle.clone(),
        ..default()
    });
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Tile;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct TileBundle {
    tile: Tile,
}

pub fn spawn_tile_collisions(
    mut commands: Commands,
    tile_query: Query<(&GridCoords, &Parent), Added<Tile>>,
    parent_query: Query<&Parent, Without<Tile>>,
    level_query: Query<(Entity, &LevelIid)>,
    asset_server: Res<AssetServer>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    if tile_query.is_empty() {
        return;
    }

    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn tile collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the tiles to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_tile_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    for (&grid_coords, parent) in &tile_query {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_tile_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    }

    let ldtk_project = ldtk_project_assets
        .get(asset_server.load("level.ldtk"))
        .expect("Project should be loaded if level has spawned");

    for (level_entity, level_iid) in &level_query {
        let Some(level_tiles) = level_to_tile_locations.get(&level_entity) else {
            return;
        };

        let level = ldtk_project
            .as_standalone()
            .get_loaded_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        let LayerInstance {
            c_wid: width,
            c_hei: height,
            grid_size,
            ..
        } = level.layer_instances()[0];

        // combine tiles into flat "plates" in each individual row
        let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

        for y in 0..height {
            let mut row_plates: Vec<Plate> = Vec::new();
            let mut plate_start = None;

            // + 1 to the width so the algorithm "terminates" plates that touch the right edge
            for x in 0..width + 1 {
                match (plate_start, level_tiles.contains(&GridCoords { x, y })) {
                    (Some(s), false) => {
                        row_plates.push(Plate {
                            left: s,
                            right: x - 1,
                        });
                        plate_start = None;
                    }
                    (None, true) => plate_start = Some(x),
                    _ => (),
                }
            }

            plate_stack.push(row_plates);
        }

        // combine "plates" into rectangles across multiple rows
        let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
        let mut prev_row: Vec<Plate> = Vec::new();
        let mut tile_rects: Vec<Rect> = Vec::new();

        // an extra empty row so the algorithm "finishes" the rects that touch the top edge
        plate_stack.push(Vec::new());

        for (y, current_row) in plate_stack.into_iter().enumerate() {
            for prev_plate in &prev_row {
                if !current_row.contains(prev_plate) {
                    // remove the finished rect so that the same plate in the future starts a new rect
                    if let Some(rect) = rect_builder.remove(prev_plate) {
                        tile_rects.push(rect);
                    }
                }
            }

            for plate in &current_row {
                rect_builder
                    .entry(plate.clone())
                    .and_modify(|e| e.top += 1)
                    .or_insert(Rect {
                        bottom: y as i32,
                        top: y as i32,
                        left: plate.left,
                        right: plate.right,
                    });
            }

            prev_row = current_row;
        }

        commands.entity(level_entity).with_children(|level| {
            // Spawn colliders for every rectangle..
            // Making the collider a child of the level serves two purposes:
            // 1. Adjusts the transforms to be relative to the level for free
            // 2. the colliders will be despawned automatically when levels unload
            for tile_rect in tile_rects {
                let transform = TransformBundle::from_transform(Transform::from_xyz(
                    (tile_rect.left + tile_rect.right + 1) as f32 * grid_size as f32 / 2.,
                    (tile_rect.bottom + tile_rect.top + 1) as f32 * grid_size as f32 / 2.,
                    0.,
                ));
                let center = transform.local.translation.xy();

                level.spawn((
                    Name::new("TileRect"),
                    transform,
                    SolidBundle::new(
                        center,
                        Vec2::new(
                            (tile_rect.right - tile_rect.left + 1) as f32 * grid_size as f32 / 2.,
                            (tile_rect.top - tile_rect.bottom + 1) as f32 * grid_size as f32 / 2.,
                        ),
                    ),
                ));
            }
        });
    }
}

pub fn spawn_player(
    mut commands: Commands,
    level_query: Query<Entity, Added<LevelIid>>,
    asset_server: Res<AssetServer>,
) {
    let transform = Transform::from_xyz(20., 64., 0.);
    for level_entity in &level_query {
        commands.entity(level_entity).with_children(|builder| {
            builder.spawn(PlayerBundle::new(
                transform,
                asset_server.load("player.png"),
            ));
        });
    }
}
