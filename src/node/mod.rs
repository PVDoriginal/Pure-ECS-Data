use bevy::prelude::*;

use crate::{
    RECURSION_LIMIT,
    node::{
        connections::{CarriedData, Connections, InletOf, InletType, Inlets, OutletOf, Outlets},
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

#[derive(EntityEvent)]
pub(crate) struct ActivateNode {
    pub entity: Entity,
    pub recursion: usize,
}

pub(crate) struct NodesPlugin;

impl Plugin for NodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_node::<Print>().add_node::<Bang>();
    }
}

trait AddNode<const IN: usize, const OUT: usize> {
    fn add_node<N: Node<IN, OUT> + Component>(&mut self) -> &mut Self;
}

impl<const IN: usize, const OUT: usize> AddNode<IN, OUT> for App {
    fn add_node<N: Node<IN, OUT> + Component>(&mut self) -> &mut Self {
        self.add_systems(Update, activate_nodes_on_input::<IN, OUT, N>);
        self.add_observer(on_node_activation::<IN, OUT, N>);
        self.register_required_components::<N, Inlets>();
        self.register_required_components::<N, Outlets>();
        self
    }
}

// Iterates over all nodes that have binded input, queue them for activation is input is valid.
fn activate_nodes_on_input<const IN: usize, const OUT: usize, N: Node<IN, OUT> + Component>(
    nodes: Query<(Entity, &N, &Input)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for (entity, _node, input) in nodes {
        if (input.input)(keys.clone()) {
            commands.entity(entity).trigger(|entity| ActivateNode {
                entity,
                recursion: 0,
            });
        }
    }
}

// Triggers when a Node is activated.
fn on_node_activation<const IN: usize, const OUT: usize, N: Node<IN, OUT> + Component>(
    trigger: On<ActivateNode>,
    nodes: Query<(&N, &Inlets, &Outlets)>,
    mut inlets: Query<(&mut CarriedData, &Connections, &InletOf)>,
    outlets: Query<&Connections, With<OutletOf>>,
    mut commands: Commands,
) {
    if trigger.recursion >= RECURSION_LIMIT {
        error!("Recursion limit reached: {RECURSION_LIMIT}");
        return;
    }

    let Ok((node, node_inlets, node_outlets)) = nodes.get(trigger.entity) else {
        return;
    };

    // Gathers inputs from all inlets, emptying them.
    let mut inputs = [const { Data::None }; IN];

    for (i, inlet) in node_inlets.collection().iter().enumerate() {
        let (mut inlet_data, _, _) = inlets.get_mut(*inlet).unwrap();
        inputs[i] = inlet_data.0.clone();
        inlet_data.0 = Data::None;
    }

    // Converts inputs to outputs.
    let outputs = node.process(inputs);

    let outlets: Vec<_> = node_outlets
        .collection()
        .iter()
        .map(|e| outlets.get(*e).unwrap())
        .collect();

    // Writes the relevant output to each inlet connected to one of this node's outlets.
    // If input was put in a Hot inlet, queues that node for activation.
    outlets.iter().enumerate().for_each(|(i, c)| {
        for inlet in &c.0 {
            let (mut inlet_data, _, inlet_of) = inlets.get_mut(*inlet).unwrap();
            inlet_data.0 = outputs[i].clone();

            if matches!(inlet_of.inlet_type, InletType::Hot) {
                commands
                    .entity(inlet_of.entity)
                    .trigger(|entity| ActivateNode {
                        entity,
                        recursion: trigger.recursion + 1,
                    });
            }
        }
    });
}
