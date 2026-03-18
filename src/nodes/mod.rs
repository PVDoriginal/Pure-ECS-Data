use bevy::prelude::*;

use crate::{nodes::data::Data, patch::Input};

pub mod data;

#[derive(Component)]
pub struct Outlet {
    pub node: Entity,
    pub inlets: Vec<Entity>,
}

#[derive(Component)]
pub struct Inlet {
    pub node: Entity,
    pub outlets: Vec<Entity>,
}

pub trait Node<const IN: usize, const OUT: usize> {
    /// Called when the first inlet of the Node receives input.
    fn process(&self, inputs: &[Data]) -> &[Data];
}

pub trait NodeComponent {
    fn spawn_component<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a>;
}

#[derive(Component, Default, Clone)]
pub struct Print(pub Data);

impl Node<1, 0> for Print {
    fn process(&self, inputs: &[Data]) -> &[Data] {
        println!("{:?}", inputs[0]);
        &[]
    }
}

impl NodeComponent for Print {
    fn spawn_component<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

#[derive(Component, Default, Clone)]
pub struct Bang;

impl Node<0, 1> for Bang {
    fn process(&self, _: &[Data]) -> &[Data] {
        &[Data::Bang]
    }
}

impl NodeComponent for Bang {
    fn spawn_component<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}

#[derive(Component)]
pub(crate) struct Active;

#[derive(Component)]
pub(crate) struct DisableNextFrame;

pub(crate) struct NodesPlugin;

impl Plugin for NodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_node::<Print>();
    }
}

trait AddNode<const IN: usize, const OUT: usize> {
    fn add_node<N: Node<IN, OUT> + Component>(&mut self);
}

impl<const IN: usize, const OUT: usize> AddNode<IN, OUT> for App {
    fn add_node<N: Node<IN, OUT> + Component>(&mut self) {
        self.add_systems(PreUpdate, activate_nodes::<IN, OUT, N>);
        self.add_systems(Update, process_active_nodes::<IN, OUT, N>);
    }
}

fn activate_nodes<const IN: usize, const OUT: usize, N: Node<IN, OUT> + Component>(
    nodes: Query<(Entity, &N, Option<&Input>, Has<DisableNextFrame>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, _node, input, disable_next_frame) in nodes {
        if let Some(Input { input }) = input {
            if input(keys.clone()) {
                commands.entity(entity).insert(Active);
                commands.entity(entity).insert(DisableNextFrame);
            } else {
                if disable_next_frame {
                    commands.entity(entity).remove::<DisableNextFrame>();
                    commands.entity(entity).remove::<Active>();
                }
            }
        }
    }
}

fn process_active_nodes<const IN: usize, const OUT: usize, N: Node<IN, OUT> + Component>(
    nodes: Query<&N, With<Active>>,
) {
    for node in nodes {
        node.process(&["lol".into()]);
    }
}
