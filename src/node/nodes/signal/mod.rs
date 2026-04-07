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

mod mult;
pub use mult::*;

mod noise;
pub use noise::*;

mod phasor;
pub use phasor::*;

mod minus;
pub use minus::*;

mod clip;
pub use clip::*;

pub(crate) struct SignalNodesPlugin;

impl Plugin for SignalNodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_node::<OscS>();
        app.add_audio_node::<DacS>();
        app.add_audio_node::<MultS>();
        app.add_audio_node::<MinusS>();
        app.add_audio_node::<NoiseS>();
        app.add_audio_node::<PhasorS>();
        app.add_audio_node::<ClipS>();
    }
}
