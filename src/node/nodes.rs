use super::Node;
use bevy::prelude::*;

use crate::node::data::{Data, Num};

#[derive(Component, Default, Clone)]
pub struct Print;

impl Node<1, 0> for Print {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 0] {
        println!("{}", inputs[0]);
        []
    }
}

#[derive(Component, Default, Clone)]
pub struct Bang;

impl Node<1, 1> for Bang {
    fn process(&mut self, _: [Data; 1]) -> [Data; 1] {
        [Data::Bang]
    }
}

#[derive(Component, Default, Clone)]
pub struct F(pub Option<Num>);

impl Node<2, 1> for F {
    fn process(&mut self, inputs: [Data; 2]) -> [Data; 1] {
        if matches!(inputs[1], Data::None)
            && let Data::Num(n) = &inputs[0]
        {
            return [n.clone().into()];
        }

        if let Data::Num(n) = &inputs[1] {
            self.0 = Some(n.clone());
        }

        [self.0.clone().unwrap_or_default().into()]
    }
}

#[derive(Component, Default, Clone)]
pub struct Add<const N: usize>;

impl<const N: usize> Node<N, 1> for Add<N> {
    fn process(&mut self, inputs: [Data; N]) -> [Data; 1] {
        let mut res = Data::None;

        for input in inputs {
            res += input;
        }

        [res]
    }
}

#[derive(Component, Clone)]
pub struct Number(pub Num);

impl Default for Number {
    fn default() -> Number {
        Number(Num::Int(0))
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number(Num::Int(value))
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number(Num::Float(value))
    }
}

impl Node<1, 1> for Number {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 1] {
        if let Data::Num(n) = inputs[0].clone() {
            self.0 = n;
        }

        [self.0.clone().into()]
    }
}

#[derive(Component, Clone)]
pub struct Trigger<const N: usize>(pub [Data; N]);

impl<const N: usize> Node<1, N> for Trigger<N> {
    fn process(&mut self, _: [Data; 1]) -> [Data; N] {
        self.0.clone()
    }
    fn outlet_order() -> [usize; N] {
        let mut order = [0; N];
        for i in 0..N {
            order[i] = N - i;
        }
        order
    }
}
