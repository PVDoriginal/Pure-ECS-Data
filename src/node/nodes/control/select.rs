use bevy_seedling::{edge::Connect, prelude::MainBus};
use firewheel::{
    channel_config::{ChannelConfig, ChannelCount},
    diff::{Diff, Patch},
    event::ProcEvents,
    node::{
        AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, ProcBuffers,
        ProcExtra, ProcInfo, ProcessStatus,
    },
};

use super::*;

/// equivalent to switch-case
///
/// N = nr of cases (should include default!)
#[derive(Diff, Patch, Debug, Reflect, Clone, Copy, PartialEq, Component)]
pub struct Select<const N: usize> {
    pos_vals: [f32; N],
}

impl<const N: usize> Default for Select<N> {
    fn default() -> Self {
        // fast, safe array init for Copy types
        Self {
            pos_vals: [0.0f32; N],
        }
    }
}

impl<const N: usize> Node<1, 0, N, 0> for Select<N> {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; N] {
        let mut res = [const { Data::None }; N];

        for i in 0..(self.pos_vals.len() - 1) {
            if self.pos_vals[i] == Into::<f32>::into(inputs[0].clone()) {
                res[i] = Data::Bang;
            }
            return res;
        }

        let pos: usize = N - 1;
        res[pos] = Data::Num(Num::Float(inputs[0].clone().into()));
        res
    }
}

impl<const N: usize> NodeComponent for Select<N> {
    fn spawn_component<'a>(
        &self,
        _data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        let entity = commands.spawn(self.clone()).id();
        commands.entity(entity).connect(MainBus);
        commands.entity(entity)
    }
}

#[derive(Default, Clone, PartialEq, Component)]
pub struct SelectConfig;

impl<const N: usize> AudioNode for Select<N> {
    type Configuration = SelectConfig;

    fn info(&self, _config: &Self::Configuration) -> AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("Select Signal Node")
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::STEREO,
                num_outputs: ChannelCount::STEREO,
            })
    }

    fn construct_processor(
        &self,
        _config: &Self::Configuration,
        _cx: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        Processor
    }
}

struct Processor;

impl AudioNodeProcessor for Processor {
    fn process(
        &mut self,
        _info: &ProcInfo,
        buffers: ProcBuffers,
        _events: &mut ProcEvents,
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        for (i, s) in buffers.outputs[0].iter_mut().enumerate() {
            *s = buffers.inputs[0][i];
        }
        for (i, s) in buffers.outputs[1].iter_mut().enumerate() {
            *s = buffers.inputs[1][i];
        }

        ProcessStatus::OutputsModified
    }
}
