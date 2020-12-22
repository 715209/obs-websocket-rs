use crate::typedefs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Indicates a scene change.
/// Added in v0.3
pub struct SwitchScenes {
    #[serde(rename = "kebab-case")]
    /// The new scene.
    pub scene_name: String,

    /// List of scene items in the new scene. Same specification as
    /// GetCurrentScene.
    pub sources: Vec<typedefs::SceneItem>,
}

#[derive(Serialize, Deserialize, Debug)]
/// Note: This event is not fired when the scenes are reordered.
/// Added in v0.3
pub struct ScenesChanged {
    /// Scenes list.
    pub scenes: Vec<typedefs::Scene>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Note: Triggered when switching to another scene collection or when renaming
/// the current scene collection.
/// Added in v4.0.0
pub struct SceneCollectionChanged {
    /// Name of the new current scene collection.
    pub scene_collection: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneCollections {
    /// Scene collection name.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Triggered when a scene collection is created, added, renamed, or removed.
/// Added in v4.0.0
pub struct SceneCollectionListChanged {
    /// Scene collections list.
    pub scene_collections: Vec<SceneCollections>,
}
