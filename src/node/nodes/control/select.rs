use super::*;

#[derive(Component, Clone, Reflect)]
pub struct Select<const N: usize> {
    pos_vals: [f32; N],
}

impl<const N: usize> Default for Select<N> {
    fn default() -> Self {
        // fast, safe array init for Copy types
        Self {
            pos_vals: [0.0f32; N],
        }
    }
}
impl<const N: usize> From<[Data; N]> for Select<N> {
    fn from(values: [Data; N]) -> Self {
        let mut result = Select::<N>::default();
        for i in 0..N - 1 {
            match &values[i] {
                Data::Num(nr) => result.pos_vals[i] = nr.clone().into(),
                _ => {}
            }
        }
        result
    }
}

impl<const N: usize> Node<1, 0, N, 0> for Select<N> {
    fn process(&mut self, inputs: [Data; 1]) -> [Data; N] {
        let mut res = [const { Data::None }; N];

        for i in 0..(self.pos_vals.len() - 1) {
            if self.pos_vals[i] == Into::<f32>::into(inputs[0].clone()) {
                res[i] = Data::Bang;
            }
            return res;
        }

        let pos: usize = N - 1;
        res[pos] = Data::Num(Num::Float(inputs[0].clone().into()));
        res
    }
}

impl<const N: usize> NodeComponent for Select<N> {
    fn spawn_component<'a>(
        &self,
        data: Vec<Data>,
        commands: &'a mut Commands,
    ) -> EntityCommands<'a> {
        if data.len() != N {
            panic!(
                "wrong amount of arguments for select! {} != {}",
                N,
                data.len()
            );
        }
        info!("trying");
        let arr: [Data; N] = data.into_iter().collect::<Vec<_>>().try_into().unwrap();
        info!("Data {}", N);
        let comp: Select<N> = arr.into();

        commands.spawn(comp)
    }
}
