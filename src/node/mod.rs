use crate::{
    RECURSION_LIMIT,
    node::{
        connections::{
            CarriedData, Connections, InletOf, InletType, Inlets, OtherInlets, OutletOf, Outlets,
        },
        data::Data,
        nodes::*,
    },
    patch::inputs::Input,
};
use bevy::{ecs::component::Mutable, prelude::*};

pub mod connections;
pub mod data;
pub mod nodes;

pub trait Node<const IN: usize, const OUT: usize> {
    /// Called when the first inlet of the Node receives input.
    fn process(&mut self, inputs: [Data; IN]) -> [Data; OUT];

    fn outlet_order() -> [usize; OUT] {
        let mut array = [0; OUT];
        for i in 0..OUT {
            array[i] = i;
        }
        array
    }

    fn argument_order(&self) -> [usize; IN] {
        let mut array = [0; IN];
        for i in 0..IN {
            array[i] = i;
        }
        array
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

#[derive(EntityEvent)]
pub(crate) struct ActivateNode {
    pub entity: Entity,
    pub inputs: Option<Vec<Data>>,
    pub recursion: usize,
}

pub(crate) struct NodesPlugin;

impl Plugin for NodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ControlNodesPlugin, SignalNodesPlugin));
    }
}

trait AddNode<const IN: usize, const OUT: usize> {
    fn add_node<N: Node<IN, OUT> + Component<Mutability = Mutable>>(&mut self) -> &mut Self;
}

impl<const IN: usize, const OUT: usize> AddNode<IN, OUT> for App {
    fn add_node<N: Node<IN, OUT> + Component<Mutability = Mutable>>(&mut self) -> &mut Self {
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
                inputs: None,
                recursion: 0,
            });
        }
    }
}

// Triggers when a Node is activated.
fn on_node_activation<
    const IN: usize,
    const OUT: usize,
    N: Node<IN, OUT> + Component<Mutability = Mutable>,
>(
    trigger: On<ActivateNode>,
    mut nodes: Query<(&mut N, &Inlets, &Outlets)>,
    mut inlets: Query<(&mut CarriedData, &InletOf, &OtherInlets)>,
    outlets: Query<&Connections, With<OutletOf>>,
    mut commands: Commands,
) {
    if trigger.recursion >= RECURSION_LIMIT {
        error!("Recursion limit reached: {RECURSION_LIMIT}");
        return;
    }

    let Ok((mut node, node_inlets, node_outlets)) = nodes.get_mut(trigger.entity) else {
        return;
    };

    // Gathers inputs from all inlets.
    let mut inputs = [const { Data::None }; IN];

    if let Some(cached_inputs) = &trigger.inputs {
        for (i, input) in cached_inputs.iter().enumerate() {
            inputs[i] = input.clone();
        }
    } else {
        for (i, inlet) in node_inlets.collection().iter().enumerate() {
            let (inlet_data, _, _) = inlets.get(*inlet).unwrap();
            inputs[i] = inlet_data.0.clone();
        }
    }

    // Converts inputs to outputs.
    let outputs = node.process(inputs);

    let outlets: Vec<_> = node_outlets
        .collection()
        .iter()
        .map(|e| outlets.get(*e).unwrap())
        .collect();

    let order = N::outlet_order();
    let mut outlets: Vec<_> = outlets.iter().zip(order.iter()).collect();
    outlets.sort_by_key(|(_, i)| **i);

    let outlets: Vec<_> = outlets.iter().map(|(c, _)| c).collect();

    let mut queue_activation = vec![];

    // Writes the relevant output to each inlet connected to one of this node's outlets.
    // If input was put in a Hot inlet, queues that node for activation.
    outlets.iter().enumerate().for_each(|(i, c)| {
        for inlet in &c.0 {
            let (mut inlet_data, inlet_of, other_inlets) = inlets.get_mut(*inlet).unwrap();
            inlet_data.0.assign(outputs[i].clone());

            if matches!(inlet_of.inlet_type, InletType::Hot) {
                queue_activation.push((inlet_of.entity, other_inlets.0.clone()));
            }
        }
    });

    for (queued_node, queued_inlets) in queue_activation {
        let mut inputs = vec![];

        for inlet in queued_inlets.iter() {
            let (inlet_data, _, _) = inlets.get(*inlet).unwrap();
            inputs.push(inlet_data.0.clone());
        }

        commands.entity(queued_node).trigger(|entity| ActivateNode {
            entity,
            inputs: Some(inputs),
            recursion: trigger.recursion + 1,
        });
    }
}
