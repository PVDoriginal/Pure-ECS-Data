use super::*;

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

impl<const N: usize> Node<N, 0, 1, 0> for Sum<N> {
    fn process(&mut self, inputs: [Data; N]) -> [Data; 1] {
        let mut res = Data::None;

        for input in inputs {
            res += input;
        }

        [res]
    }
}

impl<const N: usize> NodeComponent for Sum<N> {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}
