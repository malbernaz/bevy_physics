use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::physics::collider::*;
use crate::physics::solid::SolidBundle;
use crate::player::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.5,
            ..default()
        },
        ..default()
    });
}

pub fn spawn_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let map_size = TilemapSize { x: 40, y: 1 };
    let tile_size = TilemapTileSize { x: 8.0, y: 8.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();
    let tilemap_entity = commands.spawn(Name::new("TileMap")).id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };

            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        ..default()
                    },
                    Name::new("Tile"),
                ))
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let mut map_transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);
    map_transform.translation.y = -176.;
    let collider = Collider::new(
        Vec2::new(0., -176.),
        Vec2::new(
            map_size.x as f32 * tile_size.x / 2.,
            map_size.y as f32 * tile_size.y / 2.,
        ),
    );

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: map_transform,
            ..default()
        },
        SolidBundle::new(collider),
    ));
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("player.png");
    commands.spawn(PlayerBundle::new(texture));
}
