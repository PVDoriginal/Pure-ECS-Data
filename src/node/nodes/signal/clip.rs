use super::*;

#[derive(Diff, Patch, Debug, Reflect, Clone, Copy, PartialEq, Component)]
pub struct ClipS(f32, f32);

impl Default for ClipS {
    fn default() -> Self {
        ClipS(-1.0, 1.0)
    }
}

impl Node<2, 1, 0, 1> for ClipS {
    fn process(&mut self, inputs: [Data; 2]) -> [Data; 0] {
        if matches!(inputs[0], Data::None) || matches!(inputs[1], Data::None) {
            return [];
        }

        self.0 = inputs[0].clone().into();
        self.1 = inputs[1].clone().into();
        []
    }
    fn continuous_activation() -> bool {
        true
    }
}

impl NodeComponent for ClipS {
    fn spawn_component<'a>(
        &self,
        _data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

#[derive(Default, Clone, PartialEq, Component)]
pub struct ClipSConfig;

impl AudioNode for ClipS {
    type Configuration = ClipSConfig;

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
            mn: -1.0,
            mx: 1.0,
            params: *self,
        }
    }
}

struct Processor {
    mn: f32,
    mx: f32,
    params: ClipS,
}

impl AudioNodeProcessor for Processor {
    fn process(
        &mut self,
        _info: &ProcInfo,
        buffers: ProcBuffers,
        events: &mut ProcEvents,
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        for patch in events.drain_patches::<ClipS>() {
            match patch {
                ClipSPatch::Field0(mn) => {
                    self.mn = mn;
                }
                ClipSPatch::Field1(mx) => {
                    self.mx = mx;
                }
            }

            Patch::apply(&mut self.params, patch);
        }

        if self.mx < self.mn {
            return ProcessStatus::ClearAllOutputs;
        }

        for (i, s) in buffers.outputs[0].iter_mut().enumerate() {
            *s = buffers.inputs[0][i].clamp(self.mn, self.mx);
        }

        ProcessStatus::OutputsModified
    }
}
