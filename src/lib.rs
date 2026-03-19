use bevy::prelude::*;

use crate::{node::NodesPlugin, patch::PatchPlugin};

pub mod node;

pub mod patch;

pub mod prelude {
    pub use crate::node::{
        data::{Data, Num},
        nodes::*,
    };
    pub use crate::patch::{AddPatch, NodeRef, Patch};
    pub use crate::{
        PureDataPlugin, inlet, keys, keys_internal, keys_once, keys_once_internal, outlet,
    };
}

pub(crate) const RECURSION_LIMIT: usize = 5_000;

pub struct PureDataPlugin;

impl Plugin for PureDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PatchPlugin, NodesPlugin));
    }
}
