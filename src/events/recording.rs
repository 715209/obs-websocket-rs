use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Note: recordingFilename is not provided in this event because this
/// information is not available at the time this event is emitted.
/// Added in v0.3
pub struct RecordingStarting {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Recording started successfully.
/// Added in v0.3
pub struct RecordingStarted {
    /// Absolute path to the file of the current recording.
    pub recording_filename: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A request to stop recording has been issued.
/// Added in v0.3
pub struct RecordingStopping {
    /// Absolute path to the file of the current recording.
    pub recording_filename: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Recording stopped successfully.
/// Added in v0.3
pub struct RecordingStopped {
    /// Absolute path to the file of the current recording.
    pub recording_filename: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Current recording paused
/// Added in v4.7.0
pub struct RecordingPaused {}

#[derive(Serialize, Deserialize, Debug)]
/// Current recording resumed
/// Added in v4.7.0
pub struct RecordingResumed {}
