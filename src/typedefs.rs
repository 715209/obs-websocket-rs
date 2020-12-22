use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SceneItem {
    pub cy: f64,
    pub cx: f64,

    /// The point on the source that the item is manipulated from.
    /// The sum of 1=Left or 2=Right, and 4=Top or 8=Bottom, or omit to
    /// center on that axis.
    pub alignment: f64,

    /// The name of this Scene Item.
    pub name: String,

    /// Scene item ID
    pub id: usize,

    /// Whether or not this Scene Item is set to "visible".
    pub render: bool,

    /// Whether or not this Scene Item is muted.
    pub muted: bool,

    /// Whether or not this Scene Item is locked and can't be moved around
    pub locked: bool,

    pub source_cx: f64,
    pub source_cy: f64,

    #[serde(rename = "type")]
    /// Source type. Value is one of the following:
    /// "input", "filter", "transition", "scene" or "unknown".
    /// There are actually more types.
    pub kind: String,

    pub volume: f64,
    pub x: f64,
    pub y: f64,

    /// Name of the item's parent (if this item belongs to a group)
    pub parent_group_name: Option<String>,

    /// List of children (if this item is a group)
    pub group_children: Option<Vec<SceneItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    /// The x position of the scene item from the left.
    pub x: f64,

    /// The y position of the scene item from the top.
    pub y: f64,

    /// The point on the scene item that the item is manipulated from.
    pub alignment: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scale {
    /// The x-scale factor of the scene item.
    pub x: f64,

    /// The y-scale factor of the scene item.
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crop {
    /// The number of pixels cropped off the top of the scene item before
    /// scaling.
    pub top: i32,

    /// The number of pixels cropped off the right of the scene item before
    /// scaling.
    pub right: i32,

    /// The number of pixels cropped off the bottom of the scene item before
    /// scaling.
    pub bottom: i32,

    /// The number of pixels cropped off the left of the scene item before
    /// scaling.
    pub left: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BoundsKind {
    ObsBoundsStretch,
    ObsBoundsScaleInner,
    ObsBoundsScaleOuter,
    ObsBoundsScaleToWidth,
    ObsBoundsScaleToHeight,
    ObsBoundsMaxOnly,
    ObsBoundsNone,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bounds {
    #[serde(rename = "type")]
    /// Type of bounding box. Can be "OBS_BOUNDS_STRETCH",
    /// "OBS_BOUNDS_SCALE_INNER", "OBS_BOUNDS_SCALE_OUTER",
    /// "OBS_BOUNDS_SCALE_TO_WIDTH", "OBS_BOUNDS_SCALE_TO_HEIGHT",
    /// "OBS_BOUNDS_MAX_ONLY" or "OBS_BOUNDS_NONE".
    pub kind: BoundsKind,

    /// Alignment of the bounding box.
    pub alignment: i32,

    /// Width of the bounding box.
    pub x: f64,

    /// Height of the bounding box.
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SceneItemTransform {
    pub position: Position,
    pub scale: Scale,
    pub crop: Crop,
    pub bounds: Bounds,

    /// The clockwise rotation of the scene item in degrees around the point
    /// of alignment.
    pub rotation: f64,

    /// If the scene item is visible.
    pub visible: bool,

    /// If the scene item is locked in position.
    pub locked: bool,

    /// Base width (without scaling) of the source
    pub source_width: i32,

    /// Base source (without scaling) of the source
    pub source_height: i32,

    /// Scene item width (base source width multiplied by the horizontal scaling factor)
    pub width: f64,

    /// Scene item height (base source height multiplied by the vertical scaling factor)
    pub height: f64,

    /// Name of the item's parent (if this item belongs to a group)
    pub parent_group_name: Option<String>,

    /// List of children (if this item is a group)
    pub group_children: Option<Vec<SceneItemTransform>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ObsStats {
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

    /// Average frame render time (in milliseconds)
    pub average_frame_time: f64,

    /// Current CPU usage (percentage)
    pub cpu_usage: f64,

    /// Current RAM usage (in megabytes)
    pub memory_usage: f64,

    /// Free recording disk space (in megabytes)
    pub free_disk_space: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    /// Raw flags value
    pub raw_value: i32,

    /// Output uses audio
    pub audio: bool,

    /// Output uses video
    pub video: bool,

    /// Output is encoded
    pub encoded: bool,

    /// Output uses several audio tracks
    pub multi_track: bool,

    /// Output uses a service
    pub service: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    /// Output name
    pub name: String,

    #[serde(rename = "type")]
    /// Output type/kind
    pub kind: String,

    /// Video output width
    pub width: i32,

    /// Video output height
    pub height: i32,

    /// Output flags
    pub flags: Flags,

    /// Output name
    pub settings: serde_json::value::Value,

    /// Output status (active or not)
    pub active: bool,

    /// Output reconnection status (reconnecting or not)
    pub reconnecting: bool,

    /// Output congestion
    pub congestion: f64,

    /// Number of frames sent
    pub total_frames: i32,

    /// Number of frames dropped
    pub dropped_frames: i32,

    /// Total bytes sent
    pub total_bytes: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Scene {
    /// Name of the currently active scene.
    pub name: String,

    /// Ordered list of the current scene's source items.
    pub sources: Vec<SceneItem>,
}
