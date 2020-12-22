use crate::typedefs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// The selected preview scene has changed (only available in Studio Mode).
/// Added in v4.1.0
pub struct PreviewSceneChanged {
    /// Name of the scene being previewed.
    pub scene_name: String,

    /// List of sources composing the scene.
    /// Same specification as GetCurrentScene.
    pub sources: Vec<typedefs::SceneItem>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// Studio Mode has been enabled or disabled.
/// Added in v4.1.0
pub struct StudioModeSwitched {
    /// The new enabled state of Studio Mode.
    pub new_state: bool,
}
