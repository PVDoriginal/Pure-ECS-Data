use super::*;

#[derive(Component, Default, Clone, Reflect)]
pub struct Div;

impl Node<2, 0, 1, 0> for Div {
    fn process(&mut self, inputs: [Data; 2]) -> [Data; 1] {
        let mut res = Data::None;

        let a: f32 = inputs[0].clone().into();
        let b: f32 = inputs[1].clone().into();

        if b != 0. {
            res = Data::Num(Num::Float(a / b));
        }

        [res]
    }
}

impl NodeComponent for Div {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}
