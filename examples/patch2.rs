use bevy::prelude::*;
use pure_ecs_data::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PureDataPlugin));
    app.add_patch(patch);
    app.run();
}

fn patch() -> Patch {
    let mut patch = Patch::default();

    let bang = patch.create_node(Bang).with_input(keys_once!(Space)).id();

    let num_1 = patch.create_node(Number::from(3)).id();
    let num_2 = patch.create_node(Number::from(5)).id();

    let add = patch.create_node(Add::<2>).id();

    let print = patch.create_node(Print).id();

    let trigger = patch
        .create_node(Trigger::<2>([Data::Bang, Data::Bang]))
        .id();

    patch.connect(outlet!(bang, 0), inlet!(trigger, 0));

    patch.connect(outlet!(trigger, 0), inlet!(num_1, 0));
    patch.connect(outlet!(trigger, 1), inlet!(num_2, 0));

    patch.connect(outlet!(num_1, 0), inlet!(add, 0));
    patch.connect(outlet!(num_2, 0), inlet!(add, 1));

    patch.connect(outlet!(add, 0), inlet!(print, 0));

    patch
}
