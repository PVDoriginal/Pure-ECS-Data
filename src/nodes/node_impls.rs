use super::Node;
use bevy::prelude::*;

use crate::nodes::data::Data;

#[derive(Component, Default, Clone)]
pub struct Print(pub Data);

impl Node<1, 0> for Print {
    fn process(&self, inputs: [Data; 1]) -> [Data; 0] {
        println!("{:?}", inputs[0]);
        []
    }
}

#[derive(Component, Default, Clone)]
pub struct Bang;

impl Node<0, 1> for Bang {
    fn process(&self, _: [Data; 0]) -> [Data; 1] {
        [Data::Bang]
    }
}
