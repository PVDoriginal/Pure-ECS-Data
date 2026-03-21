use firewheel::Volume;

use super::*;

#[derive(Diff, Patch, Debug, Clone, Copy, PartialEq, Component)]
pub struct OscS {
    pub gain: Volume,
}

impl Node<2, 1> for OscS {}

impl Default for OscS {
    fn default() -> Self {
        Self {
            gain: Volume::Linear(0.4),
        }
    }
}

#[derive(Default, Clone, PartialEq, Component)]
pub struct OscSConfig;

impl AudioNode for OscS {
    type Configuration = OscSConfig;

    fn info(&self, _config: &Self::Configuration) -> AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("example_noise_gen")
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::ZERO,
                num_outputs: ChannelCount::MONO,
            })
    }

    fn construct_processor(
        &self,
        _config: &Self::Configuration,
        _cx: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        Processor {
            fpd: 13,
            gain: self.gain.amp_clamped(DEFAULT_AMP_EPSILON),
            params: *self,
        }
    }
}

struct Processor {
    fpd: u32,
    gain: f32,
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
            let OscSPatch::Gain(vol) = &patch;
            self.gain = vol.amp_clamped(DEFAULT_AMP_EPSILON);

            self.params.apply(patch);
        }

        if self.gain == 0.0 {
            return ProcessStatus::ClearAllOutputs;
        }

        for s in buffers.outputs[0].iter_mut() {
            self.fpd ^= self.fpd << 13;
            self.fpd ^= self.fpd >> 17;
            self.fpd ^= self.fpd << 5;

            // Get a random normalized value in the range `[-1.0, 1.0]`.
            let r = self.fpd as f32 * (2.0 / 4_294_967_295.0) - 1.0;
            *s = self.gain * r;
            println!("{s}");
        }
        ProcessStatus::OutputsModified
    }
}
