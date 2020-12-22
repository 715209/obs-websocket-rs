use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// The active transition has been changed.
/// Added in v4.0.0
pub struct SwitchTransition {
    /// The name of the new active transition
    pub transition_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transitions {
    /// Transition name.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// The list of available transitions has been modified.
/// Transitions have been added, removed, or renamed.
/// Added in v4.0.0
pub struct TransitionListChanged {
    /// Transitions list.
    pub transitions: Vec<Transitions>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// The active transition duration has been changed.
/// Added in v4.0.0
pub struct TransitionDurationChanged {
    /// New transition duration.
    pub new_duration: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A transition (other than "cut") has begun.
/// Added in v4.0.0
pub struct TransitionBegin {
    /// Transition name.
    pub name: String,

    #[serde(rename = "type")]
    /// Transition type.
    pub kind: String,

    /// Transition duration (in milliseconds). Will be -1 for any
    /// transition with a fixed duration, such as a Stinger, due to
    /// limitations of the OBS API.
    pub duration: i32,

    /// Source scene of the transition
    pub from_scene: String,

    /// Destination scene of the transition
    pub to_scene: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A transition (other than "cut") has ended. Please note that the from-scene
/// field is not available in TransitionEnd.
/// Added in v4.8.0
pub struct TransitionEnd {
    /// Transition name.
    pub name: String,

    #[serde(rename = "type")]
    /// Transition type.
    pub kind: String,

    /// Transition duration (in milliseconds).
    pub duration: i32,

    /// Destination scene of the transition
    pub to_scene: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
/// A stinger transition has finished playing its video.
/// Added in v4.8.0
pub struct TransitionVideoEnd {
    /// Transition name.
    pub name: String,

    #[serde(rename = "type")]
    /// Transition type.
    pub kind: String,

    /// Transition duration (in milliseconds).
    pub duration: i32,

    /// Source scene of the transition
    pub from_scene: String,

    /// Destination scene of the transition
    pub to_scene: String,
}
