use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SourceType {
    Input,
    Scene,
    Transition,
    Filter,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A source has been created.
/// A source can be an input, a scene or a transition.
/// Added in v4.6.0
pub struct SourceCreated {
    /// Source name
    pub source_name: String,

    /// Source type. Can be "input", "scene", "transition" or "filter".
    pub source_type: SourceType,

    /// Source kind.
    pub source_kind: String,

    /// Source settings
    // TODO: Remove the serde_json value
    pub source_settings: serde_json::value::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A source has been destroyed/removed.
/// A source can be an input, a scene or a transition.
/// Added in v4.6.0
pub struct SourceDestroyed {
    /// Source name
    pub source_name: String,

    /// Source type. Can be "input", "scene", "transition" or "filter".
    pub source_type: SourceType,

    /// Source kind.
    pub source_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The volume of a source has changed.
/// Added in v4.6.0
pub struct SourceVolumeChanged {
    /// Source name
    pub source_name: String,

    /// Source volume
    pub volume: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A source has been muted or unmuted.
/// Added in v4.6.0
pub struct SourceMuteStateChanged {
    /// Source name
    pub source_name: String,

    /// Mute status of the source
    pub muted: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A source has removed audio.
/// Unreleased
pub struct SourceAudioDeactivated {
    /// Source name
    pub source_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A source has added audio.
/// Unreleased
pub struct SourceAudioActivated {
    /// Source name
    pub source_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The audio sync offset of a source has changed.
/// Added in v4.6.0
pub struct SourceAudioSyncOffsetChanged {
    /// Source name
    pub source_name: String,

    /// Audio sync offset of the source (in nanoseconds)
    pub sync_offset: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mixer {
    /// Mixer number
    pub id: i32,

    /// Routing status
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Audio mixer routing changed on a source.
/// Added in v4.6.0
pub struct SourceAudioMixersChanged {
    /// Source name
    pub source_name: String,

    /// Routing status of the source for each audio mixer (array of 6 values)
    pub mixers: Vec<Mixer>,

    /// Raw mixer flags (little-endian, one bit per mixer) as an
    /// hexadecimal value
    pub hex_mixers_value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A source has been renamed.
/// Added in v4.6.0
pub struct SourceRenamed {
    /// Previous source name
    pub previous_name: String,

    /// New source name
    pub new_name: String,

    /// Type of source (input, scene, filter, transition)
    pub source_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A filter was added to a source.
/// Added in v4.6.0
pub struct SourceFilterAdded {
    /// Source name
    pub source_name: String,

    /// Filter name
    pub filter_name: String,

    /// Filter type
    pub filter_type: String,

    /// Filter settings
    // TODO: Remove the serde_json value
    pub filter_settings: serde_json::value::Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// A filter was removed from a source.
/// Added in v4.6.0
pub struct SourceFilterRemoved {
    /// Source name
    pub source_name: String,

    /// Filter name
    pub filter_name: String,

    /// Filter type
    pub filter_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// The visibility/enabled state of a filter changed
/// Added in v4.7.0
pub struct SourceFilterVisibilityChanged {
    /// Source name
    pub source_name: String,

    /// Filter name
    pub filter_name: String,

    /// New filter state
    pub filter_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    /// Filter name
    pub name: String,

    #[serde(rename = "type")]
    /// Filter type
    pub kind: String,

    /// Filter visibility status
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Filters in a source have been reordered.
/// Added in v4.6.0
pub struct SourceFiltersReordered {
    /// Source name
    pub source_name: String,

    /// Ordered Filters list
    pub filters: Vec<Filter>,
}
