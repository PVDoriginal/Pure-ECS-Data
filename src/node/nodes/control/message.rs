use super::*;

#[derive(Component, Clone, Reflect)]
pub struct Message(pub Data);

impl Default for Message {
    fn default() -> Self {
        Message(Data::None)
    }
}

impl From<Data> for Message {
    fn from(value: Data) -> Self {
        Message(value)
    }
}

impl Node<1, 1> for Message {
    fn process(&mut self, _: [Data; 1]) -> [Data; 1] {
        [self.0.clone()]
    }
}

impl NodeComponent for Message {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        commands.spawn(Message::from(data[0].clone()))
    }

    fn internal_data(&self) -> Vec<Data> {
        vec![self.0.clone()]
    }
}
