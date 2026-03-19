use super::Node;
use bevy::prelude::*;

use crate::node::data::Data;

#[derive(Component, Default, Clone)]
pub struct Print;

impl Node<1, 0> for Print {
    fn process(&self, inputs: [Data; 1]) -> [Data; 0] {
        println!("{:?}", inputs[0]);
        []
    }
}

#[derive(Component, Default, Clone)]
pub struct Bang;

impl Node<1, 1> for Bang {
    fn process(&self, _: [Data; 1]) -> [Data; 1] {
        [Data::Bang]
    }
}

#[derive(Component, Default, Clone)]
pub struct F;

impl Node<2, 1> for F {
    fn process(&self, inputs: [Data; 2]) -> [Data; 1] {
        match &inputs[1] {
            Data::None => [inputs[0].clone()],
            data => [data.clone()],
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct Add<const N: usize>;

impl<const N: usize> Node<N, 1> for Add<N> {
    fn process(&self, inputs: [Data; N]) -> [Data; 1] {
        let mut res = Data::None;

        for input in inputs {
            res += input;
        }

        [res]
    }
}
