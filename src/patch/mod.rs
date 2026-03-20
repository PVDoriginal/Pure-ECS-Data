use bevy::prelude::*;

use crate::{
    node::{
        data::Data,
        node_component::{Internal, NodeComponent},
    },
    patch::{
        inputs::Input,
        loading::{LoadPatch, PatchLoadingPlugin},
    },
};

pub mod inputs;
pub mod loading;
pub mod macros;

pub struct Patch {
    pub(crate) nodes: Vec<PatchNode>,
    pub(crate) connections: Vec<((usize, usize), (usize, usize))>,
}

struct PatchNode {
    component: Box<dyn NodeComponent + Send + Sync + 'static>,
    input: Option<Input>,
    internal_data: Vec<Data>,
    inlet_data: Vec<Data>,
    inlets: usize,
    outlets: usize,
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
        node: impl NodeComponent + Send + Sync + 'static + crate::node::Node<IN, OUT>,
    ) -> NodeCommands<'a, IN, OUT> {
        let internal_data = node.internal_data().clone();

        self.nodes.push(PatchNode {
            component: Box::new(node),
            input: None,
            internal_data,
            inlet_data: vec![],
            inlets: IN,
            outlets: OUT,
        });

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
        self.nodes[node.0].input = Some(Input { input });
    }

    pub fn bind_data<const IN: usize, const OUT: usize, const N: usize>(
        &mut self,
        node: NodeRef<IN, OUT>,
        data: [Data; N],
    ) {
        const {
            assert!(
                N <= IN,
                "Node data should be less or equal to the number of inlets of the Node"
            );
        }

        self.nodes[node.0].inlet_data = data.to_vec();
    }

    pub fn bind_internal<const IN: usize, const OUT: usize>(
        &mut self,
        node: NodeRef<IN, OUT>,
        internal: Vec<Data>,
    ) {
        self.nodes[node.0].internal_data = internal;
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

    pub fn with_data<const N: usize>(&mut self, data: [Data; N]) -> &mut Self {
        self.patch.bind_data(self.node_ref, data);
        self
    }

    pub fn containing(&mut self, data: Vec<Data>) -> &mut Self {
        self.patch.bind_internal(self.node_ref, data);
        self
    }
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
        app.add_plugins(PatchLoadingPlugin);
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
