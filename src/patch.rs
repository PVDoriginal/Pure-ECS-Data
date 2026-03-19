use bevy::{ecs::system::entity_command::despawn, platform::collections::HashMap, prelude::*};

use crate::nodes::{connections::Connections, node_component::NodeComponent};
pub struct Patch {
    pub(crate) nodes: Vec<(
        Box<dyn NodeComponent + Send + Sync + 'static>,
        Input,
        // number of inlets
        usize,
        // number of outlets
        usize,
    )>,
    pub(crate) connections: Vec<((usize, usize), (usize, usize))>,
}

#[derive(Component, Clone)]
pub(crate) struct Input {
    pub input: fn(ButtonInput<KeyCode>) -> bool,
}

impl Default for Input {
    fn default() -> Input {
        Input { input: |_| false }
    }
}

impl Patch {
    pub fn default() -> Patch {
        Patch {
            nodes: vec![],
            connections: vec![],
        }
    }

    pub fn create_node<'a, const IN: usize, const OUT: usize>(
        &'a mut self,
        node: impl NodeComponent + Send + Sync + 'static + crate::nodes::Node<IN, OUT>,
    ) -> NodeCommands<'a, IN, OUT> {
        self.nodes.push((Box::new(node), default(), IN, OUT));

        NodeCommands {
            node_ref: NodeRef(self.nodes.len() - 1),
            patch: self,
        }
    }

    pub fn connect<
        const I: usize,
        const O: usize,
        const IN1: usize,
        const OUT1: usize,
        const IN2: usize,
        const OUT2: usize,
    >(
        &mut self,
        outlet: Outlet<O, IN2, OUT2>,
        inlet: Inlet<I, IN1, OUT1>,
    ) {
        self.connections
            .push(((outlet.node.0, O), (inlet.node.0, I)));
    }

    pub fn bind_input<const IN: usize, const OUT: usize>(
        &mut self,
        node: NodeRef<IN, OUT>,
        input: fn(ButtonInput<KeyCode>) -> bool,
    ) {
        self.nodes[node.0].1 = Input { input };
    }
}

pub struct NodeCommands<'a, const IN: usize, const OUT: usize> {
    node_ref: NodeRef<IN, OUT>,
    patch: &'a mut Patch,
}

impl<'a, const IN: usize, const OUT: usize> NodeCommands<'a, IN, OUT> {
    pub fn id(&self) -> NodeRef<IN, OUT> {
        self.node_ref
    }

    // pub fn connect_outlet_to(&mut self, outlet: usize, other: (NodeRef, usize)) -> &mut Self {
    //     self.patch.connect((self.node_ref, outlet), other);
    //     self
    // }

    // pub fn connect_inlet_to(&mut self, inlet: usize, other: (NodeRef, usize)) -> &mut Self {
    //     self.patch.connect(other, (self.node_ref, inlet));
    //     self
    // }

    pub fn with_input(&mut self, input: fn(ButtonInput<KeyCode>) -> bool) -> &mut Self {
        self.patch.bind_input(self.node_ref, input);
        self
    }
}

#[macro_export]
macro_rules! keys_internal {
    ($keys1:ident, $last:ident) => {
        $keys1.pressed(KeyCode::$last)
    };
    ($keys1:ident, $head:ident $($rest:ident)*) => {
        $keys1.pressed(KeyCode::$head) && keys_internal!($keys1, $($rest)*)
    };
}

#[macro_export]
macro_rules! keys {
    ($first:ident) => {
        |keys1| keys1.pressed(KeyCode::$first)
    };
    ($first:ident, $($rest:ident),*) => {
        |keys1| keys1.pressed(KeyCode::$first) && keys_internal!(keys1, $($rest)*)
    };
}

#[macro_export]
macro_rules! keys_once_internal {
    ($keys1:ident, $last:ident) => {
        $keys1.just_pressed(KeyCode::$last)
    };
    ($keys1:ident, $head:ident $($rest:ident)*) => {
        $keys1.pressed(KeyCode::$head) && keys_once_internal!($keys1, $($rest)*)
    };
}

#[macro_export]
macro_rules! keys_once {
    ($first:ident) => {
        |keys1| keys1.just_pressed(KeyCode::$first)
    };
    ($first:ident, $($rest:ident),*) => {
        |keys1| keys1.pressed(KeyCode::$first) && keys_once_internal!(keys1, $($rest)*)
    };
}

#[derive(Clone, Copy)]
pub struct NodeRef<const IN: usize, const OUT: usize>(pub(crate) usize);

impl<const IN: usize, const OUT: usize> NodeRef<IN, OUT> {
    pub fn inlet<const I: usize>(&self) -> Inlet<I, IN, OUT> {
        const { assert!(I < IN, "This inlet doesn't exist!") }

        Inlet { node: *self }
    }

    pub fn outlet<const O: usize>(&self) -> Outlet<O, IN, OUT> {
        const { assert!(O < OUT, "This outlet doesn't exist!") }

        Outlet { node: *self }
    }
}

#[macro_export]
macro_rules! inlet {
    ($node:ident, $i:expr) => {
        $node.inlet::<$i>()
    };
}

#[macro_export]
macro_rules! outlet {
    ($node:ident, $i:expr) => {
        $node.outlet::<$i>()
    };
}

pub struct Inlet<const I: usize, const IN: usize, const OUT: usize> {
    node: NodeRef<IN, OUT>,
}

pub struct Outlet<const O: usize, const IN: usize, const OUT: usize> {
    node: NodeRef<IN, OUT>,
}

pub struct PatchPlugin;

impl Plugin for PatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(load_patch);
        app.add_systems(Last, despawn_old);
    }
}

#[derive(Component, Default)]
pub(crate) struct PatchEntity;

#[derive(Event)]
struct LoadPatch(pub Patch);

#[derive(Component)]
struct QueueDespawn;

fn load_patch(
    trigger: On<LoadPatch>,
    old_entities: Query<Entity, With<PatchEntity>>,
    mut commands: Commands,
) {
    info!("reloading patch!");

    for entity in old_entities {
        commands.entity(entity).insert(QueueDespawn);
    }

    let mut hash_in = HashMap::new();
    let mut hash_out = HashMap::new();

    for (i, (node, input, ins, outs)) in trigger.0.nodes.iter().enumerate() {
        info!("spawning node {i}");

        let node = node
            .spawn_component(&mut commands)
            .insert((PatchEntity, input.clone()))
            .id();

        for j in 0..*ins {
            let inlet = commands
                .spawn(crate::nodes::connections::InletOf(node))
                .id();

            hash_in.insert((i, j), (inlet, Connections::default()));
        }

        for j in 0..*outs {
            let outlet = commands
                .spawn(crate::nodes::connections::OutletOf(node))
                .id();

            hash_out.insert((i, j), (outlet, Connections::default()));
        }
    }

    for ((node_1, outlet), (node_2, inlet)) in &trigger.0.connections {
        let outlet = hash_out.get_mut(&(*node_1, *outlet)).unwrap();
        let inlet = hash_in.get_mut(&(*node_2, *inlet)).unwrap();

        outlet.1.0.push(inlet.0);
        inlet.1.0.push(outlet.0);
    }

    for (inlet, connections) in hash_in.values() {
        commands.entity(*inlet).insert(connections.clone());
    }

    for (outlet, connections) in hash_out.values() {
        commands.entity(*outlet).insert(connections.clone());
    }
}

fn despawn_old(entities: Query<Entity, With<QueueDespawn>>, mut commands: Commands) {
    for entity in entities {
        commands
            .entity(entity)
            .queue_silenced(|c: EntityWorldMut<'_>| c.despawn());
    }
}

pub trait AddPatch {
    fn add_patch(&mut self, patch: impl Fn() -> Patch + Send + Sync + 'static);
}

impl AddPatch for App {
    fn add_patch(&mut self, patch: impl Fn() -> Patch + Send + Sync + 'static) {
        self.add_systems(
            Update,
            move |keys: Res<ButtonInput<KeyCode>>, mut commands: Commands| {
                if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::KeyR) {
                    commands.trigger(LoadPatch(patch()));
                }
            },
        );
    }
}
