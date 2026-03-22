use super::*;

#[derive(Default, Diff, Patch, Debug, Reflect, Clone, Copy, PartialEq, Component)]
pub struct MultS(f32);

impl Node<1, 1, 0, 1> for MultS {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 0] {
        self.0 = inputs[0].clone().into();
        []
    }
    fn continuous_activation() -> bool {
        true
    }
}

impl NodeComponent for MultS {
    fn spawn_component<'a>(
        &self,
        _data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

#[derive(Default, Clone, PartialEq, Component)]
pub struct MultSConfig;

impl AudioNode for MultS {
    type Configuration = MultSConfig;

    fn info(&self, _config: &Self::Configuration) -> AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("dac signal node")
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::MONO,
                num_outputs: ChannelCount::MONO,
            })
    }

    fn construct_processor(
        &self,
        _config: &Self::Configuration,
        _cx: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        Processor {
            multi: 0.0,
            params: *self,
        }
    }
}

struct Processor {
    multi: f32,
    params: MultS,
}

impl AudioNodeProcessor for Processor {
    fn process(
        &mut self,
        _info: &ProcInfo,
        buffers: ProcBuffers,
        events: &mut ProcEvents,
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        for patch in events.drain_patches::<MultS>() {
            let MultSPatch::Field0(multi) = &patch;
            self.multi = *multi;

            Patch::apply(&mut self.params, patch);
        }

        if self.multi == 0.0 {
            return ProcessStatus::ClearAllOutputs;
        }

        for (i, s) in buffers.outputs[0].iter_mut().enumerate() {
            *s = buffers.inputs[0][i] * self.multi;
        }

        ProcessStatus::OutputsModified
    }
}
