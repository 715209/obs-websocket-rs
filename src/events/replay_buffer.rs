use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// A request to start the replay buffer has been issued.
/// Added in v4.2.0
pub struct ReplayStarting {}

#[derive(Serialize, Deserialize, Debug)]
/// Replay Buffer started successfully
/// Added in v4.2.0
pub struct ReplayStarted {}

#[derive(Serialize, Deserialize, Debug)]
/// A request to stop the replay buffer has been issued.
/// Added in v4.2.0
pub struct ReplayStopping {}

#[derive(Serialize, Deserialize, Debug)]
/// Replay Buffer stopped successfully
/// Added in v4.2.0
pub struct ReplayStopped {}
