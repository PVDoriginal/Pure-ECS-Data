use crate::{node::data::Num, prelude::Data};

use super::nodes::*;
use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct Internal(pub Option<Data>);

impl From<Num> for Internal {
    fn from(value: Num) -> Internal {
        Internal(Some(Data::Num(value)))
    }
}

pub trait NodeComponent {
    fn spawn_component<'a>(&self, data: Internal, commands: &'a mut Commands)
    -> EntityCommands<'a>;

    fn internal(&self) -> Internal {
        Internal(None)
    }
}

impl NodeComponent for Print {
    fn spawn_component<'a>(&self, _: Internal, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl NodeComponent for Bang {
    fn spawn_component<'a>(&self, _: Internal, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl<const N: usize> NodeComponent for super::nodes::Add<N> {
    fn spawn_component<'a>(&self, _: Internal, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl NodeComponent for super::nodes::F {
    fn spawn_component<'a>(&self, _: Internal, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl NodeComponent for Number {
    fn spawn_component<'a>(
        &self,
        internal: Internal,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        let mut comp = self.clone();

        match internal {
            Internal(Some(Data::Num(n))) => comp.0 = n,
            _ => {}
        }

        commands.spawn(comp)
    }

    fn internal(&self) -> Internal {
        self.0.clone().into()
    }
}
