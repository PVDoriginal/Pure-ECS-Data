use super::Node;
use bevy::prelude::*;

use crate::node::data::{Data, Num};

/// Prints the data received in its inlet.
///
/// `In`: 1
/// `Out`: 0
///
/// ### Constructor
///
/// ```Rs
/// node = Print;
/// ```
///
/// ### Example
///
/// ```Rs
/// print = Print ["Hello world!"] |# Space;
/// ```
///
/// Outputs "Hello World" each time you press `Space`.
#[derive(Component, Default, Clone, Reflect)]
pub struct Print;

impl Node<1, 0> for Print {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 0] {
        println!("{}", inputs[0]);
        []
    }
}

/// Outputs a Bang.
///
/// Most commonly, this is used by binding input to it.
///
/// `In`: 1
/// `Out`: 1
///
/// ### Constructor
/// ```Rs
/// node = Bang;
/// ```
///
/// ### Example
/// ```Rs
/// bang = Bang |# Space;
/// print = Print;
/// bang -> print;
/// ```
/// Prints "Bang" each time you press `Space`.
#[derive(Component, Default, Clone, Reflect)]
pub struct Bang;

impl Node<1, 1> for Bang {
    fn process(&mut self, _: [Data; 1]) -> [Data; 1] {
        [Data::Bang]
    }
}

/// Outputs and stores the data from its second inlet.
///
/// If the second inlet is empty, will simply output the data from the first inlet.
///
/// `In`: 2
/// `Out`: 1
///
/// ### Constructor
/// ```Rs
/// node = F;
/// ```
///
/// ### Example
/// ```Rs
/// bang = Bang |# Space;
/// print = Print;
/// add = Add<2> [1];
/// f = F;
/// bang -> f;
/// f -> add;
/// add -> f[1], print;
/// ```
/// Increments a number by 1 and prints it each time you press `Space`.
#[derive(Component, Default, Clone, Reflect)]
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

/// Outputs the sum of the data from all N inlets.
///
/// `In`: N
/// `Out`: 1
///
/// ### Constructor
/// ```Rs
/// node = Sum<N>;
/// ```
///
/// ### Example
/// ```Rs
/// a = Number { 5 };
/// sum = Sum<2> [2];
///
/// print = Print;
/// bang = Bang |# Space;
///
/// bang -> a;
/// a -> sum;
/// sum -> print;
/// ```
/// Prints "7" each time you press `Space`.
#[derive(Component, Default, Clone, Reflect)]
pub struct Sum<const N: usize>;

impl<const N: usize> Node<N, 1> for Sum<N> {
    fn process(&mut self, inputs: [Data; N]) -> [Data; 1] {
        let mut res = Data::None;

        for input in inputs {
            res += input;
        }

        [res]
    }
}

/// Contains and outputs a number.
///
/// When receiving another number, it will store that number instead.
///
/// Can be initialized with a number already inside.  
///
/// `In`: 1
/// `Out`: 1
#[derive(Component, Clone, Reflect)]
pub struct Number(pub Num);

impl Default for Number {
    fn default() -> Number {
        Number(Num::Int(0))
    }
}

impl From<Data> for Number {
    fn from(value: Data) -> Self {
        Number(value.into())
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

#[derive(Component, Clone, Reflect)]
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

impl<const N: usize> From<[Data; N]> for Trigger<N> {
    fn from(value: [Data; N]) -> Self {
        Trigger(value)
    }
}
impl From<Data> for Trigger<1> {
    fn from(value: Data) -> Self {
        Trigger([value])
    }
}

impl<const N: usize> Default for Trigger<N> {
    fn default() -> Self {
        Trigger([const { Data::Bang }; N])
    }
}
