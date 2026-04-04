use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct ReplicatedScenePath(pub String);

#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct GameScene;
