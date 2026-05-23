use std::f32::consts::TAU;

use super::*;

#[derive(Diff, Default, Patch, Debug, Clone, PartialEq, Reflect, Component)]
pub struct VlineS {
    pub attack: f32, 
    pub decay: f32, 
    pub release: f32, 
    pub delta_time: f32, 
    pub active: bool,
}


impl Node<1, 0, 0, 1> for VlineS {
    fn process(&mut self, _: [Data; 1]) -> [Data; 0] {
        self.active = true; 
        []
    }
}

impl NodeComponent for VlineS {
    fn spawn_component<'a>(
        &self,
        _data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

#[derive(Default, Clone, PartialEq, Component)]
pub struct VlineSConfig;

impl AudioNode for VlineS {
    type Configuration = VlineSConfig;

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
            current: 0.0,
            time_since_press: 0.0,
            sample_rate: u32::from(cx.stream_info.sample_rate) as f32,
            params: self.clone(),
        }
    }
}

struct Processor {
    current: f32, 
    time_since_press: f32, 
    sample_rate: f32,
    params: VlineS,
}

impl AudioNodeProcessor for Processor {
    fn process(
        &mut self,
        _info: &ProcInfo,
        buffers: ProcBuffers,
        events: &mut ProcEvents,
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        for patch in events.drain_patches::<VlineS>() {
            Patch::apply(&mut self.params, patch);
        }

        let time_inc = self.params.delta_time / self.sample_rate; 
        let mut time = if self.params.active {0.0} else {self.time_since_press};

        for s in buffers.outputs[0].iter_mut() {
            if !self.params.active {
                time += time_inc;
            }

            
            if(time < self.params.attack + self.params.decay ){
                *s = 1.;
            }
            else if (time > self.params.attack + self.params.decay + self.params.release){
                *s = 0.;
            }
            else{
                *s = 1. - ( time - self.params.attack - self.params.decay ) / self.params.release
            }
            
        }

        self.time_since_press = time;

        ProcessStatus::OutputsModified
    }
}
