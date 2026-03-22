use std::ops::Range;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_seedling::{
    edge::{Connect, Disconnect},
    prelude::MainBus,
};

use crate::{
    node::connections::{
        CarriedData, Connections, InletOf, InletType, Inlets, OtherInlets, OutletOf, Outlets,
    },
    patch::{Patch, PatchNode},
    prelude::Input,
};

pub struct PatchLoadingPlugin;

impl Plugin for PatchLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedPatch>();
        app.init_resource::<PatchMap>();
        app.init_resource::<LivePatch>();
        app.init_resource::<UpdateInlets>();
        app.init_resource::<UpdateOutlets>();
        app.init_resource::<UpdateSignals>();
        app.add_systems(PreUpdate, load_patch);
        app.add_systems(
            PreUpdate,
            (
                update_outlets.after(load_patch),
                update_inlets.after(load_patch),
                update_signals.after(load_patch),
            ),
        );
        app.add_systems(Last, despawn_old);
    }
}

#[derive(Component, Default)]
pub(crate) struct PatchEntity;

#[derive(Resource, Default)]
pub(crate) struct LoadedPatch(pub Patch);

#[derive(Resource, Default)]
pub(crate) struct LivePatch(pub Patch);

#[derive(Resource, Default)]
pub(crate) struct PatchMap(
    pub  HashMap<
        String,
        (
            Entity,
            Vec<Entity>,
            Vec<Entity>,
            Vec<(Entity, usize, usize)>,
        ),
    >,
);

#[derive(Component)]
pub(crate) struct QueueDespawn;

#[derive(Resource, Default)]
pub(crate) struct UpdateInlets(pub Vec<Entity>);

#[derive(Resource, Default)]
pub(crate) struct UpdateOutlets(pub Vec<Entity>);

#[derive(Resource, Default)]
pub(crate) struct UpdateSignals(pub Vec<Entity>);

#[derive(Component)]
pub(crate) struct NodeId(pub String);

fn load_patch(
    mut loaded_patch: ResMut<LoadedPatch>,
    live_patch: Res<LivePatch>,
    mut commands: Commands,

    mut map: ResMut<PatchMap>,

    mut update_inlets: ResMut<UpdateInlets>,
    mut update_outlets: ResMut<UpdateOutlets>,
    mut update_signals: ResMut<UpdateSignals>,
) {
    update_inlets.0 = vec![];
    update_outlets.0 = vec![];
    update_signals.0 = vec![];

    for (node_name, (live_node, live_ingoing, live_outgoing, live_signals)) in &live_patch.0.nodes {
        if let Some((loaded_node, loaded_ingoing, loaded_outgoing, loaded_signals)) =
            loaded_patch.0.nodes.get(node_name)
        {
            if loaded_node.component_id != live_node.component_id
                || loaded_node.internal_data != live_node.internal_data
            {
                despawn_loaded_node(node_name.clone(), &mut map.0, &mut commands);
                let entity = spawn_node(
                    node_name.clone(),
                    live_node,
                    live_ingoing.0.len(),
                    live_outgoing.0.len(),
                    &mut map.0,
                    &mut commands,
                );
                update_inlets.0.push(entity);
                update_outlets.0.push(entity);
                update_signals.0.push(entity);

                continue;
            }

            let mapped_node = map.0.get_mut(node_name).unwrap();

            if loaded_node.input != live_node.input {
                if let Some(input) = live_node.input.clone() {
                    commands.entity(mapped_node.0).insert(input);
                } else {
                    commands.entity(mapped_node.0).remove::<Input>();
                }
            }

            if live_ingoing != loaded_ingoing {
                if live_ingoing.0.len() > loaded_ingoing.0.len() {
                    add_inlets(
                        loaded_ingoing.0.len()..live_ingoing.0.len(),
                        mapped_node,
                        &mut commands,
                    );
                } else if live_ingoing.0.len() < loaded_ingoing.0.len() {
                    remove_inlets(
                        live_ingoing.0.len()..loaded_ingoing.0.len(),
                        mapped_node,
                        &mut commands,
                    )
                }

                update_inlets.0.push(mapped_node.0);
            }

            if live_outgoing != loaded_outgoing {
                if live_outgoing.0.len() > loaded_outgoing.0.len() {
                    add_outlets(
                        loaded_outgoing.0.len()..live_outgoing.0.len(),
                        mapped_node,
                        &mut commands,
                    );
                } else if live_outgoing.0.len() < loaded_outgoing.0.len() {
                    remove_outlets(
                        live_outgoing.0.len()..loaded_outgoing.0.len(),
                        mapped_node,
                        &mut commands,
                    )
                }

                update_outlets.0.push(mapped_node.0);
            }

            if live_signals != loaded_signals {
                update_signals.0.push(mapped_node.0);
            }
        } else {
            let entity = spawn_node(
                node_name.clone(),
                live_node,
                live_ingoing.0.len(),
                live_outgoing.0.len(),
                &mut map.0,
                &mut commands,
            );
            update_inlets.0.push(entity);
            update_outlets.0.push(entity);
            update_signals.0.push(entity);
        }
    }

    for (node_name, _) in &loaded_patch.0.nodes {
        if live_patch.0.nodes.get(node_name).is_some() {
            continue;
        };

        despawn_loaded_node(node_name.clone(), &mut map.0, &mut commands);
    }

    loaded_patch.0 = live_patch.0.clone();
}

fn despawn_loaded_node(
    name: String,
    map: &mut HashMap<
        String,
        (
            Entity,
            Vec<Entity>,
            Vec<Entity>,
            Vec<(Entity, usize, usize)>,
        ),
    >,
    commands: &mut Commands,
) {
    info!("Despawning {name}");
    let (entity, inlets, outlets, _) = map.get(&name).unwrap();

    commands.entity(*entity).insert(QueueDespawn);

    for inlet in inlets {
        commands.entity(*inlet).insert(QueueDespawn);
    }

    for outlet in outlets {
        commands.entity(*outlet).insert(QueueDespawn);
    }

    map.remove(&name);
}

fn spawn_node(
    name: String,
    node: &PatchNode,
    n_ingoing: usize,
    n_outgoing: usize,
    map: &mut HashMap<
        String,
        (
            Entity,
            Vec<Entity>,
            Vec<Entity>,
            Vec<(Entity, usize, usize)>,
        ),
    >,
    commands: &mut Commands,
) -> Entity {
    info!("Spawning {name}");
    let mut node_entity = node
        .component
        .spawn_component(node.internal_data.clone(), commands);

    if let Some(input) = node.input.clone() {
        node_entity.insert(input);
    }

    node_entity.insert(NodeId(name.clone()));

    let node_entity = node_entity.id();

    let inlet_entities: Vec<_> = (0..n_ingoing)
        .map(|i| {
            commands
                .spawn(InletOf {
                    entity: node_entity,
                    inlet_type: match i {
                        0 => InletType::Hot,
                        _ => InletType::Cold,
                    },
                })
                .id()
        })
        .collect();

    let outlet_entities: Vec<_> = (0..n_outgoing)
        .map(|_| commands.spawn(OutletOf(node_entity)).id())
        .collect();

    map.insert(name, (node_entity, inlet_entities, outlet_entities, vec![]));

    node_entity
}

fn add_inlets(
    range: Range<usize>,
    node_map: &mut (
        Entity,
        Vec<Entity>,
        Vec<Entity>,
        Vec<(Entity, usize, usize)>,
    ),
    commands: &mut Commands,
) {
    for i in range {
        let inlet = commands
            .spawn(InletOf {
                entity: node_map.0.clone(),
                inlet_type: match i {
                    0 => InletType::Hot,
                    _ => InletType::Cold,
                },
            })
            .id();

        node_map.1.push(inlet);
    }
}

fn remove_inlets(
    range: Range<usize>,
    node_map: &mut (
        Entity,
        Vec<Entity>,
        Vec<Entity>,
        Vec<(Entity, usize, usize)>,
    ),
    commands: &mut Commands,
) {
    for _ in range {
        let inlet = node_map.1.pop().unwrap();
        commands.entity(inlet).remove::<InletOf>();
        commands.entity(inlet).insert(QueueDespawn);
    }
}

fn add_outlets(
    range: Range<usize>,
    node_map: &mut (
        Entity,
        Vec<Entity>,
        Vec<Entity>,
        Vec<(Entity, usize, usize)>,
    ),
    commands: &mut Commands,
) {
    for _ in range {
        let outlet = commands.spawn(OutletOf(node_map.0.clone())).id();
        node_map.2.push(outlet);
    }
}

fn remove_outlets(
    range: Range<usize>,
    node_map: &mut (
        Entity,
        Vec<Entity>,
        Vec<Entity>,
        Vec<(Entity, usize, usize)>,
    ),
    commands: &mut Commands,
) {
    for _ in range {
        let outlet = node_map.2.pop().unwrap();
        commands.entity(outlet).remove::<OutletOf>();
        commands.entity(outlet).insert(QueueDespawn);
    }
}

fn update_inlets(
    update_inlets: Res<UpdateInlets>,
    nodes: Query<&NodeId, With<Inlets>>,
    mut inlets: Query<(&mut CarriedData, &mut OtherInlets), With<InletOf>>,
    patch: Res<LoadedPatch>,
    map: Res<PatchMap>,
) {
    for update_inlet in &update_inlets.0 {
        let Ok(node_id) = nodes.get(*update_inlet) else {
            continue;
        };

        let inlet_entities = &map.0.get(&node_id.0).unwrap().1;
        let inlets_data = &patch.0.nodes.get(&node_id.0).unwrap().1.0;

        for (i, data) in inlets_data.iter().enumerate() {
            let mut inlet_data = inlets.get_mut(inlet_entities[i]).unwrap();
            inlet_data.0.0.assign(data.clone());
            inlet_data.1.0 = inlet_entities.clone();
        }
    }
}

fn update_outlets(
    update_outlets: Res<UpdateOutlets>,
    nodes: Query<&NodeId, With<Outlets>>,
    mut outlets: Query<&mut Connections, With<OutletOf>>,
    patch: Res<LoadedPatch>,
    map: Res<PatchMap>,
) {
    for update_outlet in &update_outlets.0 {
        let Ok(node_id) = nodes.get(*update_outlet) else {
            continue;
        };

        let outlet_entities = &map.0.get(&node_id.0).unwrap().2;
        let outlet_connections: Vec<_> = patch
            .0
            .nodes
            .get(&node_id.0)
            .unwrap()
            .2
            .0
            .iter()
            .map(|connections| {
                Connections(
                    connections
                        .iter()
                        .map(|(name, i)| {
                            let mapped_node = map.0.get(name).unwrap();
                            mapped_node.1[*i]
                        })
                        .collect(),
                )
            })
            .collect();

        for (i, data) in outlet_connections.iter().enumerate() {
            let mut current_data = outlets.get_mut(outlet_entities[i]).unwrap();
            *current_data = data.clone();
        }
    }
}

fn update_signals(
    update_signals: Res<UpdateSignals>,
    nodes: Query<&NodeId>,
    patch: Res<LoadedPatch>,
    mut map: ResMut<PatchMap>,
    mut commands: Commands,
) {
    for update_signal in &update_signals.0 {
        let Ok(node_id) = nodes.get(*update_signal) else {
            continue;
        };

        for (other, i, j) in &map.0.get(&node_id.0).unwrap().3 {
            commands
                .entity(*update_signal)
                .disconnect_with(*other, &[(*i as u32, *j as u32)]);

            info!(
                "disconnecting {} {} from {} {}",
                *update_signal, i, *other, *j
            );
        }

        let connections: Vec<_> = patch
            .0
            .nodes
            .get(&node_id.0)
            .unwrap()
            .3
            .0
            .iter()
            .map(|connections| {
                connections
                    .iter()
                    .map(|(name, i)| {
                        let mapped_node = map.0.get(name).unwrap();
                        (mapped_node.0, i)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let map_list = &mut map.0.get_mut(&node_id.0).unwrap().3;
        *map_list = vec![];

        for (i, connections) in connections.iter().enumerate() {
            for (entity, j) in connections {
                commands
                    .entity(*update_signal)
                    .connect_with(*entity, &[(i as u32, **j as u32)]);

                info!("connecting {} {} to {} {}", *update_signal, i, *entity, **j);
                map_list.push((*entity, i, **j));
            }
        }
    }
}

fn despawn_old(entities: Query<Entity, With<QueueDespawn>>, mut commands: Commands) {
    for entity in entities {
        commands
            .entity(entity)
            .queue_silenced(|c: EntityWorldMut<'_>| c.despawn());
    }
}
