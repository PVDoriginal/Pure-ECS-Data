pub mod print;
pub use print::*;

pub mod bang;
pub use bang::*;

pub mod f;
pub use f::*;

pub mod sum;
pub use sum::*;

pub mod number;
pub use number::*;

pub mod trigger;
pub use trigger::*;

pub mod message;
pub use message::*;

pub use super::*;

pub(crate) struct ControlNodesPlugin;

impl Plugin for ControlNodesPlugin {
    fn build(&self, app: &mut App) {
        app.add_node::<Print>()
            .add_node::<Bang>()
            .add_node::<Number>()
            .add_node::<F>()
            .add_node::<message::Message>();

        seq!(N in 0..=10 {
            app.add_node::<Sum<N>>();
        });

        seq!(N in 0..=10 {
            app.add_node::<Trigger<N>>();
        });
    }
}
