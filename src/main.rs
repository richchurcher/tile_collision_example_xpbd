use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_tilemap::prelude::*;
use bevy_xpbd_2d::prelude::*;

#[derive(Component)]
pub struct Player;

const SPEED: f32 = 4000.0;

fn spawn_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
) {
    commands.spawn(Camera2dBundle::default());

    let map_size = TilemapSize { x: 32, y: 1 };
    let map_type = TilemapType::default();
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let tile_size = TilemapTileSize { x: 16., y: 16. };
    let tilemap_entity = commands.spawn_empty().id();
    let grid_size = tile_size.into();
    let mut tile_storage = TileStorage::empty(map_size);

    commands.entity(tilemap_entity).with_children(|parent| {
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let tile_pos = TilePos { x, y };

                // In order to add colliders, we need the world position of the tile as a Vec2.
                let center = tile_pos.center_in_world(&grid_size, &map_type);

                let tile_entity = parent
                    .spawn((
                        // bevy_xpbd needs to know that the tile is a "wall", or in other words
                        // something the player object isn't supposed to pass through.
                        RigidBody::Static,
                        Collider::cuboid(16., 16.),
                        Transform::from_xyz(center.x, center.y, 0.),
                        // The remainder of the example is essentially unchanged from
                        // bevy_ecs_tilemap's `basic` example.
                        TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(0),
                            ..Default::default()
                        },
                    ))
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("tiles.png")),
            tile_size,
            ..Default::default()
        });
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            LockedAxes::ROTATION_LOCKED,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::RegularPolygon::new(20., 4).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_xyz(0., 100., 0.)
                    .with_rotation(Quat::from_rotation_z(PI / 4.)),
                ..default()
            },
            Player,
            RigidBody::Dynamic,
        ))
        .with_children(|p| {
            p.spawn((
                Collider::cuboid(30., 30.),
                Transform::from_translation(Vec3::ZERO)
                    .with_rotation(Quat::from_rotation_z(PI / 4.)),
            ));
        });
}

fn controls(
    mut exit: EventWriter<bevy::app::AppExit>,
    keyboard_input: Res<Input<KeyCode>>,
    mut linear_velocity: Query<&mut LinearVelocity, With<Player>>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut lv) = linear_velocity.get_single_mut() else {
        return;
    };
    let Ok(mut t) = player.get_single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::A) {
        lv.x = -SPEED * time.delta_seconds();
    } else if keyboard_input.pressed(KeyCode::D) {
        lv.x = SPEED * time.delta_seconds();
    } else {
        lv.x = 0.;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        lv.y = -SPEED * 25. * time.delta_seconds();
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(bevy::app::AppExit);
    }

    if keyboard_input.just_pressed(KeyCode::Return) {
        *t = Transform::from_xyz(0., 100., 0.).with_rotation(Quat::from_rotation_z(PI / 4.));
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from(
                            "Tilemap with XPBD colliders - A,D: move, Space: jump, Return: reset position, Esc: exit.",
                        ),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            TilemapPlugin,
        ))
        .add_systems(Startup, (spawn_tiles, spawn_player))
        .add_systems(Update, controls)
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.0))
        .run();
}
