use bevy::prelude::*;

use crate::{nodes::data::Data, patch::PatchEntity};

#[derive(Component)]
#[relationship(relationship_target = Inlets)]
#[require(CarriedData)]
pub(crate) struct InletOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = InletOf, linked_spawn)]
pub(crate) struct Inlets(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = Outlets)]
#[require(CarriedData)]
pub(crate) struct OutletOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = OutletOf, linked_spawn)]
pub(crate) struct Outlets(Vec<Entity>);

#[derive(Component, Default, Clone)]
#[require(PatchEntity)]
pub struct Connections(pub Vec<Entity>);

#[derive(Component, Clone, Default)]
pub struct CarriedData(pub Data);
