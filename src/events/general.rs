use crate::typedefs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// Emitted every 2 seconds after enabling it by calling SetHeartbeat.
/// Added in v0.3
pub struct Heartbeat {
    /// Toggles between every JSON message as an "I am alive" indicator.
    pub pulse: bool,

    /// (optional) 	Current active profile.
    pub current_profile: Option<String>,

    /// (optional) 	Current active scene.
    pub current_scene: Option<String>,

    /// (optional) 	Current streaming state.
    pub streaming: Option<bool>,

    /// (optional) 	Total time (in seconds) since the stream started.
    pub total_stream_time: Option<i32>,

    /// (optional) 	Total bytes sent since the stream started.
    pub total_stream_bytes: Option<i32>,

    /// (optional) 	Total frames streamed since the stream started.
    pub total_stream_frames: Option<i32>,

    /// (optional) 	Current recording state.
    pub recording: Option<bool>,

    /// (optional) 	Total time (in seconds) since recording started.
    pub total_record_time: Option<i32>,

    /// (optional) 	Total bytes recorded since the recording started.
    pub total_record_bytes: Option<i32>,

    /// (optional) 	Total frames recorded since the recording started.
    pub total_record_frames: Option<i32>,

    /// OBS Stats
    pub stats: typedefs::ObsStats,
}

#[derive(Serialize, Deserialize, Debug)]
/// A custom broadcast message, sent by the server, requested by one of the
/// websocket clients.
/// Added in v4.7.0
pub struct BroadcastCustomMessage {
    /// Identifier provided by the sender
    pub realm: String,

    /// User-defined data
    pub data: serde_json::value::Value,
}
