//! Level asset which holds references to all sub assets which also need to be loaded.
//! This can likely be simplified with BSN in 0.19/0.20 or we could track a BSN branch.
//! Either way, this should be fun to program as a sort of shared asset for both server and client.
//! The level can be represented as a level.zip or a /level/ or a level.lvl where the zip and folder contain the .lvl

// Initial attempt of the level asset and what might be described.
// I'm not to sure if this is even currently needed. I think I'll just make a playground.
//

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_asset::<Level>();
}

#[derive(Asset, TypePath, Debug)]
#[type_path = "hate"] // not shared::level::Level, instead this becomes hate::Level
pub struct Level {
    pub name: String,
    pub scenes: Vec<Handle<Scene>>,
}
