use bevy::prelude::*;

use crate::{node::data::Data, patch::loading::PatchEntity};

#[derive(Component)]
#[relationship(relationship_target = Inlets)]
#[require(CarriedData, OtherInlets)]
pub(crate) struct InletOf {
    #[relationship]
    pub entity: Entity,
    pub inlet_type: InletType,
}

#[derive(Component, Default)]
#[relationship_target(relationship = InletOf, linked_spawn)]
pub(crate) struct Inlets(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = Outlets)]
#[require(Connections)]
pub(crate) struct OutletOf(pub Entity);

#[derive(Component, Default)]
#[relationship_target(relationship = OutletOf, linked_spawn)]
pub(crate) struct Outlets(Vec<Entity>);

#[derive(Component, Default, Clone)]
#[require(PatchEntity)]
pub struct Connections(pub Vec<Entity>);

#[derive(Component, Clone, Default)]
pub struct CarriedData(pub Data);

#[derive(Default)]
pub enum InletType {
    Hot,
    #[default]
    Cold,
}

#[derive(Component, Default, Clone)]
pub struct OtherInlets(pub Vec<Entity>);
