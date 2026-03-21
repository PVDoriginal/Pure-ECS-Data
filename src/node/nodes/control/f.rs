use super::*;

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

impl NodeComponent for F {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}
