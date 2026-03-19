use bevy::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(patch1);
    app.run();
}

fn patch1() -> Patch {
    let mut patch = Patch::default();

    let print_hello = patch.create_node(Print("hello world".into())).id();

    let bang = patch.create_node(Bang).with_input(keys_once!(Space)).id();

    patch.connect(outlet!(bang, 0), inlet!(print_hello, 0));

    patch
}
