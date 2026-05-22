use super::*;

#[derive(Component, Default, Clone, Reflect)]
pub struct MtoF;

impl Node<1, 0, 1, 0> for MtoF {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 1] {
        let midi: f32 = inputs[0].clone().into();

        let res = Data::Num(Num::Float(440.0 * 2_f32.powf((midi - 69.0) / 12.0)));

        [res]
    }
}

impl NodeComponent for MtoF {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}
