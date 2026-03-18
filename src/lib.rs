use bevy::prelude::*;

use crate::{nodes::NodesPlugin, patch::PatchPlugin};

pub mod nodes;

pub mod patch;

pub mod prelude {
    pub use crate::patch::{AddPatch, Patch};
    pub use crate::{PureDataPlugin, inlet, keys, keys_internal, outlet};
}

pub struct PureDataPlugin;

impl Plugin for PureDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PatchPlugin, NodesPlugin));
    }
}
