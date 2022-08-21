use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};
use crate::grid::dimens::Dimens;
use crate::grid::pos::Pos;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct GridConfig {
    pub dimens: Dimens,
    pub pos: Pos,
    pub tile_size: i32,
}

impl GridConfig {
    /// Loads the most relevant instance of `GridConfig`.
    ///
    /// If the `GridConfig` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `GridConfig::default()`).
    ///
    /// If the 'GridConfig' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> GridConfig {
        let override_file = get_config_override_dir().join("grid.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("grid.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> GridConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<GridConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the grid config file from {:?}! Falling back to GridConfig::default(). Error: {:?}",
                    path, error
                );
            GridConfig::default()
        })
}
