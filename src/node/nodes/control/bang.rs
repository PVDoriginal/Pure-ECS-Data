use super::*;

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

impl NodeComponent for Bang {
    fn spawn_component<'a>(&self, _: Vec<Data>, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.clone())
    }
}
