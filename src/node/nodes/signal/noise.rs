use rand::{RngExt, SeedableRng, rngs::SmallRng};

use super::*;

#[derive(Diff, Patch, Debug, Clone, PartialEq, Reflect, Component)]
pub struct NoiseS {
    pub seed: f32,
}

impl Default for NoiseS {
    fn default() -> Self {
        Self { seed: 13.0 }
    }
}

impl Node<1, 0, 0, 1> for NoiseS {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 0] {
        self.seed = inputs[0].clone().into();
        []
    }
    fn continuous_activation() -> bool {
        true
    }
}

impl NodeComponent for NoiseS {
    fn spawn_component<'a>(
        &self,
        _data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

#[derive(Default, Clone, PartialEq, Component)]
pub struct NoiseSConfig;

impl AudioNode for NoiseS {
    type Configuration = NoiseSConfig;

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
        _: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        Processor {
            rng: SmallRng::seed_from_u64(13),
            seed: 13,
            params: self.clone(),
        }
    }
}

struct Processor {
    rng: SmallRng,
    seed: u64,
    params: NoiseS,
}

impl AudioNodeProcessor for Processor {
    fn process(
        &mut self,
        _info: &ProcInfo,
        buffers: ProcBuffers,
        events: &mut ProcEvents,
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        for patch in events.drain_patches::<NoiseS>() {
            let NoiseSPatch::Seed(seed) = &patch;

            if *seed as u64 != self.seed {
                self.seed = *seed as u64;
                self.rng = SmallRng::seed_from_u64(self.seed);
            }

            Patch::apply(&mut self.params, patch);
        }

        for s in buffers.outputs[0].iter_mut() {
            *s = self.rng.random_range(-1.0..1.0);
        }

        ProcessStatus::OutputsModified
    }
}
