use super::*;

#[derive(Component, Clone, Reflect)]
pub struct Msg(pub Data);

impl Default for Msg {
    fn default() -> Self {
        Msg(Data::None)
    }
}

impl From<Data> for Msg {
    fn from(value: Data) -> Self {
        Msg(value)
    }
}

impl Node<1, 0, 1, 0> for Msg {
    fn process(&mut self, _: [Data; 1]) -> [Data; 1] {
        [self.0.clone()]
    }
}

impl NodeComponent for Msg {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        commands.spawn(Msg::from(data[0].clone()))
    }

    fn internal_data(&self) -> Vec<Data> {
        vec![self.0.clone()]
    }
}
