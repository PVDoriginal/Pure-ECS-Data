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

pub trait Node {
    /// Determines how many inlets this node has.
    fn inlets(&self) -> usize;

    /// Determines how many outlets this node has.
    fn outlets(&self) -> usize;

    /// Called when the first inlet of the Node receives input.
    fn process(&self, inputs: &[Data]) -> &[Data];
}

pub trait NodeComponent {
    fn spawn_component<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a>;
}

#[derive(Component, Default, Clone)]
pub struct Print(pub Data);

impl Node for Print {
    fn inlets(&self) -> usize {
        1
    }

    fn outlets(&self) -> usize {
        0
    }

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

impl Node for Bang {
    fn inlets(&self) -> usize {
        0
    }

    fn outlets(&self) -> usize {
        1
    }

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

trait AddNode {
    fn add_node<N: Node + Component>(&mut self);
}

impl AddNode for App {
    fn add_node<N: Node + Component>(&mut self) {
        self.add_systems(PreUpdate, activate_nodes::<N>);
        self.add_systems(Update, process_active_nodes::<N>);
    }
}

fn activate_nodes<N: Node + Component>(
    nodes: Query<(Entity, &N, Option<&Input>, Has<DisableNextFrame>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, _node, input, disable_next_frame) in nodes {
        if let Some(Input { input, toggle }) = input {
            if input(keys.clone()) {
                commands.entity(entity).insert(Active);
                if !toggle {
                    commands.entity(entity).insert(DisableNextFrame);
                }
            } else {
                if disable_next_frame {
                    commands.entity(entity).remove::<DisableNextFrame>();
                    commands.entity(entity).remove::<Active>();
                }
            }
        }
    }
}

fn process_active_nodes<N: Node + Component>(nodes: Query<&N, With<Active>>) {
    for node in nodes {
        node.process(&["lol".into()]);
    }
}
