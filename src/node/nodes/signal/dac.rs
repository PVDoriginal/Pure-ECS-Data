use bevy_seedling::{edge::Connect, prelude::MainBus};

use super::*;

#[derive(Default, Diff, Patch, Debug, Reflect, Clone, Copy, PartialEq, Component)]
pub struct DacS;

impl Node<0, 2, 0, 0> for DacS {}

impl NodeComponent for DacS {
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
pub struct DacSConfig;

impl AudioNode for DacS {
    type Configuration = DacSConfig;

    fn info(&self, _config: &Self::Configuration) -> AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("dac signal node")
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
