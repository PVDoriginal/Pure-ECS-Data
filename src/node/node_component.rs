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

pub trait NodeComponent: PartialReflect {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a>;

    fn internal_data(&self) -> Vec<Data> {
        vec![]
    }

    fn get_type(&self) -> &str {
        self.reflect_type_ident().unwrap()
    }
}

impl NodeComponent for Print {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl NodeComponent for Bang {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl<const N: usize> NodeComponent for super::nodes::Sum<N> {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl NodeComponent for super::nodes::F {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

impl NodeComponent for Number {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        let mut comp = self.clone();

        if let Some(Data::Num(n)) = data.first() {
            comp.0 = n.clone();
        }

        commands.spawn(comp)
    }

    fn internal_data(&self) -> Vec<Data> {
        vec![self.clone().0.into()]
    }
}

impl<const N: usize> NodeComponent for Trigger<N> {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        let mut comp = self.clone();

        for (i, data) in data.iter().enumerate() {
            comp.0[i] = data.clone();
        }

        commands.spawn(comp)
    }

    fn internal_data(&self) -> Vec<Data> {
        self.0.to_vec()
    }
}

impl NodeComponent for super::nodes::Message {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        commands.spawn(super::nodes::Message::from(data[0].clone()))
    }

    fn internal_data(&self) -> Vec<Data> {
        vec![self.0.clone()]
    }
}
