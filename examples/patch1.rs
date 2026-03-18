use Pure_ECS_Data::{
    PureDataPlugin,
    nodes::{Bang, Print},
    patch::{AddPatch, Patch},
};
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(patch1);
    app.run();
}

fn patch1() -> Patch {
    let mut patch = Patch::default();

    let print_hello = patch.create_node(Print("hello world".into())).id();

    patch
        .create_node(Bang)
        .with_input(|keys| keys.pressed(KeyCode::Space), false)
        .connect_to(0, (print_hello, 0));

    patch
}
