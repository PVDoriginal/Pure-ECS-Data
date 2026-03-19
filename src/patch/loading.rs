use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    node::connections::{CarriedData, Connections, InletType, OtherInlets},
    patch::Patch,
};

pub struct PatchLoadingPlugin;

impl Plugin for PatchLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(load_patch);
        app.add_systems(Last, despawn_old);
    }
}

#[derive(Component, Default)]
pub(crate) struct PatchEntity;

#[derive(Event)]
pub(crate) struct LoadPatch(pub Patch);

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

    for (i, (node, input, data, internal_data, ins, outs)) in trigger.0.nodes.iter().enumerate() {
        let node = node
            .spawn_component(internal_data.clone(), &mut commands)
            .insert((PatchEntity, input.clone()))
            .id();

        info!("spawning node {i}: {node}");

        let mut all_inlets = vec![];

        for j in 0..*ins {
            let inlet = commands
                .spawn(crate::node::connections::InletOf {
                    entity: node,
                    inlet_type: match j {
                        0 => InletType::Hot,
                        _ => InletType::Cold,
                    },
                })
                .id();

            hash_in.insert((i, j), (inlet, Connections::default()));

            if (*ins - 1 - j) < data.len() {
                commands
                    .entity(inlet)
                    .insert(CarriedData(data[*ins - 1 - j].clone()));
            }

            all_inlets.push(inlet);
        }

        for inlet in &all_inlets {
            commands
                .entity(*inlet)
                .insert(OtherInlets(all_inlets.clone()));
        }

        for j in 0..*outs {
            let outlet = commands
                .spawn(crate::node::connections::OutletOf(node))
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
