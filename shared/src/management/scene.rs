use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Created on server and sent to clients
/// The string is a path which exists on all clients
///
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct ReplicatedScenePath(pub String);

/// Marker component for ```ReplicatedScenePath```
/// and in general, just game scenes
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct GameScene;

/// When this is spawned with ```ReplicatedScenePath``` and ```GameScene```
/// the game state does not continue until all scenes are fully loaded.
// sry future me btw still trying to figure out how to comment well
// considering there being three different crates to cross reference
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct PleaseLoad;

/// Sent from client to server to let the server know all assets are loaded on their end.
#[derive(Message, Reflect, Serialize, Deserialize)]
pub struct FinishedLoading;

#[derive(Message, Reflect, Serialize, Deserialize)]
pub struct AllFinishedLoading;
