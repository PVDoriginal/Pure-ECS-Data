use super::*;

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

impl Node<1, 0, 1, 0> for Number {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 1] {
        if let Data::Num(n) = inputs[0].clone() {
            self.0 = n;
        }

        [self.0.clone().into()]
    }
}

impl NodeComponent for Number {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        let mut comp = self.clone();

        if let Some(Data::Num(n)) = data.first() {
            comp.0 = n.clone();
        }

        commands.spawn(comp)
    }

    fn internal_data(&self) -> Vec<Data> {
        vec![self.clone().0.into()]
    }
}
