use Pure_ECS_Data::{
    PureDataPlugin, inlet, keys, keys_internal,
    nodes::{Bang, Print},
    outlet,
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

    let bang = patch
        .create_node(Bang)
        .with_input(keys!(ControlLeft, KeyS), false)
        .id();

    patch.connect(inlet!(print_hello, 0), outlet!(bang, 10));

    patch
}
