use crate::animation::AnimationTimer;
use bevy::prelude::*;
use std::time::Duration;

use crate::config::config_grid::GridConfig;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, TextureId};
use crate::positioning::Depth;
use crate::positioning::Dimens;
use crate::positioning::{Coords, Pos};
use crate::positioning::{Grid, GridCell};

use super::CombineButton;

// TODO: Jazz idk how to use AssetStorage for fonts
pub fn create_grids(
    mut commands: Commands,
    config: Res<GridConfig>,
    assets: Res<AssetStorage>,
    asset_server: Res<AssetServer>,
) {
    create_grid(&mut commands, &config.inventory);
    create_grid(&mut commands, &config.crafting);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(config.event_feed.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                config.event_feed.pos.x as f32 + config.event_feed.dimens.x as f32 * 0.5,
                config.event_feed.pos.y as f32 + config.event_feed.dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("EventFeed"))
        .insert(CleanupOnGameplayEnd);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(config.record_player.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                config.record_player.pos.x as f32 + config.record_player.dimens.x as f32 * 0.5,
                config.record_player.pos.y as f32 + config.record_player.dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("MusicArea"))
        .insert(CleanupOnGameplayEnd)
        .with_children(|parent| {
            let coords = Coords::new(Pos::new(1, 1), Dimens::new(2, 2));
            parent
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(coords.dimens.as_vec2()),
                        index: 0,
                        ..default()
                    },
                    texture_atlas: assets.atlas(&TextureId::RecordPlayer),
                    transform: Transform::from_xyz(
                        coords.pos.x as f32 + coords.dimens.x as f32 * 0.5
                            - config.record_player.dimens.x as f32 * 0.5,
                        coords.pos.y as f32 + coords.dimens.y as f32 * 0.5
                            - config.record_player.dimens.y as f32 * 0.5,
                        1., // Relative to parent grid.
                    ),
                    ..default()
                })
                .insert(coords)
                .insert(AnimationTimer {
                    timer: Timer::new(Duration::from_millis(200), true),
                    index: 0,
                    nr_frames: 2,
                    ping_pong: false,
                })
                .insert(Name::new("RecordPlayer"));
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(config.lower_bar.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                config.lower_bar.pos.x as f32 + config.lower_bar.dimens.x as f32 * 0.5,
                config.lower_bar.pos.y as f32 + config.lower_bar.dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("LowerBar"))
        .insert(CleanupOnGameplayEnd);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(config.combine.dimens.as_vec2()),
                ..default()
            },

            transform: Transform::from_xyz(
                config.combine.pos.x as f32 + config.combine.dimens.x as f32 * 0.5,
                config.combine.pos.y as f32 + config.combine.dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("Combine Trigger"))
        .insert(CombineButton {
            coords: config.combine,
        })
        .insert(CleanupOnGameplayEnd);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(config.drop_in.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                config.drop_in.pos.x as f32 + config.drop_in.dimens.x as f32 * 0.5,
                config.drop_in.pos.y as f32 + config.drop_in.dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("DropIn"))
        .insert(CleanupOnGameplayEnd);

    let parent_coords = config.equipped.coords;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(parent_coords.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                parent_coords.pos.x as f32 + parent_coords.dimens.x as f32 * 0.5,
                parent_coords.pos.y as f32 + parent_coords.dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("InventoryGrid"))
        .insert(CleanupOnGameplayEnd)
        .with_children(|parent| {
            config.equipped.slots.iter().for_each(|(slot, coords)| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.8, 0.8, 0.8, 0.5),
                            custom_size: Some(coords.dimens.as_vec2()),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (coords.pos.x as f32 + coords.dimens.x as f32 * 0.5)
                                - (parent_coords.dimens.x as f32 * 0.5),
                            (coords.pos.y as f32 + coords.dimens.y as f32 * 0.5)
                                - (parent_coords.dimens.y as f32 * 0.5),
                            1., // Relative to parent grid.
                        ),
                        ..default()
                    })
                    .insert(Name::new(format!("{:?}", slot)));
            });
        });
}

fn create_grid(commands: &mut Commands, coords: &Coords) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(coords.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                coords.pos.x as f32 + coords.dimens.x as f32 * 0.5,
                coords.pos.y as f32 + coords.dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("Grid"))
        .insert(Grid::default())
        .insert(CleanupOnGameplayEnd)
        .with_children(|grid| {
            for y in 0..coords.dimens.y {
                for x in 0..coords.dimens.x {
                    grid.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.8, 0.8, 0.8, 0.5),
                            custom_size: Some(Dimens::unit().as_vec2()),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 + 0.5) - (coords.dimens.x as f32 * 0.5),
                            (y as f32 + 0.5) - (coords.dimens.y as f32 * 0.5),
                            1., // Relative to parent grid.
                        )
                        .with_scale(Vec3::new(0.9, 0.9, 1.)),
                        ..default()
                    })
                    .insert(GridCell::default());
                }
            }
        });
}
