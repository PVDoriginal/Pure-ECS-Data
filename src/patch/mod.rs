use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    node::{data::Data, node_component::NodeComponent},
    patch::{
        inputs::Input,
        loading::{LivePatch, PatchLoadingPlugin},
    },
    prelude::Print,
};

use crate::node::Node;

pub mod inputs;
pub mod loading;
pub mod macros;

#[derive(Default, Clone)]
pub struct Patch {
    pub(crate) nodes: HashMap<String, (PatchNode, Ingoing, Outgoing)>,
}

pub(crate) struct PatchNode {
    component: Box<dyn NodeComponent + Send + Sync + 'static>,
    component_id: String,
    input: Option<Input>,
    internal_data: Vec<Data>,
    arg_order: Vec<usize>,
}

impl Clone for PatchNode {
    fn clone(&self) -> Self {
        Self {
            component: Box::new(Print),
            component_id: self.component_id.clone(),
            input: self.input.clone(),
            internal_data: self.internal_data.clone(),
            arg_order: vec![],
        }
    }
}

#[derive(PartialEq, Clone)]
pub(crate) struct Ingoing(Vec<Data>);

#[derive(PartialEq, Clone)]
pub(crate) struct Outgoing(Vec<Vec<(String, usize)>>);

impl Patch {
    pub fn create_node<'a, const IN: usize, const OUT: usize>(
        &'a mut self,
        name: String,
        node: impl NodeComponent + Send + Sync + 'static + crate::node::Node<IN, OUT>,
    ) -> NodeCommands<'a, IN, OUT> {
        let component_id: String = node.get_type().into();
        let internal_data = node.internal_data().clone();
        let arg_order = node.argument_order().to_vec();

        self.nodes.insert(
            name.clone(),
            (
                PatchNode {
                    component: Box::new(node),
                    component_id,
                    input: None,
                    internal_data,
                    arg_order,
                },
                Ingoing([const { Data::None }; IN].to_vec()),
                Outgoing([const { vec![] }; OUT].to_vec()),
            ),
        );

        NodeCommands {
            node_ref: NodeRef(name),
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
        self.nodes.get_mut(&outlet.node.0).unwrap().2.0[O].push((inlet.node.0.clone(), I));
    }

    pub fn bind_input<const IN: usize, const OUT: usize>(
        &mut self,
        node: NodeRef<IN, OUT>,
        input: Input,
    ) {
        self.nodes.get_mut(&node.0).unwrap().0.input = Some(input);
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

        let node = self.nodes.get_mut(&node.0).unwrap();
        for (i, data) in data.iter().enumerate() {
            node.1.0[node.0.arg_order[i]] = data.clone();
        }
    }

    pub fn bind_data_inlet<const I: usize, const IN: usize, const OUT: usize>(
        &mut self,
        node: Inlet<I, IN, OUT>,
        data: Data,
    ) {
        self.nodes.get_mut(&node.node.0).unwrap().1.0[I] = data;
    }

    pub fn bind_internal<const IN: usize, const OUT: usize>(
        &mut self,
        node: &NodeRef<IN, OUT>,
        internal: Vec<Data>,
    ) {
        self.nodes.get_mut(&node.0).unwrap().0.internal_data = internal;
    }
}

pub struct NodeCommands<'a, const IN: usize, const OUT: usize> {
    node_ref: NodeRef<IN, OUT>,
    patch: &'a mut Patch,
}

impl<'a, const IN: usize, const OUT: usize> NodeCommands<'a, IN, OUT> {
    pub fn id(&self) -> NodeRef<IN, OUT> {
        self.node_ref.clone()
    }

    pub fn with_input(&mut self, input: Input) -> &mut Self {
        self.patch.bind_input(self.node_ref.clone(), input);
        self
    }

    pub fn with_input_maybe(&mut self, input: Option<Input>) -> &mut Self {
        let Some(input) = input else { return self };

        self.patch.bind_input(self.node_ref.clone(), input);
        self
    }

    pub fn with_data<const N: usize>(&mut self, data: [Data; N]) -> &mut Self {
        self.patch.bind_data(self.node_ref.clone(), data);
        self
    }

    pub fn containing(&mut self, data: Vec<Data>) -> &mut Self {
        self.patch.bind_internal(&self.node_ref, data);
        self
    }
}

#[derive(Clone)]
pub struct NodeRef<const IN: usize, const OUT: usize>(pub(crate) String);

impl<const IN: usize, const OUT: usize> NodeRef<IN, OUT> {
    pub fn inlet<const I: usize>(&self) -> Inlet<I, IN, OUT> {
        const { assert!(I < IN, "This inlet doesn't exist!") }

        Inlet { node: self.clone() }
    }

    pub fn outlet<const O: usize>(&self) -> Outlet<O, IN, OUT> {
        const { assert!(O < OUT, "This outlet doesn't exist!") }

        Outlet { node: self.clone() }
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

#[derive(Clone)]
pub struct Inlet<const I: usize, const IN: usize, const OUT: usize> {
    node: NodeRef<IN, OUT>,
}

#[derive(Clone)]
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
        self.add_systems(Update, move |mut commands: Commands| {
            let patch = patch();
            commands.insert_resource(LivePatch(patch));
        });
    }
}
