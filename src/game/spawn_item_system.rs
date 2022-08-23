use bevy::prelude::*;
use crate::config::config_grid::GridConfig;
use crate::config::data_items::ItemsData;
use crate::game::items::Item;
use crate::game::{AssetStorage, CleanupOnGameplayEnd};
use crate::positioning::Depth;
use crate::positioning::Pos;
use crate::positioning::{Coords};
use super::dragging_items_system::BeingDragged;

/// === Resources ===
pub struct ItemSpawnTimer(Timer);

/// === Events ===
/// Broadcast this as an event to spawn an item.
#[derive(Debug)]
pub struct SpawnItemEvent {
    item: Item,
    coords: Coords,
}

impl SpawnItemEvent {
    pub fn new(item: Item, coords: Coords) -> Self {
        SpawnItemEvent { item, coords }
    }
}

/// === Systems ===
pub fn setup_spawn_item_timer(mut commands: Commands) {
    commands.insert_resource(ItemSpawnTimer(Timer::from_seconds(5.0, true))); // Ref 1
}

pub fn update_spawn_item_timer(
    time: Res<Time>,
    grid: Res<GridConfig>,
    items_query: Query<&Coords, (With<Item>, Without<BeingDragged>)>,
    mut timer: ResMut<ItemSpawnTimer>,
    items_data: Res<ItemsData>,
    mut spawn: EventWriter<SpawnItemEvent>,
) {
    // update our timer with the time elapsed since the last update
    if timer.0.tick(time.delta()).just_finished() {
        let (dimens, item) = items_data.get_random_item();

        let mut free_coords: Option<Coords> = None;

        for y in 0..grid.inventory.dimens.y {
            for x in 0..grid.inventory.dimens.x {
                let coords = Coords {
                    pos: Pos::new(x, y),
                    dimens,
                };

                let overlap_conflict = items_query.iter().any(|item| coords.overlaps(item));
                let bound_conflict = !grid.inventory.encloses(&coords);
                if !overlap_conflict && !bound_conflict {
                    free_coords = Some(coords);
                    break;
                }
            }
        }
        if let Some(coords) = free_coords {
            spawn.send(SpawnItemEvent::new(item, coords))
        };
    }
}

pub fn spawn_new_items(
    mut commands: Commands,
    mut events: EventReader<SpawnItemEvent>,
    assets: Res<AssetStorage>,
) {
    for event in events.iter() {
        debug!("Received spawn item event: {:?}", event);

        let SpawnItemEvent { item, coords } = event;

        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(coords.dimens.as_vec2()),
                    ..default()
                },
                texture: assets.texture(&item.texture_id),
                transform: Transform::from_xyz(
                    coords.pos.x as f32 + coords.dimens.x as f32 * 0.5,
                    coords.pos.y as f32 + coords.dimens.y as f32 * 0.5,
                    Depth::Item.z(),
                ),
                ..Default::default()
            })
            .insert(Name::new(item.name.clone()))
            .insert(item.clone())
            .insert(*coords)
            .insert(CleanupOnGameplayEnd);
    }
}
