use super::nodes::*;
use bevy::prelude::*;

pub trait NodeComponent {
    fn spawn_component<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a>;
}

impl NodeComponent for Print {
    fn spawn_component<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl NodeComponent for Bang {
    fn spawn_component<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}
