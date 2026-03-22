use super::*;
use bevy_seedling::{node::RegisterNode, prelude::ChannelCount};
use firewheel::{
    channel_config::ChannelConfig,
    diff::{Diff, Patch},
    event::ProcEvents,
    node::{
        AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, ProcBuffers,
        ProcExtra, ProcInfo, ProcessStatus,
    },
};

mod dac;
pub use dac::*;

mod osc;
pub use osc::*;

pub(crate) struct SignalNodesPlugin;

impl Plugin for SignalNodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_node::<OscS>();
        app.add_node::<DacS>();

        app.register_node::<OscS>();
        app.register_node::<DacS>();
    }
}
