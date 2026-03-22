use super::*;

#[derive(Component, Clone, Reflect)]
pub struct Trigger<const N: usize>(pub [Data; N]);

impl<const N: usize> Node<1, 0, N, 0> for Trigger<N> {
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

impl<const N: usize> NodeComponent for Trigger<N> {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        let mut comp = self.clone();

        for (i, data) in data.iter().enumerate() {
            comp.0[i] = data.clone();
        }

        commands.spawn(comp)
    }

    fn internal_data(&self) -> Vec<Data> {
        self.0.to_vec()
    }
}
