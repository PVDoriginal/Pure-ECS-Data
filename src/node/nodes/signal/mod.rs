use super::*;
use bevy_seedling::prelude::ChannelCount;
use firewheel::{
    channel_config::ChannelConfig,
    diff::{Diff, Patch},
    dsp::volume::DEFAULT_AMP_EPSILON,
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
    fn build(&self, app: &mut App) {}
}
