use std::f32::consts::TAU;

use super::*;

#[derive(Diff, Patch, Debug, Clone, PartialEq, Reflect, Component)]
pub struct OscS {
    pub hertz: f32,
}

impl Default for OscS {
    fn default() -> Self {
        Self { hertz: 0.0 }
    }
}

impl Node<1, 0, 0, 1> for OscS {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 0] {
        self.hertz = inputs[0].clone().into();
        []
    }
    fn continuous_activation() -> bool {
        true
    }
}

impl NodeComponent for OscS {
    fn spawn_component<'a>(
        &self,
        _data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

#[derive(Default, Clone, PartialEq, Component)]
pub struct OscSConfig;

impl AudioNode for OscS {
    type Configuration = OscSConfig;

    fn info(&self, _config: &Self::Configuration) -> AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("osc signal node")
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::ZERO,
                num_outputs: ChannelCount::MONO,
            })
    }

    fn construct_processor(
        &self,
        _config: &Self::Configuration,
        cx: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        Processor {
            phase: 0.0,
            hertz: self.hertz,
            sample_rate: u32::from(cx.stream_info.sample_rate) as f32,
            params: self.clone(),
        }
    }
}

struct Processor {
    phase: f32,
    hertz: f32,
    sample_rate: f32,
    params: OscS,
}

impl AudioNodeProcessor for Processor {
    fn process(
        &mut self,
        _info: &ProcInfo,
        buffers: ProcBuffers,
        events: &mut ProcEvents,
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        for patch in events.drain_patches::<OscS>() {
            let OscSPatch::Hertz(hertz) = &patch;
            self.hertz = *hertz;

            Patch::apply(&mut self.params, patch);
        }

        if self.hertz == 0.0 {
            return ProcessStatus::ClearAllOutputs;
        }

        let phase_inc = self.hertz / self.sample_rate;

        for s in buffers.outputs[0].iter_mut() {
            let value = (self.phase * TAU).sin();
            *s = value;

            self.phase += phase_inc;

            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
        }

        ProcessStatus::OutputsModified
    }
}
