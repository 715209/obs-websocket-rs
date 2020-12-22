use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
/// Triggered when switching to another profile or when renaming
/// the current profile.
/// Added in v4.0.0
pub struct ProfileChanged {
    /// Name of the new current profile.
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    /// Profile name.
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Triggered when a profile is created, added, renamed, or removed.
/// Added in v4.0.0
pub struct ProfileListChanged {
    /// Profiles list.
    pub profiles: Vec<Profiles>,
}
