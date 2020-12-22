use crate::typedefs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct SceneItem {
    /// Item source name
    pub source_name: String,

    /// Scene item unique ID
    pub item_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// Scene items within a scene have been reordered.
/// Added in v4.0.0
pub struct SourceOrderChanged {
    /// Name of the scene where items have been reordered.
    pub scene_name: String,

    /// Ordered list of scene items
    pub scene_items: Vec<SceneItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A scene item has been added to a scene.
/// Added in v4.0.0
pub struct SceneItemAdded {
    /// Name of the scene.
    pub scene_name: String,

    /// Name of the item added to the scene.
    pub item_name: String,

    /// Scene item ID
    pub item_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A scene item has been removed from a scene.
/// Added in v4.0.0
pub struct SceneItemRemoved {
    /// Name of the scene.
    pub scene_name: String,

    /// Name of the item removed from the scene.
    pub item_name: String,

    /// Scene item ID
    pub item_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A scene item's visibility has been toggled.
/// Added in v4.0.0
pub struct SceneItemVisibilityChanged {
    /// Name of the scene.
    pub scene_name: String,

    /// Name of the item in the scene.
    pub item_name: String,

    /// Scene item ID
    pub item_id: i32,

    /// New visibility state of the item.
    pub item_visible: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A scene item's locked status has been toggled.
/// Added in v4.8.0
pub struct SceneItemLockChanged {
    /// Name of the scene.
    pub scene_name: String,

    /// Name of the item in the scene.
    pub item_name: String,

    /// Scene item ID
    pub item_id: i32,

    /// New locked state of the item.
    pub item_locked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A scene item's transform has been changed.
/// Added in v4.6.0
pub struct SceneItemTransformChanged {
    /// Name of the scene.
    pub scene_name: String,

    /// Name of the item in the scene.
    pub item_name: String,

    /// Scene item ID
    pub item_id: i32,

    /// Scene item transform properties
    pub transform: typedefs::SceneItemTransform,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A scene item is selected.
/// Added in v4.6.0
pub struct SceneItemSelected {
    /// Name of the scene.
    pub scene_name: String,

    /// Name of the item in the scene.
    pub item_name: String,

    /// Name of the item in the scene.
    pub item_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A scene item is deselected.
/// Added in v4.6.0
pub struct SceneItemDeselected {
    /// Name of the scene.
    pub scene_name: String,

    /// Name of the item in the scene.
    pub item_name: String,

    /// Name of the item in the scene.
    pub item_id: i32,
}
