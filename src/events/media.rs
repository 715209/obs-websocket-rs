use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Note: This event is only emitted when something actively controls the
/// media/VLC source. In other words, the source will never emit this on
/// its own naturally.
/// Unreleased
pub struct MediaPlaying {
    /// Source name
    pub source_name: String,

    /// The ID type of the source (Eg. vlc_source or ffmpeg_source)
    pub source_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Note: This event is only emitted when something actively controls the
/// media/VLC source. In other words, the source will never emit this on
/// its own naturally.
/// Unreleased
pub struct MediaPaused {
    /// Source name
    pub source_name: String,

    /// The ID type of the source (Eg. vlc_source or ffmpeg_source)
    pub source_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Note: This event is only emitted when something actively controls the
/// media/VLC source. In other words, the source will never emit this on its
/// own naturally.
/// Unreleased
pub struct MediaRestarted {
    /// Source name
    pub source_name: String,

    /// The ID type of the source (Eg. vlc_source or ffmpeg_source)
    pub source_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Note: This event is only emitted when something actively controls the
/// media/VLC source. In other words, the source will never emit this on its
/// own naturally.
/// Unreleased
pub struct MediaStopped {
    /// Source name
    pub source_name: String,

    /// The ID type of the source (Eg. vlc_source or ffmpeg_source)
    pub source_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Note: This event is only emitted when something actively controls the
/// media/VLC source. In other words, the source will never emit this on its
/// own naturally.
/// Unreleased
pub struct MediaNext {
    /// Source name
    pub source_name: String,

    /// The ID type of the source (Eg. vlc_source or ffmpeg_source)
    pub source_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Note: This event is only emitted when something actively controls the
/// media/VLC source. In other words, the source will never emit this on its
/// own naturally.
/// Unreleased
pub struct MediaPrevious {
    /// Source name
    pub source_name: String,

    /// The ID type of the source (Eg. vlc_source or ffmpeg_source)
    pub source_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Note: These events are emitted by the OBS sources themselves. For example
/// when the media file starts playing. The behavior depends on the type of
/// media source being used.
/// Unreleased
pub struct MediaStarted {
    /// Source name
    pub source_name: String,

    /// The ID type of the source (Eg. vlc_source or ffmpeg_source)
    pub source_kind: String,
}

/// Note: These events are emitted by the OBS sources themselves. For example
/// when the media file ends. The behavior depends on the type of media
/// source being used.
/// Unreleased
pub struct MediaEnded {
    /// Source name
    pub source_name: String,

    /// The ID type of the source (Eg. vlc_source or ffmpeg_source)
    pub source_kind: String,
}
