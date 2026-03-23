mod print;
pub use print::*;

mod bang;
pub use bang::*;

mod f;
pub use f::*;

mod sum;
pub use sum::*;

mod number;
pub use number::*;

mod trigger;
pub use trigger::*;

mod msg;
pub use msg::*;

use super::*;

pub(crate) struct ControlNodesPlugin;

impl Plugin for ControlNodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_node::<Print>()
            .add_node::<Bang>()
            .add_node::<Number>()
            .add_node::<F>()
            .add_node::<Msg>();

        seq!(N in 0..=10 {
            app.add_node::<Sum<N>>()
                .add_node::<Trigger<N>>();
        });
    }
}
