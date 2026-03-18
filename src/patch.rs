use std::arch::x86_64;

use crate::nodes::NodeComponent;
use bevy::prelude::*;
pub struct Patch {
    pub(crate) nodes: Vec<(Box<dyn NodeComponent + Send + Sync + 'static>, Input)>,
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
        self.nodes.push((Box::new(node), default()));

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
        $keys1.just_pressed(KeyCode::$last)
    };
    ($keys1:ident, $head:ident $($rest:ident)*) => {
        $keys1.pressed(KeyCode::$head) && keys_internal!($keys1, $($rest)*)
    };
}
#[macro_export]
macro_rules! keys {
    ($first:ident) => {
        |keys1| keys1.just_pressed(KeyCode::$first)
    };
    ($first:ident, $($rest:ident),*) => {
        |keys1| keys1.pressed(KeyCode::$first) && keys_internal!(keys1, $($rest)*)
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
    }
}

#[derive(Component)]
struct PatchEntity;

#[derive(Event)]
struct LoadPatch(pub Patch);

fn load_patch(
    trigger: On<LoadPatch>,
    old_entities: Query<Entity, With<PatchEntity>>,
    mut commands: Commands,
) {
    info!("loading new patch!");

    for entity in old_entities {
        commands.entity(entity).despawn();
    }

    for (node, input) in &trigger.0.nodes {
        node.spawn_component(&mut commands).insert(input.clone());
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
