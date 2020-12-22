use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A request to start streaming has been issued.
/// Added in v0.3
pub struct StreamStarting {
    /// Always false (retrocompatibility).
    pub preview_only: bool,
}

#[derive(Serialize, Deserialize, Debug)]
/// Streaming started successfully.
/// Added in v0.3
pub struct StreamStarted {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A request to stop streaming has been issued.
/// Added in v0.3
pub struct StreamStopping {
    /// Always false (retrocompatibility).
    pub preview_only: bool,
}

#[derive(Serialize, Deserialize, Debug)]
/// Streaming stopped successfully.
/// Added in v0.3
pub struct StreamStopped {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// Emitted every 2 seconds when stream is active.
/// Added in v0.3
pub struct StreamStatus {
    /// Current streaming state.
    pub streaming: bool,

    /// Current recording state.
    pub recording: bool,

    /// Replay Buffer status
    pub replay_buffer_active: bool,

    /// Amount of data per second (in bytes) transmitted by the
    /// stream encoder.
    pub bytes_per_sec: i32,

    /// Amount of data per second (in kilobits) transmitted by the
    /// stream encoder.
    pub kbits_per_sec: i32,

    /// Percentage of dropped frames.
    pub strain: f64,

    /// Total time (in seconds) since the stream started.
    pub total_stream_time: i32,

    /// Total number of frames transmitted since the stream started.
    pub num_total_frames: i32,

    /// Number of frames dropped by the encoder since the stream started.
    pub num_dropped_frames: i32,

    /// Current framerate.
    pub fps: f64,

    /// Number of frames rendered
    pub render_total_frames: i32,

    /// Number of frames missed due to rendering lag
    pub render_missed_frames: i32,

    /// Number of frames outputted
    pub output_total_frames: i32,

    /// Number of frames skipped due to encoding lag
    pub output_skipped_frames: i32,

    /// Average frame time (in milliseconds)
    pub average_frame_time: f64,

    /// Current CPU usage (percentage)
    pub cpu_usage: f64,

    /// Current RAM usage (in megabytes)
    pub memory_usage: f64,

    /// Free recording disk space (in megabytes)
    pub free_disk_space: f64,

    /// Always false (retrocompatibility).
    pub preview_only: bool,
}
