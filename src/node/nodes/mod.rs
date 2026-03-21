use super::Node;
use bevy::prelude::*;

mod control;
mod signal;

pub use control::*;
pub use signal::*;

use crate::node::data::{Data, Num};

use crate::node::AddNode;
use crate::prelude::NodeComponent;

use seq_macro::seq;
