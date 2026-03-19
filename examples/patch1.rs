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

    let print = patch.create_node(Print).id();

    patch.connect(outlet!(bang, 0), inlet!(bang2, 0));
    patch.connect(outlet!(bang2, 0), inlet!(bang, 0));

    patch
}
