use crate::nodes::NodeComponent;
use bevy::prelude::*;
pub struct Patch {
    pub(crate) nodes: Vec<(Box<dyn NodeComponent + Send + Sync + 'static>, Input)>,
    pub(crate) connections: Vec<((NodeRef, usize), (NodeRef, usize))>,
}

#[derive(Component, Clone)]
pub(crate) struct Input {
    pub input: fn(ButtonInput<KeyCode>) -> bool,
    pub toggle: bool,
}

impl Default for Input {
    fn default() -> Input {
        Input {
            input: |_| false,
            toggle: false,
        }
    }
}

impl Patch {
    pub fn default() -> Patch {
        Patch {
            nodes: vec![],
            connections: vec![],
        }
    }

    pub fn create_node<'a>(
        &'a mut self,
        node: impl NodeComponent + Send + Sync + 'static,
    ) -> NodeCommands<'a> {
        self.nodes.push((Box::new(node), default()));

        NodeCommands {
            node_ref: NodeRef(self.nodes.len() - 1),
            patch: self,
        }
    }

    pub fn connect(&mut self, node_1: (NodeRef, usize), node_2: (NodeRef, usize)) {
        self.connections.push((node_1, node_2));
    }

    pub fn bind_input(
        &mut self,
        node: NodeRef,
        input: fn(ButtonInput<KeyCode>) -> bool,
        toggle: bool,
    ) {
        self.nodes[node.0].1 = Input { input, toggle };
    }
}

pub struct NodeCommands<'a> {
    node_ref: NodeRef,
    patch: &'a mut Patch,
}

impl<'a> NodeCommands<'a> {
    pub fn id(&self) -> NodeRef {
        self.node_ref
    }

    pub fn connect_to(&mut self, outlet: usize, other: (NodeRef, usize)) -> &mut Self {
        self.patch.connect((self.node_ref, outlet), other);
        self
    }

    pub fn with_input(
        &mut self,
        input: fn(ButtonInput<KeyCode>) -> bool,
        toggle: bool,
    ) -> &mut Self {
        self.patch.bind_input(self.node_ref, input, toggle);
        self
    }
}

#[derive(Clone, Copy)]
pub struct NodeRef(pub(crate) usize);

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
