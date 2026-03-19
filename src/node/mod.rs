use bevy::prelude::*;

use crate::{
    node::{
        connections::{CarriedData, Connections, InletOf, Inlets, OutletOf, Outlets},
        data::Data,
        nodes::*,
    },
    patch::inputs::Input,
};

pub mod connections;
pub mod data;
pub mod node_component;
pub mod nodes;

pub trait Node<const IN: usize, const OUT: usize> {
    /// Called when the first inlet of the Node receives input.
    fn process(&self, inputs: [Data; IN]) -> [Data; OUT];
}

#[derive(Component)]
pub(crate) struct Active;

#[derive(Component)]
pub(crate) struct DisableNextFrame;

pub(crate) struct NodesPlugin;

impl Plugin for NodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, move_signal);
        app.add_node::<Print>().add_node::<Bang>();
    }
}

trait AddNode<const IN: usize, const OUT: usize> {
    fn add_node<N: Node<IN, OUT> + Component>(&mut self) -> &mut Self;
}

impl<const IN: usize, const OUT: usize> AddNode<IN, OUT> for App {
    fn add_node<N: Node<IN, OUT> + Component>(&mut self) -> &mut Self {
        self.add_systems(Update, activate_nodes::<IN, OUT, N>);
        self.add_systems(PostUpdate, process_active_nodes::<IN, OUT, N>);
        self.register_required_components::<N, Inlets>();
        self.register_required_components::<N, Outlets>();
        self
    }
}

fn activate_nodes<const IN: usize, const OUT: usize, N: Node<IN, OUT> + Component>(
    nodes: Query<(Entity, &N, Option<&Input>, Has<DisableNextFrame>, &Inlets)>,
    inlets: Query<&CarriedData, With<InletOf>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, _node, input, disable_next_frame, node_inlets) in nodes {
        let active_inlet = node_inlets
            .collection()
            .first()
            .is_some_and(|i| !matches!(inlets.get(*i).unwrap().0, Data::None));

        if input.is_some_and(|i| (i.input)(keys.clone())) || active_inlet {
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

fn process_active_nodes<const IN: usize, const OUT: usize, N: Node<IN, OUT> + Component>(
    nodes: Query<(&N, &Inlets, &Outlets, Has<Active>)>,
    inlets: Query<(&CarriedData, &Connections), With<InletOf>>,
    mut commands: Commands,
) {
    for (node, node_inlets, node_outlets, is_active) in nodes {
        let mut outputs = [const { Data::None }; OUT];

        if is_active {
            let mut inputs = [const { Data::None }; IN];

            for (i, inlet) in node_inlets.collection().iter().enumerate() {
                let (inlet_data, _) = inlets.get(*inlet).unwrap();
                inputs[i] = inlet_data.0.clone();
            }

            outputs = node.process(inputs);
        }

        for (i, outlet) in node_outlets.collection().iter().enumerate() {
            commands
                .entity(*outlet)
                .insert(CarriedData(outputs[i].clone()));
        }
    }
}

fn move_signal(
    outlets: Query<(&CarriedData, &Connections), With<OutletOf>>,
    mut commands: Commands,
) {
    for (data, connections) in outlets {
        for connection in &connections.0 {
            commands.entity(*connection).insert((*data).clone());
        }
    }
}
