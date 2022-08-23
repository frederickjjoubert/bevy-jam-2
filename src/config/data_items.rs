use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};
use crate::game::items::{Item, ItemId};
use crate::positioning::Dimens;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ItemsData {
    pub items: Vec<(Dimens, Item)>,
}

impl ItemsData {
    #[must_use]
    pub fn load_from_file() -> ItemsData {
        let override_file = get_config_override_dir().join("items.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("items.ron"))
        }
    }

    pub fn get_random_item(&self) -> (Dimens, Item) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.items.len());
        self.items.get(index).unwrap().clone()
    }

    pub fn get_item(&self, item_id: ItemId) -> Option<(Dimens, Item)> {
        self.items.iter().find(|(dimens, item)| item.id == item_id).cloned()
    }
}

fn load_from_path(path: &Path) -> ItemsData {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<ItemsData>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the items data file from {:?}! Falling back to ItemsData::default(). Error: {:?}",
                    path, error
                );
            ItemsData::default()
        })
}
