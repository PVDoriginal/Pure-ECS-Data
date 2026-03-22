use bevy::prelude::*;
use bevy_seedling::SeedlingPlugin;

use crate::{node::NodesPlugin, patch::PatchPlugin};

pub mod node;

pub mod patch;

pub mod prelude {
    pub use crate::node::NodeComponent;
    pub use crate::node::nodes;
    pub use crate::node::{
        Node,
        data::{Data, Num},
        nodes::*,
    };
    pub use crate::patch::inputs::Input;
    pub use crate::patch::macros::*;
    pub use crate::patch::{AddPatch, NodeRef, Patch};
    pub use crate::{
        PureDataPlugin, inlet, keys, keys_internal, keys_once, keys_once_internal, outlet,
    };
    pub use paste::paste;
}

pub(crate) const RECURSION_LIMIT: usize = 5_000;

pub struct PureDataPlugin;

impl Plugin for PureDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PatchPlugin, NodesPlugin, SeedlingPlugin::default()));
    }
}
