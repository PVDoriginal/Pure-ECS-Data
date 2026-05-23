use super::*;

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
pub struct Print {
    print_name: String,
}

impl Node<1, 0, 0, 0> for Print {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; 0] {
        println!("{}", inputs[0]);
        []
    }
}

impl NodeComponent for Print {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        let mut comp = self.clone();

        if let Some(Data::String(s)) = data.first() {
            comp.print_name = s.clone();
        }

        commands.spawn(comp)
    }

    fn internal_data(&self) -> Vec<Data> {
        vec![Data::String(self.clone().print_name)]
    }
}
